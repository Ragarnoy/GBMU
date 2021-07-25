mod error;
#[macro_use]
mod register;

use error::Error;
use modular_bitfield::{
	bitfield,
	specifiers::{B2, B3},
};
use register::{Register, Register16Bits};
use std::{
	convert::{From, TryFrom},
	fmt,
};

macro_rules! op {
	($t:ident, $($v:expr),+) => {
		Opcode::$t($($v),+)
	};
	($t:ident) => {
		Opcode::$t
	};
}

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

	Add(Store, Value),

	/// load value from **Value** and load it to **Store**
	///
	/// Timing:
	/// - r8 -> r8 : 4
	/// - r8 -> *r16 : 8
	/// - *16 -> r8 : 8
	/// - n -> r8 : 8
	/// - *nn -> r8 : 16
	Ld(Store, Value),
	/// Load value into `*HL` then decrement `HL`
	/// *HL-- = n
	LddFrom(Value),
	/// Load value into `*HL` then increment `HL`
	/// *HL++ = n
	LdiFrom(Value),
	/// Load value from `*HL` store it to `n` then decrement `HL`
	/// n = *HL--
	LddInto(Store),
	/// Load value from `*HL` store it to `n` the increment `HL`
	/// n = *HL++
	LdiInto(Store),
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

			Opcode::Add(s, v) => write!(f, "add {}, {}", s, v),

			Opcode::Ld(from, to) => write!(f, "ld {}, {}", from, to),
			Opcode::LddFrom(v) => write!(f, "ldd (HL), {}", v),
			Opcode::LdiFrom(v) => write!(f, "ldi (HL), {}", v),
			Opcode::LddInto(s) => write!(f, "ldd {}, (HL)", s),
			Opcode::LdiInto(s) => write!(f, "ldi {}, (HL)", s),
		}
	}
}

#[test]
fn test_display_opcode() {
	use register::{Register8Bits, RegisterSpecial};

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
			Store::Indirect(0x123),
			Value::Register(RegisterSpecial::SP.into())
		)
		.to_string(),
		"ld (123), SP"
	);

	assert_eq!(
		Opcode::LddFrom(register8!(A).into()).to_string(),
		"ldd (HL), A"
	);
	assert_eq!(
		Opcode::LdiFrom(register8!(A).into()).to_string(),
		"ldi (HL), A"
	);
	assert_eq!(
		Opcode::LddInto(register8!(A).into()).to_string(),
		"ldd A, (HL)"
	);
	assert_eq!(
		Opcode::LdiInto(register8!(A).into()).to_string(),
		"ldi A, (HL)"
	);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Store {
	/// Register Id
	Register(Register),
	/// Use the addr to register
	IndirectReg(Register16Bits),
	/// Addresse in memory (should be!)
	Indirect(u16),
}

impl From<Register> for Store {
	fn from(r: Register) -> Self {
		Self::Register(r)
	}
}

impl From<u16> for Store {
	fn from(v: u16) -> Self {
		Self::Indirect(v)
	}
}

impl fmt::Display for Store {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Register(reg) => write!(f, "{}", reg),
			Self::IndirectReg(reg) => write!(f, "({})", reg),
			Self::Indirect(addr) => write!(f, "({:x})", addr),
		}
	}
}

#[test]
fn test_store_display() {
	use register::Register8Bits;

	assert_eq!(Store::Register(Register8Bits::A.into()).to_string(), "A");
	assert_eq!(Store::Indirect(0x3a).to_string(), "(3a)");
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
	Register(Register),
	IndirectReg(Register16Bits),
	Indirect(u16),
	Nn(u16),
	N(u8),
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

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Register(reg) => write!(f, "{}", reg),
			Self::IndirectReg(reg) => write!(f, "({})", reg),
			Self::Indirect(adr) => write!(f, "({:x})", adr),
			Self::Nn(v) => write!(f, "{:x}", v),
			Self::N(v) => write!(f, "{:x}", v),
		}
	}
}

#[test]
fn test_value_display() {
	use register::Register8Bits;

	assert_eq!(Value::Register(Register8Bits::A.into()).to_string(), "A");
	assert_eq!(Value::Nn(0x1023_u16).to_string(), "1023");
	assert_eq!(Value::N(0x23_u8).to_string(), "23");
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

	fn get_d(&mut self) -> Option<i8> {
		self.stream.next().map(|r| r as i8)
	}

	fn get_n(&mut self) -> Option<u8> {
		self.stream.next()
	}

	fn get_nn(&mut self) -> Option<u16> {
		let bytes: [u8; 2] = [self.stream.next()?, self.stream.next()?];
		Some(u16::from_le_bytes(bytes))
	}
}

