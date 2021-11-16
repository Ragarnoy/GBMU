use std::fmt::{Display, Formatter, write};
use std::str::FromStr;
use anyhow::anyhow;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map;
use nom::sequence::tuple;
use crate::dbg_interfaces::{CpuRegs, RegisterDebugOperations};

#[derive(Debug)]
enum Operator {
    Eq,
    NotEq,
    Sup,
    Inf,
    SupEq,
    InfEq,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Eq => write!(f, "=="),
            Operator::NotEq => write!(f, "!="),
            Operator::Sup => write!(f, ">"),
            Operator::Inf => write!(f, "<"),
            Operator::SupEq => write!(f, ">="),
            Operator::InfEq => write!(f, "<="),
        }
    }
}

#[derive(Debug)]
pub struct BreakpointNode {
    lhs: CpuRegs,
    op: Operator,
    rhs: u16,
}

impl Display for BreakpointNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let CpuRegs::PC = self.lhs {
            if let Operator::Eq = self.op {
                return write!(f, "0x{:04X}", self.rhs)
            }
        }

        write!(f, "{:?} {} 0x{:04X}", self.lhs, self.op, self.rhs)
    }
}

impl BreakpointNode {
    pub fn compute<T: RegisterDebugOperations>(&self, regs: &T) -> bool {
        match self.op {
            Operator::Eq => u16::from(regs.cpu_get(self.lhs).unwrap()) == self.rhs,
            Operator::NotEq => u16::from(regs.cpu_get(self.lhs).unwrap()) != self.rhs,
            Operator::Sup => u16::from(regs.cpu_get(self.lhs).unwrap()) > self.rhs,
            Operator::Inf => u16::from(regs.cpu_get(self.lhs).unwrap()) < self.rhs,
            Operator::SupEq => u16::from(regs.cpu_get(self.lhs).unwrap()) >= self.rhs,
            Operator::InfEq => u16::from(regs.cpu_get(self.lhs).unwrap()) <= self.rhs,
        }
    }

    pub fn new_simple(address: u16) -> Self {
        Self {
            lhs: CpuRegs::PC,
            op: Operator::Eq,
            rhs: address
        }
    }
}

impl FromStr for BreakpointNode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, (reg, op, val)) = match tuple((register, operator, value))(s) {
            Ok(ret) => ret,
            Err(_) => {
                return Err(anyhow!("Invalid input"));
            }
        };
        if !rest.is_empty() {
            Err(anyhow!("Invalid input"))
        }
        else {
            Ok(
                BreakpointNode {
                    lhs: reg,
                    op,
                    rhs: val,
                }
            )
        }
    }
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::NotEq),
        map(tag(">"), |_| Operator::Sup),
        map(tag(">="), |_| Operator::SupEq),
        map(tag("<"), |_| Operator::Inf),
        map(tag("<="), |_| Operator::InfEq),
    ))(input)
}

fn register(input: &str) -> IResult<&str, CpuRegs> {
    alt((
        map(tag("AF"), |_| CpuRegs::AF),
        map(tag("BC"), |_| CpuRegs::BC),
        map(tag("DE"), |_| CpuRegs::DE),
        map(tag("HL"), |_| CpuRegs::HL),
        map(tag("PC"), |_| CpuRegs::PC),
        map(tag("SP"), |_| CpuRegs::SP),
    ))(input)
}

fn value(input: &str) -> IResult<&str, u16> {
    map(
        take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()),
        |s| u16::from_str_radix(s, 16).unwrap()
    )(input)
}
