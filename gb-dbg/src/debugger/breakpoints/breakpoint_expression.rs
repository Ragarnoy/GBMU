use crate::dbg_interfaces::{CpuRegs, RegisterDebugOperations};
use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
enum Operand {
    Register(CpuRegs),
    Value(u16),
}

impl Operand {
    fn realize<T: RegisterDebugOperations>(&self, regs: &T) -> u16 {
        match self {
            Operand::Register(r) => regs.cpu_get(*r).into(),
            Operand::Value(v) => *v,
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => {
                write!(f, "{:?}", r)
            }
            Operand::Value(v) => {
                write!(f, "0x{:04X}", v)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
pub struct BreakpointExpression {
    lhs: Operand,
    op: Operator,
    rhs: Operand,
}

impl Display for BreakpointExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

impl BreakpointExpression {
    pub fn compute<T: RegisterDebugOperations>(&self, regs: &T) -> bool {
        match self.op {
            Operator::Eq => self.lhs.realize(regs) == self.rhs.realize(regs),
            Operator::NotEq => self.lhs.realize(regs) != self.rhs.realize(regs),
            Operator::Sup => self.lhs.realize(regs) > self.rhs.realize(regs),
            Operator::Inf => self.lhs.realize(regs) < self.rhs.realize(regs),
            Operator::SupEq => self.lhs.realize(regs) >= self.rhs.realize(regs),
            Operator::InfEq => self.lhs.realize(regs) <= self.rhs.realize(regs),
        }
    }

    pub fn new_simple(address: u16) -> Self {
        Self {
            lhs: Operand::Register(CpuRegs::PC),
            op: Operator::Eq,
            rhs: Operand::Value(address),
        }
    }
}

impl FromStr for BreakpointExpression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, (reg, op, val)) = match tuple((operand, operator, operand))(s) {
            Ok(ret) => ret,
            Err(_) => {
                return Err(anyhow!("Invalid breakpoint input (Cannot parse `{}`)", s));
            }
        };
        if !rest.is_empty() {
            Err(anyhow!("Invalid breakpoint input"))
        } else {
            Ok(BreakpointExpression {
                lhs: reg,
                op,
                rhs: val,
            })
        }
    }
}

fn operand(input: &str) -> IResult<&str, Operand> {
    alt((map(register, Operand::Register), map(value, Operand::Value)))(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::NotEq),
        map(tag(">"), |_| Operator::Sup),
        map(tag(">="), |_| Operator::SupEq),
        map(tag("<"), |_| Operator::Inf),
        map(tag("<="), |_| Operator::InfEq),
    ))(input.trim())
}

fn register(input: &str) -> IResult<&str, CpuRegs> {
    alt((
        map(tag("AF"), |_| CpuRegs::AF),
        map(tag("BC"), |_| CpuRegs::BC),
        map(tag("DE"), |_| CpuRegs::DE),
        map(tag("HL"), |_| CpuRegs::HL),
        map(tag("PC"), |_| CpuRegs::PC),
        map(tag("SP"), |_| CpuRegs::SP),
    ))(input.trim())
}

fn value(input: &str) -> IResult<&str, u16> {
    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input.trim())
}
