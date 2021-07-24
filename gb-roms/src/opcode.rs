mod error;
mod register;

use error::Error;
use modular_bitfield::{
	bitfield,
	specifiers::{B2, B3},
};
use register::Register;
use std::{convert::From, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
	/// jump to addr
	Jump(u16),
	/// relative jump to PC + value
	JumpR(i8),

	Nop,
	Stop,

	/// load value from **left** and load it to **right**
	Ld(Value, Value),
}

impl fmt::Display for Opcode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opcode::Jump(addr) => write!(f, "jmp {:x}", addr),
			Opcode::JumpR(value) => write!(f, "jr {:x}", value),
			Opcode::Nop => write!(f, "nop"),
			Opcode::Stop => write!(f, "stop"),
			Opcode::Ld(from, to) => write!(f, "ld {}, {}", from, to),
		}
	}
}

#[test]
fn test_display_opcode() {
	assert_eq!(Opcode::Jump(0x150).to_string(), "jmp 150");
	assert_eq!(Opcode::JumpR(0x42).to_string(), "jr 42");
	assert_eq!(Opcode::Nop.to_string(), "nop");
	assert_eq!(Opcode::Stop.to_string(), "stop");
	assert_eq!(
		Opcode::Ld(Value::Indirect(0x123), Value::Register(Register::SP)).to_string(),
		"ld (123), SP"
	);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
	Register(Register),
	Indirect(u16),
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Register(reg) => write!(f, "{}", reg),
			Value::Indirect(addr) => write!(f, "({:x})", addr),
		}
	}
}

#[test]
fn test_value_display() {
	assert_eq!(Value::Register(Register::A).to_string(), "A");
	assert_eq!(Value::Indirect(0x3a).to_string(), "(3a)");
}

pub struct OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	stream: It,
}

impl<It> OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	pub fn new(stream: It) -> Self {
		Self { stream }
	}

	fn decode_x(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.x() {
			0 => self.decode_0_z(v, o),
			// 1 => ,
			// 2 => ,
			3 => self.decode_3_z(v, o),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_0_z(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.z() {
			0 => self.decode_0_0_y(v, o),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_0_0_y(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.y() {
			0 => Ok(Opcode::Nop),
			1 => {
				let bytes: [u8; 2] = [self.stream.next().unwrap(), self.stream.next().unwrap()];
				let indirect = Value::Indirect(u16::from_le_bytes(bytes));

				Ok(Opcode::Ld(indirect, Value::Register(Register::SP)))
			}
			2 => Ok(Opcode::Stop),
			3 => Ok(Opcode::JumpR(self.stream.next().unwrap() as i8)),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_3_z(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.z() {
			3 => self.decode_3_3_y(v, o),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_3_3_y(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.y() {
			0 => {
				let bytes: [u8; 2] = [self.stream.next().unwrap(), self.stream.next().unwrap()];
				Ok(Opcode::Jump(u16::from_le_bytes(bytes)))
			}
			_ => Err(Error::UnknownOpcode(v)),
		}
	}
}

#[bitfield]
#[derive(Debug, PartialEq, Eq)]
pub struct OpcodeBits {
	z: B3,
	y: B3,
	x: B2,
}

impl<It> From<It> for OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	fn from(it: It) -> Self {
		Self::new(it)
	}
}

impl<It> Iterator for OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	type Item = Result<Opcode, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.stream.next()?;
		Some(self.decode_x(current, OpcodeBits::from_bytes([current])))
	}
}

#[test]
fn test_convert_opcode() {
	assert_eq!(
		OpcodeGenerator::from(vec![0xc3, 0x50, 0x01].into_iter()).next(),
		Some(Ok(Opcode::Jump(0x150)))
	);
	assert_eq!(
		OpcodeGenerator::from(vec![0x18, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
		Some(Ok(Opcode::JumpR(-24)))
	);
	assert_eq!(
		OpcodeGenerator::from(vec![0x0].into_iter()).next(),
		Some(Ok(Opcode::Nop))
	);
	assert_eq!(
		OpcodeGenerator::from(vec![0x10].into_iter()).next(),
		Some(Ok(Opcode::Stop))
	);
	assert_eq!(
		OpcodeGenerator::from(vec![0x8, 0x34, 0x12].into_iter()).next(),
		Some(Ok(Opcode::Ld(
			Value::Indirect(0x1234),
			Value::Register(Register::SP)
		)))
	);
}
