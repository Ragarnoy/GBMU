use super::register::{Register, Register16Bits, Register8Bits};
use std::fmt;

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
	Push(Register16Bits),

	/// Pop u16 from stack
	/// inc SP twice
	/// Timing: 12
	Pop(Register16Bits),

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
			Self::LdhFrom(v) => write!(f, "ldh A, (0xff00 + {:x})", v),
			Self::LdhInto(s) => write!(f, "ldh (0xff00 + {:x}), A", s),
			Self::Ldhl(addr) => write!(f, "ldhl SP, {:x}", addr),

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
	use super::register::{Register8Bits, RegisterSpecial};

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
