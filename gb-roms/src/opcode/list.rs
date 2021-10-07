use super::{condition::Condition, register::Register16Bits, store::Store, value::Value};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    /// jump to addr
    /// Timing:
    /// - u16: 12
    /// - *HL: 4
    Jump(Value),

    /// jump to addr when condition is meet
    /// Timing: 12
    JumpConditional(Condition, u16),

    /// relative jump to PC + value
    JumpRelative(i8),
    /// relative jump to PC + value when condition is meet
    JumpRelativeConditional(Condition, i8),

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
    /// when condition is meet
    /// Timing: 12
    CallConditional(Condition, u16),

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

    /// When condition is meet
    /// Pop u16 from stack & jump to that addr
    /// Timing: 8
    ReturnConditional(Condition),
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(addr) => write!(f, "JP {}", addr),
            Self::JumpConditional(cond, addr) => write!(f, "JP {}, {:x}", cond, addr),

            Self::JumpRelative(value) => write!(f, "JR {:x}", value),
            Self::JumpRelativeConditional(cond, value) => write!(f, "JR {}, {:x}", cond, value),
            Self::Nop => write!(f, "NOP"),
            Self::Halt => write!(f, "HALT"),
            Self::Stop => write!(f, "STOP"),

            Self::Ld(from, to) => write!(f, "LD {}, {}", from, to),
            Self::LddFrom(v) => write!(f, "LDD (HL), {}", v),
            Self::LdiFrom(v) => write!(f, "LDI (HL), {}", v),
            Self::LddInto(s) => write!(f, "LDD {}, (HL)", s),
            Self::LdiInto(s) => write!(f, "LDI {}, (HL)", s),
            Self::LdhFrom(v) => write!(f, "LDH A, (0xff00 + {:x})", v),
            Self::LdhInto(s) => write!(f, "LDH (0xff00 + {:x}), A", s),
            Self::Ldhl(addr) => write!(f, "LDHL SP, {:x}", addr),

            Self::Push(reg) => write!(f, "PUSH {}", reg),
            Self::Pop(reg) => write!(f, "POP {}", reg),

            Self::Add(s, v) => write!(f, "ADD {}, {}", s, v),
            Self::Adc(v) => write!(f, "ADC A, {}", v),
            Self::Sub(v) => write!(f, "SUB A, {}", v),
            Self::Sbc(v) => write!(f, "SBC A, {}", v),
            Self::And(v) => write!(f, "AND A, {}", v),
            Self::Or(v) => write!(f, "OR A, {}", v),
            Self::Xor(v) => write!(f, "XOR A, {}", v),
            Self::Cp(v) => write!(f, "CP A, {}", v),

            Self::Inc(s) => write!(f, "INC {}", s),
            Self::Dec(s) => write!(f, "DEC {}", s),

            Self::Swap(s) => write!(f, "SWAP {}", s),

            Self::Daa => write!(f, "DAA"),
            Self::Cpl => write!(f, "CPL"),
            Self::Ccf => write!(f, "CCF"),
            Self::Scf => write!(f, "SCF"),

            Self::Di => write!(f, "DI"),
            Self::Ei => write!(f, "EI"),

            Self::Rlca => write!(f, "RLCA"),
            Self::Rla => write!(f, "RLA"),

            Self::Rrca => write!(f, "RRCA"),
            Self::Rra => write!(f, "RRA"),

            Self::Rlc(n) => write!(f, "RLC {}", n),
            Self::Rl(n) => write!(f, "RL {}", n),
            Self::Rrc(n) => write!(f, "RRC {}", n),
            Self::Rr(n) => write!(f, "RR {}", n),

            Self::Sla(n) => write!(f, "SLA {}", n),
            Self::Sra(n) => write!(f, "SRA {}", n),
            Self::Srl(n) => write!(f, "SRL {}", n),

            Self::Bit(b, r) => write!(f, "BIT {}, {}", b, r),
            Self::Set(b, r) => write!(f, "SET {}, {}", b, r),
            Self::Res(b, r) => write!(f, "RES {}, {}", b, r),

            Self::Call(addr) => write!(f, "CALL {:x}", addr),
            Self::CallConditional(cond, addr) => write!(f, "CALL {}, {:x}", cond, addr),

            Self::Restart(addr) => write!(f, "RST {:x}", addr),

            Self::Return => write!(f, "RET"),
            Self::ReturnI => write!(f, "RETI"),
            Self::ReturnConditional(cond) => write!(f, "RET {}", cond),
        }
    }
}

#[test]
fn test_display_opcode() {
    use super::register::{Register, Register8Bits, RegisterSpecial};

    assert_eq!(Opcode::Jump(0x150_u16.into()).to_string(), "JP 150");

    assert_eq!(Opcode::JumpRelative(0x42).to_string(), "JR 42");
    assert_eq!(
        Opcode::JumpRelativeConditional(Condition::NotZero, 0x42).to_string(),
        "JR NZ, 42"
    );
    assert_eq!(
        Opcode::JumpRelativeConditional(Condition::Zero, 0x42).to_string(),
        "JR Z, 42"
    );
    assert_eq!(
        Opcode::JumpRelativeConditional(Condition::NotCarry, 0x42).to_string(),
        "JR NC, 42"
    );
    assert_eq!(
        Opcode::JumpRelativeConditional(Condition::Carry, 0x42).to_string(),
        "JR C, 42"
    );

    assert_eq!(Opcode::Nop.to_string(), "NOP");
    assert_eq!(Opcode::Stop.to_string(), "STOP");
    assert_eq!(
        Opcode::Ld(
            Store::Indirect16(0x123),
            Value::Register(RegisterSpecial::SP.into())
        )
        .to_string(),
        "LD (123), SP"
    );

    assert_eq!(
        Opcode::LddFrom(register8!(A).into()).to_string(),
        "LDD (HL), A"
    );
    assert_eq!(
        Opcode::LdiFrom(register8!(A).into()).to_string(),
        "LDI (HL), A"
    );
    assert_eq!(
        Opcode::LddInto(register8!(A).into()).to_string(),
        "LDD A, (HL)"
    );
    assert_eq!(
        Opcode::LdiInto(register8!(A).into()).to_string(),
        "LDI A, (HL)"
    );
}
