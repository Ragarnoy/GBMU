use crate::{
    breakpoint::{Operator, UnaryOperator},
    register, Ast,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

#[macro_export]
macro_rules! boxed {
    ($any:expr) => {
        Box::new($any)
    };
}

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

/// Skip surounding whitespaces
fn ws<I, O, E, P>(parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
    P: nom::Parser<I, O, E>,
{
    use nom::character::complete::space0;

    delimited(space0, parser, space0)
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
    alt((unary_expr, value))(input)
}

/// Parse a [Ast::UnaryExpr]
///
/// # Definition
///
/// ```txt
/// unary_expr = (L|U) '(' register ')'
/// ```
pub fn unary_expr(input: &str) -> IResult<&str, Ast> {
    let (input, unary_op) = unary_expr_id(input)?;

    let (input, reg) = delimited(tag("("), register, tag(")"))(input)?;
    Ok((
        input,
        Ast::UnaryExpr {
            op: unary_op,
            child: boxed!(reg),
        },
    ))
}

/// Parse a [UnaryOperator] identifier
///
/// ```
/// # use gb_breakpoint::parser::unary_expr_id;
/// assert!(unary_expr_id("U").is_ok());
/// ```
pub fn unary_expr_id(input: &str) -> IResult<&str, UnaryOperator> {
    alt((
        map(tag("L"), |_| UnaryOperator::Lower),
        map(tag("U"), |_| UnaryOperator::Upper),
    ))(input)
}

/// Wrap [u16] in [Ast::Value]
pub fn value(input: &str) -> IResult<&str, Ast> {
    alt((register, map(raw_value, Ast::Value), address))(input)
}

/// Wrap [register::register] in [Ast::Register]
pub fn register(input: &str) -> IResult<&str, Ast> {
    map(register::register, Ast::Register)(input)
}

/// Parse an [u16]
///
/// # Definition
///
/// ```txt
/// raw_value = [A-Fa-f0-9]{1,4}
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::raw_value;
/// assert_eq!(raw_value("42"), Ok(("", 0x42)));
/// ```
pub fn raw_value(input: &str) -> IResult<&str, u16> {
    use nom::bytes::complete::take_while_m_n;

    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
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

    map(raw_value, Ast::Address)(input)
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
