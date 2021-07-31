mod error;
#[macro_use]
mod register;

use error::Error;
use modular_bitfield::{
	bitfield,
	specifiers::{B2, B3},
};
use register::{Register, Register16Bits as Reg16, Register8Bits as Reg8};
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
	/// Timing:
	/// - u16: 12
	/// - *HL: 4
	Jump(Value),

	/// jump to addr when zero flag is set
	/// Timing: 12
	JumpZero(u16),

	/// jump to addr when zero flag is not set
	/// Timing: 12
	JumpNZero(u16),

	/// jump to addr when carry flag is set
	/// Timing: 12
	JumpCarry(u16),

	/// jump to addr when carry flag is not set
	/// Timing: 12
	JumpNCarry(u16),

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

	/// No operation
	/// Timing: 4
	Nop,

	/// Power down CPU until an interrupt occurs.
	/// Timing: 4
	Halt,

	/// Halt CPU & LCD display until button pressed
	/// Timing: 4
	Stop,

	/// load value from **Value** and load it to **Store**
	///
	/// Timing:
	/// - r8 -> r8 : 4
	/// - r8 -> *r16 : 8
	/// - *16 -> r8 : 8
	/// - n -> r8 : 8
	/// - *nn -> r8 : 16
	/// - nn -> r16 : 12
	/// - r16 -> r16 : 8
	/// - *nn -> SP : 20
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
	/// Ldh put *(0xff00 + n) in A
	/// Timing: 12
	LdhFrom(u8),
	/// Ldh put A into *(0xff00 + n)
	/// Timing: 12
	LdhInto(u8),
	/// ldhl put SP + n in HL
	/// Timing: 12
	Ldhl(i8),

	/// Push reg16 onto stack
	/// dec SP twice
	/// Timing: 16
	Push(Reg16),
	/// Pop u16 from stack
	/// inc SP twice
	/// Timing: 12
	Pop(Reg16),

	// Timing for alu op:
	// - r8 + r8 : 4
	// - r8 + *r16 : 8
	// - r8 + n : 8
	// - r16 + d : 16
	/// Add value to *S*
	Add(Store, Value),
	/// Add value + carry to A
	Adc(Value),
	/// Sub value to A
	Sub(Value),
	/// Sub value + carry to A
	Sbc(Value),
	/// Logic And with A : `A = A & n`
	And(Value),
	/// Logic Or with A : `A = A | n`
	Or(Value),
	/// Logic Xor with A: `A = A ^ n`
	Xor(Value),
	/// Logic compare with A: A == n ?
	Cp(Value),

	// Timing for inc/dec:
	// - r8: 4
	// - *HL: 12
	/// Increment n
	Inc(Store),
	/// Decrement n
	Dec(Store),

	/// Swap upper & lower nibles of n
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Swap(Store),

	/// Decimal adjust register A
	/// Adjust register A to obtain a Binary Coded Decimal (BCD)
	/// - 42 : 0b0010_1010 => `BCD(42) = 0b0100_0010`
	/// Timing: 4
	Daa,

	/// Complement a register (flip all bits)
	/// `0b0011_0101` => `0b1100_1010`
	/// Timing: 4
	Cpl,

	/// Complement carry flag (toggle carry flag)
	/// - On => Off
	/// - Off => On
	/// Timing: 4
	Ccf,

	/// Set carry flag
	/// Timing: 4
	Scf,

	/// Disable Interrupts after next instruction
	/// Timimg: 4
	Di,

	/// Enable Interrupts after next instruction
	/// Timing: 4
	Ei,

	/// Rotate A left
	/// Timing: 4
	Rlca,

	/// Rotate A left
	/// Timing: 4
	Rla,

	/// Rotate A right
	/// Timing: 4
	Rrca,

	/// Rotate A right
	/// Timimg: 4
	Rra,

	/// Rotate n left
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Rlc(Store),

	/// Rotate n left
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Rl(Store),

	/// Rotate n right
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Rrc(Store),

	/// Rotate n right
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Rr(Store),

	/// Shift n left into Carry,
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Sla(Store),

	/// Shift n right into carry,
	/// Msb doesn't change
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Sra(Store),

	/// Shift n right into carry
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Srl(Store),

	/// Test bit b in register r
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Bit(u8, Store),

	/// Set bit b in register r
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Set(u8, Store),

	/// Reset bit b in register r
	/// Timing:
	/// - r8: 8
	/// - *HL: 16
	Res(u8, Store),

	/// Push addr of next instruction onto stack and then jump to address nn
	/// Timing: 12
	Call(u16),

	/// Push addr of next instruction onto stack and then jump to address nn
	/// when zero flag is set
	/// Timing: 12
	CallZero(u16),

	/// Push addr of next instruction onto stack and then jump to address nn
	/// when zero flag is not set
	/// Timing: 12
	CallNZero(u16),

	/// Push addr of next instruction onto stack and then jump to address nn
	/// when carry flag is set
	/// Timing: 12
	CallCarry(u16),

	/// Push addr of next instruction onto stack and then jump to address nn
	/// when carry flag is not set
	/// Timing: 12
	CallNCarry(u16),

	/// Push present addr onto stack
	/// Then jump to addr n
	/// Timing: 32
	Restart(u8),

	/// Pop u16 from stack & jump to that addr
	/// Timing: 8
	Return,

	/// Pop u16 from stack & jump to that addr
	/// Then enable interrupts
	/// Timing: 8
	ReturnI,

	/// When zero flag is set
	/// Pop u16 from stack & jump to that addr
	/// Timing: 8
	ReturnZero,

	/// When zero flag is not set
	/// Pop u16 from stack & jump to that addr
	/// Timing: 8
	ReturnNZero,

	/// When carry flag is set
	/// Pop u16 from stack & jump to that addr
	/// Timing: 8
	ReturnCarry,

	/// When carry flag is not set
	/// Pop u16 from stack & jump to that addr
	/// Timing: 8
	ReturnNCarry,
}

