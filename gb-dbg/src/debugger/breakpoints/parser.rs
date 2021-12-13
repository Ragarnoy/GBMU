//! ## Parser Definition
//!
//! ```ignore
//! expr = operation comb_op expr
//! operation = any_value bin_op any_value
//!
//! comb_op |= '&&'
//!         |= '||'
//!         |= '^^'
//!
//! bin_op |= '=='
//!        |= '!='
//!        |= '>'
//!        |= '<'
//!        |= '>='
//!        |= '<='
//!        |= '&'
//!        |= '|'
//!        |= '^'
//!
//! any_value |= unary
//!           |= value
//!
//! unary = transformator
//!
//! transformator = (L|U) '(' register ')'
//!
//! register |= 'AF'
//!          |= 'BC'
//!          |= 'DE'
//!          |= 'HL'
//!          |= 'PC'
//!          |= 'SP'
//!
//! value |= register
//!       |= address
//!       |= raw_value
//!
//! address = '*' raw_value
//!
//! raw_value = [A-Fa-f0-9]{1,4}
//! ```

use crate::dbg_interfaces::CpuRegs;
use crate::debugger::breakpoints::breakpoint::{Node, Operator, UnaryOperator};
use nom::{
    branch::alt,
    bytes::streaming::tag,
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

pub fn expr_complete(input: &str) -> IResult<&str, Node> {
    use nom::combinator::complete;

    alt((
        map(
            complete(tuple((operation, ws(comb_op), expr_complete))),
            |(lhs, op, rhs)| Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            },
        ),
        complete(operation),
    ))(input)
}

/*pub fn expr(input: &str) -> IResult<&str, Node> {
    alt((
        map(tuple((operation, ws(comb_op), expr)), |(lhs, op, rhs)| {
            Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            }
        }),
        operation,
    ))(input)
}
*/
pub fn operation(input: &str) -> IResult<&str, Node> {
    map(
        tuple((any_value, ws(bin_op), any_value)),
        |(lhs, op, rhs)| Node::BinaryExpr {
            op,
            lhs: boxed!(lhs),
            rhs: boxed!(rhs),
        },
    )(input)
}

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

pub fn comb_op(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("&&"), |_| Operator::LogicAnd),
        map(tag("||"), |_| Operator::LogicOr),
        map(tag("^^"), |_| Operator::LogicXor),
    ))(input)
}

pub fn any_value(input: &str) -> IResult<&str, Node> {
    alt((unary_expr, value))(input)
}

pub fn unary_expr(input: &str) -> IResult<&str, Node> {
    let (input, unary_op) = unary_expr_id(input)?;

    let (input, reg) = delimited(tag("("), register, tag(")"))(input)?;
    Ok((
        input,
        Node::UnaryExpr {
            op: unary_op,
            child: boxed!(reg),
        },
    ))
}

pub fn unary_expr_id(input: &str) -> IResult<&str, UnaryOperator> {
    alt((
        map(tag("L"), |_| UnaryOperator::Lower),
        map(tag("U"), |_| UnaryOperator::Upper),
    ))(input)
}

pub fn register(input: &str) -> IResult<&str, Node> {
    map(raw_register, Node::Register)(input)
}

pub fn raw_register(input: &str) -> IResult<&str, CpuRegs> {
    alt((
        map(tag("AF"), |_| CpuRegs::AF),
        map(tag("BC"), |_| CpuRegs::BC),
        map(tag("DE"), |_| CpuRegs::DE),
        map(tag("HL"), |_| CpuRegs::HL),
        map(tag("PC"), |_| CpuRegs::PC),
        map(tag("SP"), |_| CpuRegs::SP),
    ))(input)
}

pub fn value(input: &str) -> IResult<&str, Node> {
    alt((register, map(raw_value, Node::Value), address))(input)
}

pub fn raw_value(input: &str) -> IResult<&str, u16> {
    use nom::bytes::complete::take_while_m_n;

    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

pub fn address(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("*")(input)?;

    map(raw_value, Node::Address)(input)
}

pub fn bin_op(input: &str) -> IResult<&str, Operator> {
    use nom::bytes::complete::tag;

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
