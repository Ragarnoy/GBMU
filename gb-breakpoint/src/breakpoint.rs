use std::fmt::{self, Display};

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