impl fmt::Display for Opcode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Jump(addr) => write!(f, "jp {}", addr),
			Self::JumpZero(addr) => write!(f, "jpz {}", addr),
			Self::JumpNZero(addr) => write!(f, "jpnz {}", addr),
			Self::JumpCarry(addr) => write!(f, "jpc {}", addr),
			Self::JumpNCarry(addr) => write!(f, "jpnc {}", addr),

			Self::JumpR(value) => write!(f, "jr {:x}", value),
			Self::JumpRNZero(value) => write!(f, "jrnz {:x}", value),
			Self::JumpRZero(value) => write!(f, "jrz {:x}", value),
			Self::JumpRNCarry(value) => write!(f, "jrnc {:x}", value),
			Self::JumpRCarry(value) => write!(f, "jrc {:x}", value),

			Self::Nop => write!(f, "nop"),
			Self::Halt => write!(f, "halt"),
			Self::Stop => write!(f, "stop"),

			Self::Ld(from, to) => write!(f, "ld {}, {}", from, to),
			Self::LddFrom(v) => write!(f, "ldd (HL), {}", v),
			Self::LdiFrom(v) => write!(f, "ldi (HL), {}", v),
			Self::LddInto(s) => write!(f, "ldd {}, (HL)", s),
			Self::LdiInto(s) => write!(f, "ldi {}, (HL)", s),
			Self::LdhFrom(v) => write!(f, "ldh A, (0xff00 + {})", v),
			Self::LdhInto(s) => write!(f, "ldh (0xff00 + {}), A", s),
			Self::Ldhl(addr) => write!(f, "ldhl SP, {}", addr),

			Self::Push(reg) => write!(f, "push {}", reg),
			Self::Pop(reg) => write!(f, "pop {}", reg),

			Self::Add(s, v) => write!(f, "add {}, {}", s, v),
			Self::Adc(v) => write!(f, "adc A, {}", v),
			Self::Sub(v) => write!(f, "sub A, {}", v),
			Self::Sbc(v) => write!(f, "sbc A, {}", v),
			Self::And(v) => write!(f, "and A, {}", v),
			Self::Or(v) => write!(f, "or A, {}", v),
			Self::Xor(v) => write!(f, "xor A, {}", v),
			Self::Cp(v) => write!(f, "cp A, {}", v),

			Self::Inc(s) => write!(f, "inc {}", s),
			Self::Dec(s) => write!(f, "dec {}", s),

			Self::Swap(s) => write!(f, "swap {}", s),

			Self::Daa => write!(f, "daa"),
			Self::Cpl => write!(f, "cpl"),
			Self::Ccf => write!(f, "ccf"),
			Self::Scf => write!(f, "scf"),

			Self::Di => write!(f, "di"),
			Self::Ei => write!(f, "ei"),

			Self::Rlca => write!(f, "rlca"),
			Self::Rla => write!(f, "rla"),

			Self::Rrca => write!(f, "rrca"),
			Self::Rra => write!(f, "rra"),

			Self::Rlc(n) => write!(f, "rlc {}", n),
			Self::Rl(n) => write!(f, "rl {}", n),
			Self::Rrc(n) => write!(f, "rrc {}", n),
			Self::Rr(n) => write!(f, "rr {}", n),

			Self::Sla(n) => write!(f, "sla {}", n),
			Self::Sra(n) => write!(f, "sra {}", n),
			Self::Srl(n) => write!(f, "srl {}", n),

			Self::Bit(b, r) => write!(f, "bit {}, {}", b, r),
			Self::Set(b, r) => write!(f, "set {}, {}", b, r),
			Self::Res(b, r) => write!(f, "res {}, {}", b, r),

			Self::Call(addr) => write!(f, "call {:x}", addr),
			Self::CallZero(addr) => write!(f, "callz {:x}", addr),
			Self::CallNZero(addr) => write!(f, "callnz {:x}", addr),
			Self::CallCarry(addr) => write!(f, "callc {:x}", addr),
			Self::CallNCarry(addr) => write!(f, "callnc {:x}", addr),

			Self::Restart(addr) => write!(f, "rst {:x}", addr),

			Self::Return => write!(f, "ret"),
			Self::ReturnI => write!(f, "reti"),
			Self::ReturnZero => write!(f, "retz"),
			Self::ReturnNZero => write!(f, "retnz"),
			Self::ReturnCarry => write!(f, "retc"),
			Self::ReturnNCarry => write!(f, "retnc"),
		}
	}
}

