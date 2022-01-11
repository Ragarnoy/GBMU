use crate::{boxed, wrapper::wrap_register, Ast};
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited, IResult};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: UnaryOperator,
    pub child: Box<Ast>,
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.op, self.child)
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

/// Parse a [Ast::UnaryExpr]
///
/// # Definition
///
/// ```txt
/// unary_expr = (L|U) '(' register ')'
/// ```
pub fn unary_expr(input: &str) -> IResult<&str, UnaryExpr> {
    let (input, unary_op) = unary_expr_id(input)?;

    let (input, reg) = delimited(tag("("), wrap_register, tag(")"))(input)?;
    Ok((
        input,
        UnaryExpr {
            op: unary_op,
            child: boxed!(reg),
        },
    ))
}

#[test]
fn test_unary_expr() {
    use crate::Register;

    assert_eq!(
        unary_expr("U(AF)"),
        Ok((
            "",
            UnaryExpr {
                op: UnaryOperator::Upper,
                child: crate::boxed!(Ast::Register(Register::AF))
            }
        ))
    );
}

/// Parse a [UnaryOperator] identifier
///
/// ```
/// # use gb_breakpoint::unary::unary_expr_id;
/// assert!(unary_expr_id("U").is_ok());
/// ```
pub fn unary_expr_id(input: &str) -> IResult<&str, UnaryOperator> {
    alt((
        map(tag("L"), |_| UnaryOperator::Lower),
        map(tag("U"), |_| UnaryOperator::Upper),
    ))(input)
}

#[test]
fn test_unary_expr_id() {
    assert_eq!(unary_expr_id("U"), Ok(("", UnaryOperator::Upper)));
    assert_eq!(unary_expr_id("L"), Ok(("", UnaryOperator::Lower)));
}
