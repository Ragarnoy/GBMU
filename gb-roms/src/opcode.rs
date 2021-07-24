mod error;
mod register;

use error::Error;
use modular_bitfield::{
	bitfield,
	specifiers::{B2, B3},
};
use register::Register;
use std::{
	convert::{From, TryFrom},
	fmt,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
	/// jump to addr
	Jump(u16),
	/// relative jump to PC + value
	JumpR(i8),
	/// relative jump to PC + value when flag Z is unset
	JumpRNZero(i8),
	/// relative jump to PC + value when flag Z is set
	JumpRZero(i8),
	/// relative jump to PC + value when flag C is unset
	JumpRCarry(i8),
	/// relative jump to PC + value when flag C is set
	JumpRNCarry(i8),

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
			Opcode::JumpRNZero(value) => write!(f, "jrnz {:x}", value),
			Opcode::JumpRZero(value) => write!(f, "jrz {:x}", value),
			Opcode::JumpRNCarry(value) => write!(f, "jrnc {:x}", value),
			Opcode::JumpRCarry(value) => write!(f, "jrc {:x}", value),

			Opcode::Nop => write!(f, "nop"),
			Opcode::Stop => write!(f, "stop"),
			Opcode::Ld(from, to) => write!(f, "ld {}, {}", from, to),
		}
	}
}

#[test]
fn test_display_opcode() {
	use register::RegisterSpecial;

	assert_eq!(Opcode::Jump(0x150).to_string(), "jmp 150");

	assert_eq!(Opcode::JumpR(0x42).to_string(), "jr 42");
	assert_eq!(Opcode::JumpRNZero(0x42).to_string(), "jrnz 42");
	assert_eq!(Opcode::JumpRZero(0x42).to_string(), "jrz 42");
	assert_eq!(Opcode::JumpRNCarry(0x42).to_string(), "jrnc 42");
	assert_eq!(Opcode::JumpRCarry(0x42).to_string(), "jrc 42");

	assert_eq!(Opcode::Nop.to_string(), "nop");
	assert_eq!(Opcode::Stop.to_string(), "stop");
	assert_eq!(
		Opcode::Ld(
			Value::Indirect(0x123),
			Value::Register(RegisterSpecial::SP.into())
		)
		.to_string(),
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
	use register::Register8Bits;

	assert_eq!(Value::Register(Register8Bits::A.into()).to_string(), "A");
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
		let y = o.y();

		match y {
			0 => Ok(Opcode::Nop),
			1 => {
				use register::RegisterSpecial;
				let bytes: [u8; 2] = [self.stream.next().unwrap(), self.stream.next().unwrap()];
				let indirect = Value::Indirect(u16::from_le_bytes(bytes));

				Ok(Opcode::Ld(
					indirect,
					Value::Register(RegisterSpecial::SP.into()),
				))
			}
			2 => Ok(Opcode::Stop),
			3 => Ok(Opcode::JumpR(self.stream.next().unwrap() as i8)),
			4..=7 => {
				let value = self.stream.next().unwrap() as i8;

				match ConditionalTable::try_from(y - 4).expect("jump relative condition") {
					ConditionalTable::NZ => Ok(Opcode::JumpRNZero(value)),
					ConditionalTable::Z => Ok(Opcode::JumpRZero(value)),
					ConditionalTable::NC => Ok(Opcode::JumpRNCarry(value)),
					ConditionalTable::C => Ok(Opcode::JumpRCarry(value)),
				}
			}
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

#[derive(Debug, PartialEq, Eq)]
enum ConditionalTable {
	NZ,
	Z,
	NC,
	C,
}

impl TryFrom<u8> for ConditionalTable {
	type Error = ();

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::NZ),
			1 => Ok(Self::Z),
			2 => Ok(Self::NC),
			3 => Ok(Self::C),
			_ => Err(()),
		}
	}
}

#[test]
fn test_conditional_table_convert() {
	assert_eq!(ConditionalTable::try_from(0), Ok(ConditionalTable::NZ));
	assert_eq!(ConditionalTable::try_from(1), Ok(ConditionalTable::Z));
	assert_eq!(ConditionalTable::try_from(2), Ok(ConditionalTable::NC));
	assert_eq!(ConditionalTable::try_from(3), Ok(ConditionalTable::C));
}

