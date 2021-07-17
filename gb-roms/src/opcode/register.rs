use std::convert::TryFrom;

use super::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Register {
	// Accumulator 8-bits register
	A,

	// Auxiliary 8-bits register
	B,
	C,
	D,
	E,
	F,
	H,
	L,

	// Auxiliary 16-bits register
	HL,

	// Program Counter 16-bits register
	PC,

	// Stack Pointer 16-bits register
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
