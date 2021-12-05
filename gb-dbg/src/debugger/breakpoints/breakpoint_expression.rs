use crate::dbg_interfaces::CpuRegs;
use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::streaming::{tag, take_while_m_n},
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

/// ## Parser Definition
///
/// ```
/// expr |= expr comb_op expr
///      |= any_value bin_op any_value
///
/// comb_op |= '&&'
///         |= '||'
///         |= '^^'
///
/// bin_op |= '=='
///        |= '!='
///        |= '>'
///        |= '<'
///        |= '>='
///        |= '<='
///
/// any_value |= unary
///           |= value
///
/// unary = transformator
///
/// transformator = (L|U) '(' register ')'
///
/// register |= 'AF'
///          |= 'BC'
///          |= 'DE'
///          |= 'HL'
///          |= 'PC'
///          |= 'SP'
///
/// value |= register
///       |= address
///       |= raw_value
///
/// address = '*' raw_value
///
/// raw_value = [A-Fa-f0-9]{1,4}
/// ```
#[derive(Debug)]
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
            Node::Address(addr) => write!(f, "*{}", addr),
            Node::Value(v) => write!(f, "{}", v),
            Node::UnaryExpr { op, child } => write!(f, "{}({})", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, rhs, op),
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
        let (rest, node) = match expr(s) {
            Ok(ret) => ret,
            Err(e) => {
                return Err(anyhow!("Invalid input: {:?}", e));
            }
        };
        if !rest.is_empty() {
            Err(anyhow!("Invalid input: not everything was consumed"))
        } else {
            Ok(node)
        }
    }
}

fn expr(input: &str) -> IResult<&str, Node> {
    alt((
        map(tuple((expr, comb_op, expr)), |(lhs, op, rhs)| {
            Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            }
        }),
        map(tuple((any_value, bin_op, any_value)), |(lhs, op, rhs)| {
            Node::BinaryExpr {
                op,
                lhs: boxed!(lhs),
                rhs: boxed!(rhs),
            }
        }),
    ))(input)
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
    let (input, unary_op) = alt((tag("L"), tag("U")))(input)?;
    let unary_op = match unary_op {
        "L" => UnaryOperator::Lower,
        "H" => UnaryOperator::Upper,
        _ => panic!("unexpected valid unary op `{}'", unary_op),
    };

    let (input, reg) = delimited(tag("("), register, tag(")"))(input)?;
    Ok((
        input,
        Node::UnaryExpr {
            op: unary_op,
            child: boxed!(reg),
        },
    ))
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
    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}

fn address(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("*")(input)?;

    map(raw_value, Node::Address)(input)
}

fn bin_op(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag("=="), |_| Operator::Eq),
        map(tag("!="), |_| Operator::NotEq),
        map(tag(">"), |_| Operator::Sup),
        map(tag(">="), |_| Operator::SupEq),
        map(tag("<"), |_| Operator::Inf),
        map(tag("<="), |_| Operator::InfEq),
    ))(input.trim())
}
