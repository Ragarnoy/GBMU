use super::register::{Register, Register16Bits, Register8Bits};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Store {
    /// Register Id
    Register(Register),
    /// Use the addr to register
    IndirectReg16(Register16Bits),
    /// Use the addr that result of `addr = Reg + 0xff00`
    IndierectReg8(Register8Bits),
    /// Addresse in memory (should be!)
    Indirect16(u16),
    /// Use the addr that result of `addr = n + 0xff00`
    Indirect8(u8),
}

impl From<Register> for Store {
    fn from(r: Register) -> Self {
        Self::Register(r)
    }
}

impl From<u16> for Store {
    fn from(v: u16) -> Self {
        Self::Indirect16(v)
    }
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{}", reg),
            Self::IndirectReg16(reg) => write!(f, "({})", reg),
            Self::IndierectReg8(reg) => write!(f, "(0xff00 + {})", reg),
            Self::Indirect16(addr) => write!(f, "({:x})", addr),
            Self::Indirect8(addr) => write!(f, "(0xff00 + {:x})", addr),
        }
    }
}

impl Store {
    pub fn from_r_table(v: u8) -> Option<Store> {
        match v {
            0 => Some(register8!(B).into()),
            1 => Some(register8!(C).into()),
            2 => Some(register8!(D).into()),
            3 => Some(register8!(E).into()),
            4 => Some(register8!(H).into()),
            5 => Some(register8!(L).into()),
            6 => Some(Store::IndirectReg16(Register16Bits::HL)),
            7 => Some(register8!(A).into()),
            _ => None,
        }
    }
}

#[test]
fn test_store_display() {
    assert_eq!(Store::Register(Register8Bits::A.into()).to_string(), "A");
    assert_eq!(Store::Indirect16(0x3a).to_string(), "(3a)");
}
