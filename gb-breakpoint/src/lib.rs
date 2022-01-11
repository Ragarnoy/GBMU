pub mod native;
pub mod operation;
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

use operation::BinaryExpr;
pub use operation::Operator;
use register::Register;
use unary::UnaryExpr;
pub use unary::UnaryOperator;

#[macro_export]
macro_rules! boxed {
    ($any:expr) => {
        Box::new($any)
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Register(Register),
    Address(u16),
    Raw(u16),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
}

impl Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ast::Register(r) => write!(f, "{}", r),
            Ast::Address(addr) => write!(f, "*{:#X}", addr),
            Ast::Raw(v) => write!(f, "{:#X}", v),
            Ast::UnaryExpr(expr) => write!(f, "{}", expr),
            Ast::BinaryExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Ast {
    pub fn simple(address: u16) -> Self {
        Self::BinaryExpr(BinaryExpr {
            op: Operator::Eq,
            lhs: boxed!(Self::Register(Register::PC)),
            rhs: boxed!(Self::Raw(address)),
        })
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
