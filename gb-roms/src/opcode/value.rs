use super::register::{Register, Register16Bits, Register8Bits};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Register(Register),
    IndirectReg16(Register16Bits),
    /// Use the addr that result of `addr = Reg + 0xff00`
    IndirectReg8(Register8Bits),
    Indirect16(u16),
    /// Use the addr that result of `addr = n + 0xff00`
    Indirect8(u8),
    Nn(u16),
    N(u8),
    D(i8),
}

impl From<Register> for Value {
    fn from(r: Register) -> Self {
        Self::Register(r)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Self::Nn(v)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::N(v)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Self::D(v)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{}", reg),
            Self::IndirectReg16(reg) => write!(f, "({})", reg),
            Self::IndirectReg8(reg) => write!(f, "(0xff00 + {})", reg),
            Self::Indirect16(adr) => write!(f, "({:x})", adr),
            Self::Indirect8(addr) => write!(f, "(0xff00 + {:x})", addr),
            Self::Nn(v) => write!(f, "{:x}", v),
            Self::N(v) => write!(f, "{:x}", v),
            Self::D(v) => write!(f, "{:x}", v),
        }
    }
}

#[test]
fn test_value_display() {
    assert_eq!(Value::Register(Register8Bits::A.into()).to_string(), "A");
    assert_eq!(Value::Nn(0x1023_u16).to_string(), "1023");
    assert_eq!(Value::N(0x23_u8).to_string(), "23");
}
