use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Debug)]
pub enum Condition {
    Zero,
    Carry,
    NotZero,
    NotCarry,
}

impl Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "Z"),
            Self::Carry => write!(f, "C"),
            Self::NotZero => write!(f, "NZ"),
            Self::NotCarry => write!(f, "NC"),
        }
    }
}

#[test]
fn test_display() {
    assert_eq!(Condition::Zero.to_string(), "Z");
    assert_eq!(Condition::NotZero.to_string(), "NZ");
    assert_eq!(Condition::Carry.to_string(), "C");
    assert_eq!(Condition::NotCarry.to_string(), "NC");
}
