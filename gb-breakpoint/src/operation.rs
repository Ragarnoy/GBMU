use std::fmt::{self, Display};

use crate::{
    boxed,
    parser::{any_value, ws},
    Ast,
};
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};

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

/// Parse a [Operator] token
///
/// # Definition
///
/// ```txt
/// bin_op |= '=='
///        |= '!='
///        |= '>'
///        |= '<'
///        |= '>='
///        |= '<='
///        |= '&'
///        |= '|'
///        |= '^'
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::bin_op;
/// assert!(bin_op("==").is_ok());
/// ```
pub fn bin_op(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::NotEq),
        map(tag("<="), |_| Operator::InfEq),
        map(tag(">="), |_| Operator::SupEq),
        map(tag(">"), |_| Operator::Sup),
        map(tag("<"), |_| Operator::Inf),
        map(tag("|"), |_| Operator::BinaryOr),
        map(tag("&"), |_| Operator::BinaryAnd),
        map(tag("^"), |_| Operator::BinaryXor),
    ))(input.trim())
}

#[test]
fn test_bin_op() {
    assert_eq!(bin_op("=="), Ok(("", Operator::Eq)));
    assert_eq!(bin_op("!="), Ok(("", Operator::NotEq)));
    assert_eq!(bin_op(">="), Ok(("", Operator::SupEq)));
    assert_eq!(bin_op("<="), Ok(("", Operator::InfEq)));
    assert_eq!(bin_op("<"), Ok(("", Operator::Inf)));
    assert_eq!(bin_op(">"), Ok(("", Operator::Sup)));
    assert_eq!(bin_op("|"), Ok(("", Operator::BinaryOr)));
    assert_eq!(bin_op("&"), Ok(("", Operator::BinaryAnd)));
    assert_eq!(bin_op("^"), Ok(("", Operator::BinaryXor)));
}

/// Parse a [Operator] token
///
/// # Definition
///
/// ```txt
/// comb_op |= '&&'
///         |= '||'
///         |= '^^'
/// ```
///
/// # Example
///
/// ```
/// # use gb_breakpoint::parser::comb_op;
/// assert!(comb_op("&&").is_ok());
/// ```
pub fn comb_op(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("&&"), |_| Operator::LogicAnd),
        map(tag("||"), |_| Operator::LogicOr),
        map(tag("^^"), |_| Operator::LogicXor),
    ))(input)
}

#[test]
fn test_comb_op() {
    assert_eq!(comb_op("&&"), Ok(("", Operator::LogicAnd)));
    assert_eq!(comb_op("||"), Ok(("", Operator::LogicOr)));
    assert_eq!(comb_op("^^"), Ok(("", Operator::LogicXor)));
}

/// Parse an expression to generate an [Ast]
///
/// # Definition
///
/// ```txt
/// operation = any_value bin_op any_value
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::operation;
/// assert!(operation("BC == DE").is_ok());
/// ```
pub fn operation(input: &str) -> IResult<&str, Ast> {
    map(
        tuple((any_value, ws(bin_op), any_value)),
        |(lhs, op, rhs)| Ast::BinaryExpr {
            op,
            lhs: boxed!(lhs),
            rhs: boxed!(rhs),
        },
    )(input)
}

#[cfg(test)]
mod unit_operation {
    use crate::operation::operation;
    use crate::test_parser::utils_test_expr;

    #[test]
    fn no_space() {
        utils_test_expr(operation, "AF==42", "AF == 0x42");
        utils_test_expr(operation, "SP<=fffe", "SP <= 0xFFFE");
        utils_test_expr(operation, "HL!=*ff0f", "HL != *0xFF0F");
        utils_test_expr(operation, "HL<DE", "HL < DE");
    }

    #[test]
    fn space() {
        utils_test_expr(operation, "AF ==42", "AF == 0x42");
        utils_test_expr(operation, "SP<= fffe", "SP <= 0xFFFE");
        utils_test_expr(operation, "HL != *ff0f", "HL != *0xFF0F");
        utils_test_expr(operation, "HL < DE", "HL < DE");
    }
}