/// Arithmetic / Logic Table
#[derive(Debug, PartialEq, Eq)]
enum ALTable {
	/// Add n to X
	Add,
	/// Add n + Carry to X
	Adc,
	/// Sub n to X
	Sub,
	/// Sub n + Carry to X
	Sbc,
	/// And n with X
	And,
	/// Xor n with X
	Xor,
	/// Or n with X
	Or,
	/// Compare n with X
	/// This is basically X - n where the results are thrown away
	Cmp,
}

impl TryFrom<u8> for ALTable {
	type Error = ();

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Add),
			1 => Ok(Self::Adc),
			2 => Ok(Self::Sub),
			3 => Ok(Self::Sbc),
			4 => Ok(Self::And),
			5 => Ok(Self::Xor),
			6 => Ok(Self::Or),
			7 => Ok(Self::Cmp),
			_ => Err(()),
		}
	}
}

#[test]
fn test_al_table_convert() {
	assert_eq!(ALTable::try_from(0), Ok(ALTable::Add));
	assert_eq!(ALTable::try_from(1), Ok(ALTable::Adc));
	assert_eq!(ALTable::try_from(2), Ok(ALTable::Sub));
	assert_eq!(ALTable::try_from(3), Ok(ALTable::Sbc));
	assert_eq!(ALTable::try_from(4), Ok(ALTable::And));
	assert_eq!(ALTable::try_from(5), Ok(ALTable::Xor));
	assert_eq!(ALTable::try_from(6), Ok(ALTable::Or));
	assert_eq!(ALTable::try_from(7), Ok(ALTable::Cmp));
}

/// Rotation / Shift Table
#[derive(Debug, PartialEq, Eq)]
enum RotTable {
	/// Rotate left
	/// The old 7th bit put in carry
	Rlc,
	/// Rotate right
	/// The old 0th bit put in carry
	Rrc,
	/// Rotate left through Carry flag
	Rl,
	/// Rotate right through carry flag
	Rr,
	/// Shift left into Carry
	/// LSB of n set to 0
	Sla,
	/// Shift right into Carry
	/// MSB doesn't change
	Sra,
	/// Swap upper & lower nibles of n
	Swap,
	/// Shift right into Carry
	/// MSB set to 0
	Srl,
}

impl TryFrom<u8> for RotTable {
	type Error = ();

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(RotTable::Rlc),
			1 => Ok(RotTable::Rrc),
			2 => Ok(RotTable::Rl),
			3 => Ok(RotTable::Rr),
			4 => Ok(RotTable::Sla),
			5 => Ok(RotTable::Sra),
			6 => Ok(RotTable::Swap),
			7 => Ok(RotTable::Srl),
			_ => Err(()),
		}
	}
}

#[test]
fn test_rot_table_convert() {
	assert_eq!(RotTable::try_from(0), Ok(RotTable::Rlc));
	assert_eq!(RotTable::try_from(1), Ok(RotTable::Rrc));
	assert_eq!(RotTable::try_from(2), Ok(RotTable::Rl));
	assert_eq!(RotTable::try_from(3), Ok(RotTable::Rr));
	assert_eq!(RotTable::try_from(4), Ok(RotTable::Sla));
	assert_eq!(RotTable::try_from(5), Ok(RotTable::Sra));
	assert_eq!(RotTable::try_from(6), Ok(RotTable::Swap));
	assert_eq!(RotTable::try_from(7), Ok(RotTable::Srl));
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

#[cfg(test)]
mod test_convert_opcode {
	use super::register::RegisterSpecial;
	use super::{Opcode, OpcodeGenerator, Value};

	#[test]
	fn test_convert_opcode() {
		assert_eq!(
			OpcodeGenerator::from(vec![0xc3, 0x50, 0x01].into_iter()).next(),
			Some(Ok(Opcode::Jump(0x150)))
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
				Value::Register(RegisterSpecial::SP.into())
			)))
		);
	}

	#[test]
	fn test_relative_jump() {
		assert_eq!(
			OpcodeGenerator::from(vec![0x18, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
			Some(Ok(Opcode::JumpR(-24)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x20, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
			Some(Ok(Opcode::JumpRNZero(-24)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x28, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
			Some(Ok(Opcode::JumpRZero(-24)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x30, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
			Some(Ok(Opcode::JumpRNCarry(-24)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x38, (-24_i8).to_le_bytes()[0]].into_iter()).next(),
			Some(Ok(Opcode::JumpRCarry(-24)))
		);
	}
}
