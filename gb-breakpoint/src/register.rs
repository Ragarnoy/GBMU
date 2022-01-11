use std::fmt::Display;

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AF => write!(f, "AF"),
            Register::BC => write!(f, "BC"),
            Register::DE => write!(f, "DE"),
            Register::HL => write!(f, "HL"),
            Register::SP => write!(f, "SP"),
            Register::PC => write!(f, "PC"),
        }
    }
}

/// ```
/// # use gb_breakpoint::register::register
///
/// assert_eq!(register("AF"), Ok(("", Register::AF)));
/// assert!(register("foo").is_err());
/// ```
pub fn register(input: &str) -> IResult<&str, Register> {
    alt((
        map(tag("AF"), |_| Register::AF),
        map(tag("BC"), |_| Register::BC),
        map(tag("DE"), |_| Register::DE),
        map(tag("HL"), |_| Register::HL),
        map(tag("SP"), |_| Register::SP),
        map(tag("PC"), |_| Register::PC),
    ))(input)
}

#[test]
fn test_raw_register() {
    assert_eq!(register("AF"), Ok(("", Register::AF)));
    assert_eq!(register("BC"), Ok(("", Register::BC)));
    assert_eq!(register("DE"), Ok(("", Register::DE)));
    assert_eq!(register("HL"), Ok(("", Register::HL)));
    assert_eq!(register("PC"), Ok(("", Register::PC)));
    assert_eq!(register("SP"), Ok(("", Register::SP)));
}
