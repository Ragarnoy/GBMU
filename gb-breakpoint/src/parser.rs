use crate::{
    boxed,
    operation::{comb_op, operation},
    unary::unary_expr,
    wrapper::{wrap_register, wrap_value},
    Ast,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

/// Parse a breakpoint expression to generate an [Ast]
///
/// # Definition
///
/// ```txt
/// expr |= operation comb_op expr
///      |= operation
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::expr;
/// assert!(expr("AF == DEAD").is_ok());
/// assert!(expr("PC == 42 && *FF0F == 5").is_ok());
/// ```
pub fn expr(input: &str) -> IResult<&str, Ast> {
    alt((
        map(tuple((operation, ws(comb_op), expr)), |(lhs, op, rhs)| {
            Ast::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            }
        }),
        operation,
    ))(input)
}

/// Skip surounding whitespaces
pub fn ws<I, O, E, P>(parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
    P: nom::Parser<I, O, E>,
{
    use nom::character::complete::space0;

    delimited(space0, parser, space0)
}

/// Parser a value that can be wrapped in a [UnaryExpr]
///
/// # Definition
///
/// ```txt
/// any_value = unary | value
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::any_value;
/// assert!(any_value("42").is_ok());
/// assert!(any_value("U(AF)").is_ok());
/// ```
pub fn any_value(input: &str) -> IResult<&str, Ast> {
    alt((unary_expr, wrap_value))(input)
}

/// Parse a value
///
/// # Definition
///
/// ```txt
/// value |= register
///       |= u16
///       |= adress
/// ```
pub fn value(input: &str) -> IResult<&str, Ast> {
    alt((wrap_register, wrap_value, address))(input)
}

/// Parse an [Ast::Address]
///
/// # Definition
///
/// ```txt
/// address = '*' raw_value
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::address;
/// assert!(address("*42").is_ok());
/// ```
pub fn address(input: &str) -> IResult<&str, Ast> {
    let (input, _) = tag("*")(input)?;

    map(crate::native::value, Ast::Address)(input)
}
