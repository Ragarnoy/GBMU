use std::{convert::TryFrom, fmt};

use super::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
	/// Accumulator 8-bits register
	A,

	/// Auxiliary 8-bits register
	B,
	C,
	D,
	E,
	F,
	H,
	L,

	/// Auxiliary 16-bits register
	HL,

	/// Program Counter 16-bits register
	PC,

	/// Stack Pointer 16-bits register
	SP,
}

impl TryFrom<u8> for Register {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0b000 => Ok(Register::B),
			0b001 => Ok(Register::C),
			0b010 => Ok(Register::D),
			0b011 => Ok(Register::E),
			0b100 => Ok(Register::H),
			0b101 => Ok(Register::L),
			0b111 => Ok(Register::A),
			_ => Err(Error::InvalidRegisterValue(v)),
		}
	}
}

#[test]
fn test_convert_register() {
	assert_eq!(Register::try_from(0), Ok(Register::B));
	assert_eq!(Register::try_from(1), Ok(Register::C));
	assert_eq!(Register::try_from(2), Ok(Register::D));
	assert_eq!(Register::try_from(3), Ok(Register::E));
	assert_eq!(Register::try_from(4), Ok(Register::H));
	assert_eq!(Register::try_from(5), Ok(Register::L));
	assert_eq!(Register::try_from(7), Ok(Register::A));
}

impl fmt::Display for Register {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Register::A => write!(f, "A"),
			Register::B => write!(f, "B"),
			Register::C => write!(f, "C"),
			Register::D => write!(f, "D"),
			Register::E => write!(f, "E"),
			Register::F => write!(f, "F"),
			Register::H => write!(f, "H"),
			Register::L => write!(f, "L"),
			Register::HL => write!(f, "HL"),
			Register::PC => write!(f, "PC"),
			Register::SP => write!(f, "SP"),
		}
	}
}

#[test]
fn test_register_display() {
	assert_eq!(Register::A.to_string(), "A");
	assert_eq!(Register::B.to_string(), "B");
	assert_eq!(Register::C.to_string(), "C");
	assert_eq!(Register::D.to_string(), "D");
	assert_eq!(Register::E.to_string(), "E");
	assert_eq!(Register::F.to_string(), "F");
	assert_eq!(Register::H.to_string(), "H");
	assert_eq!(Register::L.to_string(), "L");
	assert_eq!(Register::HL.to_string(), "HL");
	assert_eq!(Register::PC.to_string(), "PC");
	assert_eq!(Register::SP.to_string(), "SP");
}
