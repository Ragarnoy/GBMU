pub mod breakpoint;
pub mod native;
pub mod parser;
pub mod register;
#[cfg(test)]
mod test_parser;
pub mod unary;
mod wrapper;

use std::{
    fmt::{self, Display},
    str::FromStr,
};

pub use breakpoint::Operator;
use register::Register;
pub use unary::UnaryOperator;

#[macro_export]
macro_rules! boxed {
    ($any:expr) => {
        Box::new($any)
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Register(Register),
    Address(u16),
    Raw(u16),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Ast>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
}

impl Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ast::Register(r) => write!(f, "{}", r),
            Ast::Address(addr) => write!(f, "*{:#X}", addr),
            Ast::Raw(v) => write!(f, "{:#X}", v),
            Ast::UnaryExpr { op, child } => write!(f, "{}({})", op, child),
            Ast::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

impl Ast {
    pub fn simple(address: u16) -> Self {
        Self::BinaryExpr {
            op: Operator::Eq,
            lhs: boxed!(Self::Register(Register::PC)),
            rhs: boxed!(Self::Raw(address)),
        }
    }
}

impl FromStr for Ast {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::combinator::all_consuming;

        let (_rest, ast) = match all_consuming(parser::expr)(s) {
            Ok(ret) => ret,
            Err(e) => {
                return Err(format!("Invalid input: {}", e));
            }
        };
        Ok(ast)
    }
}