#[bitfield]
#[derive(PartialEq, Eq)]
pub struct OpcodeBits {
	z: B3,
	y: B3,
	x: B2,
}

impl OpcodeBits {
	/// p = y >> 1
	fn p(&self) -> u8 {
		self.y() >> 1
	}

	/// q = y % 2
	fn q(&self) -> u8 {
		self.y() & 1
	}
}

impl fmt::Debug for OpcodeBits {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"OpcodeBits {{ x: {}, y: {}, z: {}, p: {}, q: {} }}",
			self.x(),
			self.y(),
			self.z(),
			self.p(),
			self.q()
		)
	}
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
		use register::{Register16Bits, Register8Bits};

		let current = self.stream.next()?;
		let mut n = || self.get_n().expect("next `n` value");
		let mut nn = || self.get_nn().expect("next `nn` value");

		Some(match current {
			// Ld nn, n
			0x06 => Ok(op!(Ld, register8!(B).into(), n().into())),
			0x0E => Ok(op!(Ld, register8!(C).into(), n().into())),
			0x16 => Ok(op!(Ld, register8!(D).into(), n().into())),
			0x1E => Ok(op!(Ld, register8!(E).into(), n().into())),
			0x26 => Ok(op!(Ld, register8!(H).into(), n().into())),
			0x2E => Ok(op!(Ld, register8!(L).into(), n().into())),

			// Ld r1, r2
			0x7F => Ok(op!(Ld, register8!(A).into(), register8!(A).into())),
			0x78 => Ok(op!(Ld, register8!(A).into(), register8!(B).into())),
			0x79 => Ok(op!(Ld, register8!(A).into(), register8!(C).into())),
			0x7A => Ok(op!(Ld, register8!(A).into(), register8!(D).into())),
			0x7B => Ok(op!(Ld, register8!(A).into(), register8!(E).into())),
			0x7C => Ok(op!(Ld, register8!(A).into(), register8!(H).into())),
			0x7D => Ok(op!(Ld, register8!(A).into(), register8!(L).into())),
			0x7E => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x40 => Ok(op!(Ld, register8!(B).into(), register8!(B).into())),
			0x41 => Ok(op!(Ld, register8!(B).into(), register8!(C).into())),
			0x42 => Ok(op!(Ld, register8!(B).into(), register8!(D).into())),
			0x43 => Ok(op!(Ld, register8!(B).into(), register8!(E).into())),
			0x44 => Ok(op!(Ld, register8!(B).into(), register8!(H).into())),
			0x45 => Ok(op!(Ld, register8!(B).into(), register8!(L).into())),
			0x46 => Ok(op!(
				Ld,
				register8!(B).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x48 => Ok(op!(Ld, register8!(C).into(), register8!(B).into())),
			0x49 => Ok(op!(Ld, register8!(C).into(), register8!(C).into())),
			0x4A => Ok(op!(Ld, register8!(C).into(), register8!(D).into())),
			0x4B => Ok(op!(Ld, register8!(C).into(), register8!(E).into())),
			0x4C => Ok(op!(Ld, register8!(C).into(), register8!(H).into())),
			0x4D => Ok(op!(Ld, register8!(C).into(), register8!(L).into())),
			0x4E => Ok(op!(
				Ld,
				register8!(C).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x50 => Ok(op!(Ld, register8!(D).into(), register8!(B).into())),
			0x51 => Ok(op!(Ld, register8!(D).into(), register8!(C).into())),
			0x52 => Ok(op!(Ld, register8!(D).into(), register8!(D).into())),
			0x53 => Ok(op!(Ld, register8!(D).into(), register8!(E).into())),
			0x54 => Ok(op!(Ld, register8!(D).into(), register8!(H).into())),
			0x55 => Ok(op!(Ld, register8!(D).into(), register8!(L).into())),
			0x56 => Ok(op!(
				Ld,
				register8!(D).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x58 => Ok(op!(Ld, register8!(E).into(), register8!(B).into())),
			0x59 => Ok(op!(Ld, register8!(E).into(), register8!(C).into())),
			0x5A => Ok(op!(Ld, register8!(E).into(), register8!(D).into())),
			0x5B => Ok(op!(Ld, register8!(E).into(), register8!(E).into())),
			0x5C => Ok(op!(Ld, register8!(E).into(), register8!(H).into())),
			0x5D => Ok(op!(Ld, register8!(E).into(), register8!(L).into())),
			0x5E => Ok(op!(
				Ld,
				register8!(E).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x60 => Ok(op!(Ld, register8!(H).into(), register8!(B).into())),
			0x61 => Ok(op!(Ld, register8!(H).into(), register8!(C).into())),
			0x62 => Ok(op!(Ld, register8!(H).into(), register8!(D).into())),
			0x63 => Ok(op!(Ld, register8!(H).into(), register8!(E).into())),
			0x64 => Ok(op!(Ld, register8!(H).into(), register8!(H).into())),
			0x65 => Ok(op!(Ld, register8!(H).into(), register8!(L).into())),
			0x66 => Ok(op!(
				Ld,
				register8!(H).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x68 => Ok(op!(Ld, register8!(L).into(), register8!(B).into())),
			0x69 => Ok(op!(Ld, register8!(L).into(), register8!(C).into())),
			0x6A => Ok(op!(Ld, register8!(L).into(), register8!(D).into())),
			0x6B => Ok(op!(Ld, register8!(L).into(), register8!(E).into())),
			0x6C => Ok(op!(Ld, register8!(L).into(), register8!(H).into())),
			0x6D => Ok(op!(Ld, register8!(L).into(), register8!(L).into())),
			0x6E => Ok(op!(
				Ld,
				register8!(L).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0x70 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(B).into()
			)),
			0x71 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(C).into()
			)),
			0x72 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(D).into()
			)),
			0x73 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(E).into()
			)),
			0x74 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(H).into()
			)),
			0x75 => Ok(op!(
				Ld,
				Store::IndirectReg(register16!(HL)),
				register8!(L).into()
			)),
			0x36 => Ok(op!(Ld, Store::IndirectReg(register16!(HL)), n().into())),

			// LD A, n
			0x7F => Ok(op!(Ld, register8!(A).into(), register8!(A).into())),
			0x78 => Ok(op!(Ld, register8!(A).into(), register8!(B).into())),
			0x79 => Ok(op!(Ld, register8!(A).into(), register8!(C).into())),
			0x7A => Ok(op!(Ld, register8!(A).into(), register8!(D).into())),
			0x7B => Ok(op!(Ld, register8!(A).into(), register8!(E).into())),
			0x7C => Ok(op!(Ld, register8!(A).into(), register8!(H).into())),
			0x7D => Ok(op!(Ld, register8!(A).into(), register8!(L).into())),
			0x0A => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg(register16!(BC))
			)),
			0x1A => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg(register16!(DE))
			)),
			0x7E => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg(register16!(HL))
			)),
			0xFA => Ok(op!(Ld, register8!(A).into(), Value::Indirect(nn()))),
			0x3E => Ok(op!(Ld, register8!(A).into(), n().into())),

			_ => Err(Error::UnknownOpcode(current)),
		})
	}
}

