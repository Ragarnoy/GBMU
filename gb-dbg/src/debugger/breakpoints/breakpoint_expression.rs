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
use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};
use std::fmt::{self, Display};
use std::str::FromStr;

macro_rules! boxed {
    ($any:expr) => {
        Box::new($any)
    };
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
    And,
    /// Logical XOR, `^^`
    Xor,
    /// Logical OR, `||`
    Or,
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

            Operator::And => write!(f, "&&"),
            Operator::Xor => write!(f, "^^"),
            Operator::Or => write!(f, "||"),
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

        let (_rest, node) = match all_consuming(expr_complete)(s) {
            Ok(ret) => ret,
            Err(e) => {
                return Err(anyhow!("Invalid input: {}", e));
            }
        };
        Ok(node)
    }
}

pub fn expr_complete(input: &str) -> IResult<&str, Node> {
    use nom::combinator::complete;

    alt((
        map(
            complete(tuple((operation, comb_op, expr_complete))),
            |(lhs, op, rhs)| Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            },
        ),
        complete(operation),
    ))(input)
}

pub fn expr(input: &str) -> IResult<&str, Node> {
    alt((
        map(tuple((operation, comb_op, expr)), |(lhs, op, rhs)| {
            Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            }
        }),
        operation,
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Node> {
    map(tuple((any_value, bin_op, any_value)), |(lhs, op, rhs)| {
        Node::BinaryExpr {
            op,
            lhs: boxed!(lhs),
            rhs: boxed!(rhs),
        }
    })(input)
}

fn comb_op(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("&&"), |_| Operator::And),
        map(tag("||"), |_| Operator::Or),
        map(tag("^^"), |_| Operator::Xor),
    ))(input)
}

fn any_value(input: &str) -> IResult<&str, Node> {
    alt((unary_expr, value))(input)
}

fn unary_expr(input: &str) -> IResult<&str, Node> {
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

fn unary_expr_id(input: &str) -> IResult<&str, UnaryOperator> {
    alt((
        map(tag("L"), |_| UnaryOperator::Lower),
        map(tag("U"), |_| UnaryOperator::Upper),
    ))(input)
}

fn register(input: &str) -> IResult<&str, Node> {
    map(raw_register, Node::Register)(input)
}

fn raw_register(input: &str) -> IResult<&str, CpuRegs> {
    alt((
        map(tag("AF"), |_| CpuRegs::AF),
        map(tag("BC"), |_| CpuRegs::BC),
        map(tag("DE"), |_| CpuRegs::DE),
        map(tag("HL"), |_| CpuRegs::HL),
        map(tag("PC"), |_| CpuRegs::PC),
        map(tag("SP"), |_| CpuRegs::SP),
    ))(input)
}

fn value(input: &str) -> IResult<&str, Node> {
    alt((register, map(raw_value, Node::Value), address))(input)
}

fn raw_value(input: &str) -> IResult<&str, u16> {
    use nom::bytes::complete::take_while_m_n;

    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

fn address(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("*")(input)?;

    map(raw_value, Node::Address)(input)
}

fn bin_op(input: &str) -> IResult<&str, Operator> {
    use nom::bytes::complete::tag;

    alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::NotEq),
        map(tag("<="), |_| Operator::InfEq),
        map(tag(">="), |_| Operator::SupEq),
        map(tag(">"), |_| Operator::Sup),
        map(tag("<"), |_| Operator::Inf),
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
}

#[test]
fn test_raw_register() {
    assert_eq!(raw_register("AF"), Ok(("", CpuRegs::AF)));
    assert_eq!(raw_register("BC"), Ok(("", CpuRegs::BC)));
    assert_eq!(raw_register("DE"), Ok(("", CpuRegs::DE)));
    assert_eq!(raw_register("HL"), Ok(("", CpuRegs::HL)));
    assert_eq!(raw_register("PC"), Ok(("", CpuRegs::PC)));
    assert_eq!(raw_register("SP"), Ok(("", CpuRegs::SP)));
}

#[test]
fn test_comb_op() {
    assert_eq!(comb_op("&&"), Ok(("", Operator::And)));
    assert_eq!(comb_op("||"), Ok(("", Operator::Or)));
    assert_eq!(comb_op("^^"), Ok(("", Operator::Xor)));
}

#[test]
fn test_raw_value() {
    assert_eq!(raw_value("1"), Ok(("", 0x1_u16)));
    assert_eq!(raw_value("1f"), Ok(("", 0x1f_u16)));
    assert_eq!(raw_value("b1f"), Ok(("", 0xb1f_u16)));
    assert_eq!(raw_value("ab1f"), Ok(("", 0xab1f_u16)));
}

#[test]
fn test_address() {
    assert_eq!(address("*1"), Ok(("", Node::Address(1))));
    assert_eq!(address("*dead"), Ok(("", Node::Address(0xdead))));
}

#[test]
fn test_unary_expr_id() {
    assert_eq!(unary_expr_id("U"), Ok(("", UnaryOperator::Upper)));
    assert_eq!(unary_expr_id("L"), Ok(("", UnaryOperator::Lower)));
}

#[test]
fn test_unary_expr() {
    assert_eq!(
        unary_expr("U(AF)"),
        Ok((
            "",
            Node::UnaryExpr {
                op: UnaryOperator::Upper,
                child: boxed!(Node::Register(CpuRegs::AF))
            }
        ))
    );
}

#[cfg(test)]
fn utils_test_expr<'a, P>(parser: P, input: &'a str, expected: &str)
where
    P: nom::Parser<&'a str, Node, nom::error::Error<&'a str>>,
{
    use nom::combinator::all_consuming;

    let res = all_consuming(parser)(input);
    assert!(res.is_ok(), "for `{}': res is not ok: {:?}", input, res);
    let (left, expr) = res.unwrap();
    assert!(left.is_empty(), "data still need to be proceded: {}", left);
    assert_eq!(expr.to_string(), expected);
}

#[test]
fn test_operation() {
    utils_test_expr(operation, "AF==42", "AF == 0x42");
    utils_test_expr(operation, "SP<=fffe", "SP <= 0xFFFE");
    utils_test_expr(operation, "HL!=*ff0f", "HL != *0xFF0F");
    utils_test_expr(operation, "HL<DE", "HL < DE");
}

#[test]
fn test_expr() {
    utils_test_expr(expr_complete, "AF==42", "AF == 0x42");
    utils_test_expr(
        expr_complete,
        "AF==21||PC==dead",
        "AF == 0x21 || PC == 0xDEAD",
    );
}
