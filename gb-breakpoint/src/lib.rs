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
pub enum Node {
    Register(Register),
    Address(Box<Node>),
    Raw(u16),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Register(r) => write!(f, "{}", r),
            Node::Address(addr) => write!(f, "*{}", addr),
            Node::Raw(v) => write!(f, "{:#X}", v),
            Node::UnaryExpr(expr) => write!(f, "{}", expr),
            Node::BinaryExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Node {
    pub fn simple(address: u16) -> Self {
        Self::BinaryExpr(BinaryExpr {
            op: Operator::Eq,
            lhs: boxed!(Self::Register(Register::PC)),
            rhs: boxed!(Self::Raw(address)),
        })
    }
}

impl FromStr for Node {
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
