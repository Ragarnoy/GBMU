use crate::dbg_interfaces::{CpuRegs, DebugOperations};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::fmt;
use anyhow::anyhow;
use crate::boxed;
use crate::debugger::breakpoints::evaluation::{compute_expression, is_expression_true};

#[derive(Debug)]
pub struct Breakpoint {
    expr: Node,
    pub enabled: bool,
    last_state: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            expr: Node::simple(address),
            enabled: true,
            last_state: false,
        }
    }

    pub fn from_expression(expr: &str) -> anyhow::Result<Self> {
        let node = Node::from_str(expr)?;
        Ok(Self {
            expr: node,
            enabled: true,
            last_state: false,
        })
    }

    pub fn is_triggered<T: DebugOperations>(&self, regs: &T) -> bool {
        self.enabled && is_expression_true(&self.expr, regs)
    }

    /// check if breakpoint is active
    /// this method is used to prevent the breakpoint to trigger itself on the same session
    pub fn active<DBG: DebugOperations>(&mut self, context: &DBG) -> bool {
        let current_state = self.is_triggered(context);
        let result = !self.last_state && current_state;

        self.last_state = current_state;

        result
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    /// Get the upper bound of a value, U(<r16>)
    Upper,
    /// Get the lower bound of a value, L(<r16>)
    Lower,
    //Raising,
    //Falling,
    //Update,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::Upper => write!(f, "U"),
            UnaryOperator::Lower => write!(f, "L"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    /// Check for equality of value between another, `==`
    Eq,
    /// Logical AND, `&&`
    LogicAnd,
    /// Logical XOR, `^^`
    LogicXor,
    /// Logical OR, `||`
    LogicOr,
    /// Binary AND, `&`
    BinaryAnd,
    /// Binary XOR, `^`
    BinaryXor,
    /// Binary OR, `|`
    BinaryOr,
    /// Check for inequality of a value between another, `!=`
    NotEq,
    /// Check if a value is greather than another, `>`
    Sup,
    /// Check if a value is lower than another, `<`
    Inf,
    /// Check if a value is greather or equal to another, `>=`
    SupEq,
    /// Check if a value is lower or equal to another, `<=`
    InfEq,
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Eq => write!(f, "=="),
            Operator::NotEq => write!(f, "!="),
            Operator::Sup => write!(f, ">"),
            Operator::Inf => write!(f, "<"),
            Operator::SupEq => write!(f, ">="),
            Operator::InfEq => write!(f, "<="),

            Operator::BinaryAnd => write!(f, "&"),
            Operator::BinaryXor => write!(f, "^"),
            Operator::BinaryOr => write!(f, "|"),

            Operator::LogicAnd => write!(f, "&&"),
            Operator::LogicXor => write!(f, "^^"),
            Operator::LogicOr => write!(f, "||"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Register(CpuRegs),
    Address(u16),
    Value(u16),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Register(r) => write!(f, "{}", r),
            Node::Address(addr) => write!(f, "*{:#X}", addr),
            Node::Value(v) => write!(f, "{:#X}", v),
            Node::UnaryExpr { op, child } => write!(f, "{}({})", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

impl Node {
    pub fn simple(address: u16) -> Self {
        Self::BinaryExpr {
            op: Operator::Eq,
            lhs: boxed!(Self::Register(CpuRegs::PC)),
            rhs: boxed!(Self::Value(address)),
        }
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::combinator::all_consuming;
        use crate::debugger::breakpoints::parser;

        let (_rest, node) = match all_consuming(parser::expr_complete)(s) {
            Ok(ret) => ret,
            Err(e) => {
                return Err(anyhow!("Invalid input: {}", e));
            }
        };
        Ok(node)
    }
}