#[cfg(test)]
mod test_convert_opcode {
	use super::register::{self, Register};
	use super::{Opcode, OpcodeGenerator, Store, Value};

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
	}

	#[test]
	fn test_ld() {
		use register::{Register16Bits, RegisterSpecial};

		assert_eq!(
			OpcodeGenerator::from(vec![0x8, 0x34, 0x12].into_iter()).next(),
			Some(Ok(Opcode::Ld(
				Store::Indirect(0x1234),
				Value::Register(RegisterSpecial::SP.into())
			)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x11, 0x50, 0x01].into_iter()).next(),
			Some(Ok(Opcode::Ld(
				Register::from(Register16Bits::DE).into(),
				Value::Nn(0x150)
			)))
		);
	}

	#[test]
	fn test_ldi_ldd() {
		use register::{Register, Register8Bits};

		assert_eq!(
			OpcodeGenerator::from(vec![0x2a].into_iter()).next(),
			Some(Ok(Opcode::LdiInto(register8!(A).into())))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x3a].into_iter()).next(),
			Some(Ok(Opcode::LddInto(register8!(A).into())))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x22].into_iter()).next(),
			Some(Ok(Opcode::LdiFrom(register8!(A).into())))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x32].into_iter()).next(),
			Some(Ok(Opcode::LddFrom(register8!(A).into())))
		);
	}

	#[test]
	fn test_add() {
		use register::{Register16Bits, RegisterSpecial};

		assert_eq!(
			OpcodeGenerator::from(vec![0x39].into_iter()).next(),
			Some(Ok(Opcode::Add(
				Register::from(Register16Bits::HL).into(),
				Register::from(RegisterSpecial::SP).into()
			)))
		)
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