#[test]
fn test_display_opcode() {
	use register::{Register8Bits, RegisterSpecial};

	assert_eq!(Opcode::Jump(0x150_u16.into()).to_string(), "jp 150");

	assert_eq!(Opcode::JumpR(0x42).to_string(), "jr 42");
	assert_eq!(Opcode::JumpRNZero(0x42).to_string(), "jrnz 42");
	assert_eq!(Opcode::JumpRZero(0x42).to_string(), "jrz 42");
	assert_eq!(Opcode::JumpRNCarry(0x42).to_string(), "jrnc 42");
	assert_eq!(Opcode::JumpRCarry(0x42).to_string(), "jrc 42");

	assert_eq!(Opcode::Nop.to_string(), "nop");
	assert_eq!(Opcode::Stop.to_string(), "stop");
	assert_eq!(
		Opcode::Ld(
			Store::Indirect16(0x123),
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
	IndirectReg16(Reg16),
	/// Use the addr that result of `addr = Reg + 0xff00`
	IndierectReg8(Reg8),
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
	fn from_r_table(v: u8) -> Option<Store> {
		use register::Register8Bits;

		match v {
			0 => Some(register8!(B).into()),
			1 => Some(register8!(C).into()),
			2 => Some(register8!(D).into()),
			3 => Some(register8!(E).into()),
			4 => Some(register8!(H).into()),
			5 => Some(register8!(L).into()),
			6 => Some(Store::IndirectReg16(Reg16::HL)),
			7 => Some(register8!(A).into()),
			_ => None,
		}
	}
}

#[test]
fn test_store_display() {
	use register::Register8Bits;

	assert_eq!(Store::Register(Register8Bits::A.into()).to_string(), "A");
	assert_eq!(Store::Indirect16(0x3a).to_string(), "(3a)");
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
	Register(Register),
	IndirectReg16(Reg16),
	/// Use the addr that result of `addr = Reg + 0xff00`
	IndirectReg8(Reg8),
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

	fn read_d(&mut self) -> Option<i8> {
		self.stream.next().map(|r| r as i8)
	}

	fn read_n(&mut self) -> Option<u8> {
		self.stream.next()
	}

	fn read_nn(&mut self) -> Option<u16> {
		let bytes: [u8; 2] = [self.stream.next()?, self.stream.next()?];
		Some(u16::from_le_bytes(bytes))
	}

	fn get_d(&mut self) -> i8 {
		self.read_d().expect("next i8")
	}

	fn get_n(&mut self) -> u8 {
		self.read_n().expect("next n")
	}

	fn get_nn(&mut self) -> u16 {
		self.read_nn().expect("next nn")
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

impl<It> OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	fn decode_cb_prefix(&mut self) -> Result<Opcode, Error> {
		use register::Register8Bits;

		let current = self
			.stream
			.next()
			.ok_or_else(|| Error::InvalideOpcode(0xCB))?;

		match current {
			// swap n
			0x37 => Ok(op!(Swap, register8!(A).into())),
			0x30 => Ok(op!(Swap, register8!(B).into())),
			0x31 => Ok(op!(Swap, register8!(C).into())),
			0x32 => Ok(op!(Swap, register8!(D).into())),
			0x33 => Ok(op!(Swap, register8!(E).into())),
			0x34 => Ok(op!(Swap, register8!(H).into())),
			0x35 => Ok(op!(Swap, register8!(L).into())),
			0x36 => Ok(op!(Swap, Store::IndirectReg16(Reg16::HL))),

			// rlc n
			0x07 => Ok(op!(Rlc, register8!(A).into())),
			0x00 => Ok(op!(Rlc, register8!(B).into())),
			0x01 => Ok(op!(Rlc, register8!(C).into())),
			0x02 => Ok(op!(Rlc, register8!(D).into())),
			0x03 => Ok(op!(Rlc, register8!(E).into())),
			0x04 => Ok(op!(Rlc, register8!(H).into())),
			0x05 => Ok(op!(Rlc, register8!(L).into())),
			0x06 => Ok(op!(Rlc, Store::IndirectReg16(Reg16::HL))),

			// rl n
			0x17 => Ok(op!(Rl, register8!(A).into())),
			0x10 => Ok(op!(Rl, register8!(B).into())),
			0x11 => Ok(op!(Rl, register8!(C).into())),
			0x12 => Ok(op!(Rl, register8!(D).into())),
			0x13 => Ok(op!(Rl, register8!(E).into())),
			0x14 => Ok(op!(Rl, register8!(H).into())),
			0x15 => Ok(op!(Rl, register8!(L).into())),
			0x16 => Ok(op!(Rl, Store::IndirectReg16(Reg16::HL))),

			// rrc n
			0x0F => Ok(op!(Rrc, register8!(A).into())),
			0x08 => Ok(op!(Rrc, register8!(B).into())),
			0x09 => Ok(op!(Rrc, register8!(C).into())),
			0x0A => Ok(op!(Rrc, register8!(D).into())),
			0x0B => Ok(op!(Rrc, register8!(E).into())),
			0x0C => Ok(op!(Rrc, register8!(H).into())),
			0x0D => Ok(op!(Rrc, register8!(L).into())),
			0x0E => Ok(op!(Rrc, Store::IndirectReg16(Reg16::HL))),

			// rr n
			0x1F => Ok(op!(Rr, register8!(A).into())),
			0x18 => Ok(op!(Rr, register8!(B).into())),
			0x19 => Ok(op!(Rr, register8!(C).into())),
			0x1A => Ok(op!(Rr, register8!(D).into())),
			0x1B => Ok(op!(Rr, register8!(E).into())),
			0x1C => Ok(op!(Rr, register8!(H).into())),
			0x1D => Ok(op!(Rr, register8!(L).into())),
			0x1E => Ok(op!(Rr, Store::IndirectReg16(Reg16::HL))),

			// sla n
			0x27 => Ok(op!(Sla, register8!(A).into())),
			0x20 => Ok(op!(Sla, register8!(B).into())),
			0x21 => Ok(op!(Sla, register8!(C).into())),
			0x22 => Ok(op!(Sla, register8!(D).into())),
			0x23 => Ok(op!(Sla, register8!(E).into())),
			0x24 => Ok(op!(Sla, register8!(H).into())),
			0x25 => Ok(op!(Sla, register8!(L).into())),
			0x26 => Ok(op!(Sla, Store::IndirectReg16(Reg16::HL))),

			// sra n
			0x2F => Ok(op!(Sra, register8!(A).into())),
			0x28 => Ok(op!(Sra, register8!(B).into())),
			0x29 => Ok(op!(Sra, register8!(C).into())),
			0x2A => Ok(op!(Sra, register8!(D).into())),
			0x2B => Ok(op!(Sra, register8!(E).into())),
			0x2C => Ok(op!(Sra, register8!(H).into())),
			0x2D => Ok(op!(Sra, register8!(L).into())),
			0x2E => Ok(op!(Sra, Store::IndirectReg16(Reg16::HL))),

			// srl n
			0x3F => Ok(op!(Srl, register8!(A).into())),
			0x38 => Ok(op!(Srl, register8!(B).into())),
			0x39 => Ok(op!(Srl, register8!(C).into())),
			0x3A => Ok(op!(Srl, register8!(D).into())),
			0x3B => Ok(op!(Srl, register8!(E).into())),
			0x3C => Ok(op!(Srl, register8!(H).into())),
			0x3D => Ok(op!(Srl, register8!(L).into())),
			0x3E => Ok(op!(Srl, Store::IndirectReg16(Reg16::HL))),

			_ => self.decode_bits_command(current),
		}
	}

	fn decode_bits_command(&mut self, cmd: u8) -> Result<Opcode, Error> {
		let bits = OpcodeBits::from_bytes([cmd]);
		let bit = bits.y();
		let reg = Store::from_r_table(bits.z()).expect("expected a valid register from r table");

		match bits.x() {
			// bit b,n
			0x01 => Ok(op!(Bit, bit, reg)),
			// set b,n
			0x02 => Ok(op!(Set, bit, reg)),
			// res b,n
			0x03 => Ok(op!(Res, bit, reg)),
			_ => Err(Error::UnknownOpcode(cmd)),
		}
	}
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
		use register::{Register16Bits, Register8Bits, RegisterSpecial};

		let current = self.stream.next()?;

		Some(match current {
			// Ld nn, n
			0x06 => Ok(op!(Ld, register8!(B).into(), self.get_n().into())),
			0x0E => Ok(op!(Ld, register8!(C).into(), self.get_n().into())),
			0x16 => Ok(op!(Ld, register8!(D).into(), self.get_n().into())),
			0x1E => Ok(op!(Ld, register8!(E).into(), self.get_n().into())),
			0x26 => Ok(op!(Ld, register8!(H).into(), self.get_n().into())),
			0x2E => Ok(op!(Ld, register8!(L).into(), self.get_n().into())),

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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
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
				Value::IndirectReg16(Reg16::HL)
			)),
			0x70 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(B).into()
			)),
			0x71 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(C).into()
			)),
			0x72 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(D).into()
			)),
			0x73 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(E).into()
			)),
			0x74 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(H).into()
			)),
			0x75 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				register8!(L).into()
			)),
			0x36 => Ok(op!(
				Ld,
				Store::IndirectReg16(Reg16::HL),
				self.get_n().into()
			)),

			// LD A, n
			0x0A => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg16(Reg16::BC)
			)),
			0x1A => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::IndirectReg16(Reg16::DE)
			)),
			0xFA => Ok(op!(
				Ld,
				register8!(A).into(),
				Value::Indirect16(self.get_nn())
			)),
			0x3E => Ok(op!(Ld, register8!(A).into(), self.get_n().into())),

			0xF2 => Ok(op!(Ld, register8!(A).into(), Value::IndirectReg8(Reg8::C))),
			0xE2 => Ok(op!(Ld, Store::IndierectReg8(Reg8::C), register8!(A).into())),

			0x32 => Ok(op!(LddFrom, register8!(A).into())),
			0x3A => Ok(op!(LddInto, register8!(A).into())),

			0x22 => Ok(op!(LdiFrom, register8!(A).into())),
			0x2A => Ok(op!(LdiInto, register8!(A).into())),

			// ldh (n), A
			0xE0 => Ok(op!(LdhInto, self.get_n())),

			// ldh A, (n)
			0xF0 => Ok(op!(LdhFrom, self.get_n())),

			// ld n, nn
			0x01 => Ok(op!(Ld, register16!(BC).into(), self.get_nn().into())),
			0x11 => Ok(op!(Ld, register16!(DE).into(), self.get_nn().into())),
			0x21 => Ok(op!(Ld, register16!(HL).into(), self.get_nn().into())),
			0x31 => Ok(op!(Ld, register_special!(SP).into(), self.get_nn().into())),

			// ld sp, hl
			0xF9 => Ok(op!(
				Ld,
				register_special!(SP).into(),
				register16!(HL).into()
			)),

			// ldhl sp, n
			0xF8 => Ok(op!(Ldhl, self.get_d())),

			// ld (nn), SP
			0x08 => Ok(op!(Ld, self.get_nn().into(), register_special!(SP).into())),

			// push r16
			0xF5 => Ok(op!(Push, Reg16::AF)),
			0xC5 => Ok(op!(Push, Reg16::BC)),
			0xD5 => Ok(op!(Push, Reg16::DE)),
			0xE5 => Ok(op!(Push, Reg16::HL)),

			// pop r16
			0xF1 => Ok(op!(Pop, Reg16::AF)),
			0xC1 => Ok(op!(Pop, Reg16::BC)),
			0xD1 => Ok(op!(Pop, Reg16::DE)),
			0xE1 => Ok(op!(Pop, Reg16::HL)),

			// add A, n
			0x87 => Ok(op!(Add, register8!(A).into(), register8!(A).into())),
			0x80 => Ok(op!(Add, register8!(A).into(), register8!(B).into())),
			0x81 => Ok(op!(Add, register8!(A).into(), register8!(C).into())),
			0x82 => Ok(op!(Add, register8!(A).into(), register8!(D).into())),
			0x83 => Ok(op!(Add, register8!(A).into(), register8!(E).into())),
			0x84 => Ok(op!(Add, register8!(A).into(), register8!(H).into())),
			0x85 => Ok(op!(Add, register8!(A).into(), register8!(L).into())),
			0x86 => Ok(op!(
				Add,
				register8!(A).into(),
				Value::IndirectReg16(Reg16::HL)
			)),
			0xC6 => Ok(op!(Add, register8!(A).into(), self.get_n().into())),

			// adc A, n
			0x8F => Ok(op!(Adc, register8!(A).into())),
			0x88 => Ok(op!(Adc, register8!(B).into())),
			0x89 => Ok(op!(Adc, register8!(C).into())),
			0x8A => Ok(op!(Adc, register8!(D).into())),
			0x8B => Ok(op!(Adc, register8!(E).into())),
			0x8C => Ok(op!(Adc, register8!(H).into())),
			0x8D => Ok(op!(Adc, register8!(L).into())),
			0x8E => Ok(op!(Adc, Value::IndirectReg16(Reg16::HL))),
			0xCE => Ok(op!(Adc, self.get_n().into())),

			// sub A, n
			0x97 => Ok(op!(Sub, register8!(A).into())),
			0x90 => Ok(op!(Sub, register8!(B).into())),
			0x91 => Ok(op!(Sub, register8!(C).into())),
			0x92 => Ok(op!(Sub, register8!(D).into())),
			0x93 => Ok(op!(Sub, register8!(E).into())),
			0x94 => Ok(op!(Sub, register8!(H).into())),
			0x95 => Ok(op!(Sub, register8!(L).into())),
			0x96 => Ok(op!(Sub, Value::IndirectReg16(Reg16::HL))),
			0xD6 => Ok(op!(Sub, self.get_n().into())),

			// sbc A, n
			0x9F => Ok(op!(Sbc, register8!(A).into())),
			0x98 => Ok(op!(Sbc, register8!(B).into())),
			0x99 => Ok(op!(Sbc, register8!(C).into())),
			0x9A => Ok(op!(Sbc, register8!(D).into())),
			0x9B => Ok(op!(Sbc, register8!(E).into())),
			0x9C => Ok(op!(Sbc, register8!(H).into())),
			0x9D => Ok(op!(Sbc, register8!(L).into())),
			0x9E => Ok(op!(Sbc, Value::IndirectReg16(Reg16::HL))),
			0xDE => Ok(op!(Sbc, self.get_n().into())),

			// and A, n
			0xA7 => Ok(op!(And, register8!(A).into())),
			0xA0 => Ok(op!(And, register8!(B).into())),
			0xA1 => Ok(op!(And, register8!(C).into())),
			0xA2 => Ok(op!(And, register8!(D).into())),
			0xA3 => Ok(op!(And, register8!(E).into())),
			0xA4 => Ok(op!(And, register8!(H).into())),
			0xA5 => Ok(op!(And, register8!(L).into())),
			0xA6 => Ok(op!(And, Value::IndirectReg16(Reg16::HL))),
			0xE6 => Ok(op!(And, self.get_n().into())),

			// or A, n
			0xB7 => Ok(op!(Or, register8!(A).into())),
			0xB0 => Ok(op!(Or, register8!(B).into())),
			0xB1 => Ok(op!(Or, register8!(C).into())),
			0xB2 => Ok(op!(Or, register8!(D).into())),
			0xB3 => Ok(op!(Or, register8!(E).into())),
			0xB4 => Ok(op!(Or, register8!(H).into())),
			0xB5 => Ok(op!(Or, register8!(L).into())),
			0xB6 => Ok(op!(Or, Value::IndirectReg16(Reg16::HL))),
			0xF6 => Ok(op!(Or, self.get_n().into())),

			// xor A, n
			0xAF => Ok(op!(Xor, register8!(A).into())),
			0xA8 => Ok(op!(Xor, register8!(B).into())),
			0xA9 => Ok(op!(Xor, register8!(C).into())),
			0xAA => Ok(op!(Xor, register8!(D).into())),
			0xAB => Ok(op!(Xor, register8!(E).into())),
			0xAC => Ok(op!(Xor, register8!(H).into())),
			0xAD => Ok(op!(Xor, register8!(L).into())),
			0xAE => Ok(op!(Xor, Value::IndirectReg16(Reg16::HL))),
			0xEE => Ok(op!(Xor, self.get_n().into())),

			// cp A, n
			0xBF => Ok(op!(Cp, register8!(A).into())),
			0xB8 => Ok(op!(Cp, register8!(B).into())),
			0xB9 => Ok(op!(Cp, register8!(C).into())),
			0xBA => Ok(op!(Cp, register8!(D).into())),
			0xBB => Ok(op!(Cp, register8!(E).into())),
			0xBC => Ok(op!(Cp, register8!(H).into())),
			0xBD => Ok(op!(Cp, register8!(L).into())),
			0xBE => Ok(op!(Cp, Value::IndirectReg16(Reg16::HL))),
			0xFE => Ok(op!(Cp, self.get_n().into())),

			// inc n
			0x3C => Ok(op!(Inc, register8!(A).into())),
			0x04 => Ok(op!(Inc, register8!(B).into())),
			0x0C => Ok(op!(Inc, register8!(C).into())),
			0x14 => Ok(op!(Inc, register8!(D).into())),
			0x1C => Ok(op!(Inc, register8!(E).into())),
			0x24 => Ok(op!(Inc, register8!(H).into())),
			0x2C => Ok(op!(Inc, register8!(L).into())),
			0x34 => Ok(op!(Inc, Store::IndirectReg16(Reg16::HL))),

			// dec n
			0x3D => Ok(op!(Dec, register8!(A).into())),
			0x05 => Ok(op!(Dec, register8!(B).into())),
			0x0D => Ok(op!(Dec, register8!(C).into())),
			0x15 => Ok(op!(Dec, register8!(D).into())),
			0x1D => Ok(op!(Dec, register8!(E).into())),
			0x25 => Ok(op!(Dec, register8!(H).into())),
			0x2D => Ok(op!(Dec, register8!(L).into())),
			0x35 => Ok(op!(Dec, Store::IndirectReg16(Reg16::HL))),

			// add hl, n
			0x09 => Ok(op!(Add, register16!(HL).into(), register16!(BC).into())),
			0x19 => Ok(op!(Add, register16!(HL).into(), register16!(DE).into())),
			0x29 => Ok(op!(Add, register16!(HL).into(), register16!(HL).into())),
			0x39 => Ok(op!(
				Add,
				register16!(HL).into(),
				register_special!(SP).into()
			)),

			// add sp, d
			0xE8 => Ok(op!(Add, register_special!(SP).into(), self.get_d().into())),

			// inc nn
			0x03 => Ok(op!(Inc, register16!(BC).into())),
			0x13 => Ok(op!(Inc, register16!(DE).into())),
			0x23 => Ok(op!(Inc, register16!(HL).into())),
			0x33 => Ok(op!(Inc, register_special!(SP).into())),

			// dec nn
			0x0B => Ok(op!(Dec, register16!(BC).into())),
			0x1B => Ok(op!(Dec, register16!(DE).into())),
			0x2B => Ok(op!(Dec, register16!(HL).into())),
			0x3B => Ok(op!(Dec, register_special!(SP).into())),

			0xCB => self.decode_cb_prefix(),

			0x27 => Ok(op!(Daa)),
			0x2F => Ok(op!(Cpl)),
			0x3F => Ok(op!(Ccf)),
			0x37 => Ok(op!(Scf)),

			0x00 => Ok(op!(Nop)),
			0x76 => Ok(op!(Halt)),
			0x10 => {
				if self.stream.next() == Some(0x00) {
					Ok(op!(Stop))
				} else {
					Err(Error::InvalideOpcode(0x10))
				}
			}

			0xF3 => Ok(op!(Di)),
			0xFB => Ok(op!(Ei)),

			0x07 => Ok(op!(Rlca)),
			0x17 => Ok(op!(Rla)),

			0x0F => Ok(op!(Rrca)),
			0x1F => Ok(op!(Rra)),

			// jp nn
			0xC3 => Ok(op!(Jump, self.get_nn().into())),

			// jp cc,nn
			0xC2 => Ok(op!(JumpNZero, self.get_nn().into())),
			0xCA => Ok(op!(JumpZero, self.get_nn().into())),
			0xD2 => Ok(op!(JumpNCarry, self.get_nn().into())),
			0xDA => Ok(op!(JumpCarry, self.get_nn().into())),

			// jp (hl)
			0xE9 => Ok(op!(Jump, Value::IndirectReg16(Reg16::HL))),

			// jr d
			0x18 => Ok(op!(JumpR, self.get_d())),

			// jr cc,d
			0x20 => Ok(op!(JumpRNZero, self.get_d())),
			0x28 => Ok(op!(JumpRZero, self.get_d())),
			0x30 => Ok(op!(JumpRNCarry, self.get_d())),
			0x38 => Ok(op!(JumpRCarry, self.get_d())),

			// call nn
			0xCD => Ok(op!(Call, self.get_nn())),

			// call cc, nn
			0xC4 => Ok(op!(CallNZero, self.get_nn())),
			0xCC => Ok(op!(CallZero, self.get_nn())),
			0xD4 => Ok(op!(CallNCarry, self.get_nn())),
			0xDC => Ok(op!(CallCarry, self.get_nn())),

			// rst n
			0xC7 => Ok(op!(Restart, 0x00)),
			0xCF => Ok(op!(Restart, 0x08)),
			0xD7 => Ok(op!(Restart, 0x10)),
			0xDF => Ok(op!(Restart, 0x18)),
			0xE7 => Ok(op!(Restart, 0x20)),
			0xEF => Ok(op!(Restart, 0x28)),
			0xF7 => Ok(op!(Restart, 0x30)),
			0xFF => Ok(op!(Restart, 0x38)),

			// ret
			0xC9 => Ok(op!(Return)),

			// ret cc
			0xC0 => Ok(op!(ReturnNZero)),
			0xC8 => Ok(op!(ReturnZero)),
			0xD0 => Ok(op!(ReturnNCarry)),
			0xD8 => Ok(op!(ReturnCarry)),

			// reti
			0xD9 => Ok(op!(ReturnI)),

			_ => Err(Error::UnknownOpcode(current)),
		})
	}
}

