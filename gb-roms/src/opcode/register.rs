use std::{convert::From, fmt};

macro_rules! register8 {
    ($x:ident) => {
        Register::Bits8(Register8Bits::$x)
    };
}

macro_rules! register16 {
    ($x:ident) => {
        Register::Bits16(Register16Bits::$x)
    };
}

macro_rules! register_special {
    ($x:ident) => {
        Register::Special(RegisterSpecial::$x)
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    Bits8(Register8Bits),
    Bits16(Register16Bits),
    Special(RegisterSpecial),
    Flag(RegisterFlag),
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::Bits8(r) => write!(f, "{}", r),
            Register::Bits16(r) => write!(f, "{}", r),
            Register::Special(r) => write!(f, "{}", r),
            Register::Flag(r) => write!(f, "{}", r),
        }
    }
}

impl From<Register8Bits> for Register {
    fn from(v: Register8Bits) -> Self {
        Self::Bits8(v)
    }
}

impl From<Register16Bits> for Register {
    fn from(v: Register16Bits) -> Self {
        Self::Bits16(v)
    }
}

impl From<RegisterSpecial> for Register {
    fn from(v: RegisterSpecial) -> Self {
        Self::Special(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register8Bits {
    /// Accumulator 8-bits register
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl fmt::Display for Register8Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register8Bits::A => write!(f, "A"),
            Register8Bits::B => write!(f, "B"),
            Register8Bits::C => write!(f, "C"),
            Register8Bits::D => write!(f, "D"),
            Register8Bits::E => write!(f, "E"),
            Register8Bits::F => write!(f, "F"),
            Register8Bits::H => write!(f, "H"),
            Register8Bits::L => write!(f, "L"),
        }
    }
}

#[test]
fn test_register8bits_display() {
    assert_eq!(Register8Bits::A.to_string(), "A");
    assert_eq!(Register8Bits::B.to_string(), "B");
    assert_eq!(Register8Bits::C.to_string(), "C");
    assert_eq!(Register8Bits::D.to_string(), "D");
    assert_eq!(Register8Bits::E.to_string(), "E");
    assert_eq!(Register8Bits::F.to_string(), "F");
    assert_eq!(Register8Bits::H.to_string(), "H");
    assert_eq!(Register8Bits::L.to_string(), "L");
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterSpecial {
    /// Program Counter 16-bits register
    PC,

    /// Stack Pointer 16-bits register
    SP,
}

impl fmt::Display for RegisterSpecial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterSpecial::PC => write!(f, "PC"),
            RegisterSpecial::SP => write!(f, "SP"),
        }
    }
}

#[test]
fn test_register_special_display() {
    assert_eq!(RegisterSpecial::PC.to_string(), "PC");
    assert_eq!(RegisterSpecial::SP.to_string(), "SP");
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register16Bits {
    AF,
    BC,
    DE,
    HL,
}

impl fmt::Display for Register16Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register16Bits::AF => write!(f, "AF"),
            Register16Bits::BC => write!(f, "BC"),
            Register16Bits::DE => write!(f, "DE"),
            Register16Bits::HL => write!(f, "HL"),
        }
    }
}

#[test]
fn test_register16bits_display() {
    assert_eq!(Register16Bits::AF.to_string(), "AF");
    assert_eq!(Register16Bits::BC.to_string(), "BC");
    assert_eq!(Register16Bits::DE.to_string(), "DE");
    assert_eq!(Register16Bits::HL.to_string(), "HL");
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterFlag {
    /// Zero flag
    /// This flag is set when :
    /// - the result of a math op is zero
    /// - `Cmp` OP match 2 values
    Z,

    /// Subtract Flag
    /// This flag is set when the last math instruction was a subtraction
    N,

    /// Half Carry Flag
    /// This flag is set when a carry occurred in the lower nibble of the last math OP
    H,

    /// Carry Flag
    /// This flag is set when :
    /// - a carry occurred in the last math OP
    /// - Reg A is the smaller value when doing a `Cmp` OP
    C,
}

impl fmt::Display for RegisterFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegisterFlag::Z => write!(f, "Z"),
            RegisterFlag::N => write!(f, "N"),
            RegisterFlag::H => write!(f, "H"),
            RegisterFlag::C => write!(f, "C"),
        }
    }
}