#[cfg(test)]
mod test_convert_opcode {
	use super::register::{self, Register};
	use super::{Opcode, OpcodeGenerator, Reg16, Store, Value};

	#[test]
	fn test_convert_opcode() {
		assert_eq!(
			OpcodeGenerator::from(vec![0xc3, 0x50, 0x01].into_iter()).next(),
			Some(Ok(op!(Jump, 0x150_u16.into())))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x0].into_iter()).next(),
			Some(Ok(op!(Nop)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x10, 0x00].into_iter()).next(),
			Some(Ok(op!(Stop)))
		);
	}

	#[test]
	fn test_ld() {
		use register::RegisterSpecial;

		assert_eq!(
			OpcodeGenerator::from(vec![0x8, 0x34, 0x12].into_iter()).next(),
			Some(Ok(Opcode::Ld(
				Store::Indirect16(0x1234),
				Value::Register(RegisterSpecial::SP.into())
			)))
		);
		assert_eq!(
			OpcodeGenerator::from(vec![0x11, 0x50, 0x01].into_iter()).next(),
			Some(Ok(Opcode::Ld(
				Register::from(Reg16::DE).into(),
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
		use register::RegisterSpecial;

		assert_eq!(
			OpcodeGenerator::from(vec![0x39].into_iter()).next(),
			Some(Ok(Opcode::Add(
				Register::from(Reg16::HL).into(),
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
