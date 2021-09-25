mod error;
#[macro_use]
mod register;
mod condition;
pub mod list;
mod store;
mod value;

use condition::Condition::{Carry, NotCarry, NotZero, Zero};
use error::Error;
use list::Opcode;
use modular_bitfield::{
    bitfield,
    specifiers::{B2, B3},
};
use register::{Register, Register16Bits as Reg16, Register8Bits as Reg8};
use std::{convert::From, fmt};
use store::Store;
use value::Value;

macro_rules! op {
    ($t:ident, $($v:expr),+) => {
        Opcode::$t($($v),+)
    };
    ($t:ident) => {
        Opcode::$t
    };
}

pub struct OpcodeGenerator<It>
where
    It: Iterator<Item = u8>,
{
    stream: It,
    current_opcode_bytes: Vec<u8>,
}

impl<It> OpcodeGenerator<It>
where
    It: Iterator<Item = u8>,
{
    pub fn new(stream: It) -> Self {
        Self {
            stream,
            current_opcode_bytes: Vec::new(),
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        self.stream.next().map(|b| {
            self.current_opcode_bytes.push(b);
            b
        })
    }

    fn read_d(&mut self) -> Option<i8> {
        self.read_byte().map(|r| r as i8)
    }

    fn read_n(&mut self) -> Option<u8> {
        self.read_byte()
    }

    fn read_nn(&mut self) -> Option<u16> {
        let bytes: [u8; 2] = [self.read_byte()?, self.read_byte()?];
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

impl<It> OpcodeGenerator<It>
where
    It: Iterator<Item = u8>,
{
    fn decode_cb_prefix(&mut self) -> Result<Opcode, Error> {
        use register::Register8Bits;

        let current = self.read_byte().ok_or(Error::InvalideOpcode(0xCB))?;

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
            // res b,n
            0x02 => Ok(op!(Res, bit, reg)),
            // set b,n
            0x03 => Ok(op!(Set, bit, reg)),
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
    type Item = Result<(Opcode, Vec<u8>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        use register::{Register16Bits, Register8Bits, RegisterSpecial};

        self.current_opcode_bytes.clear();
        let current = self.read_byte()?;

        let decode_res: Result<Opcode, Error> = match current {
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
                if self.read_byte() == Some(0x00) {
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
            0xC2 => Ok(op!(JumpConditional, NotZero, self.get_nn())),
            0xCA => Ok(op!(JumpConditional, Zero, self.get_nn())),
            0xD2 => Ok(op!(JumpConditional, NotCarry, self.get_nn())),
            0xDA => Ok(op!(JumpConditional, Carry, self.get_nn())),

            // jp (hl)
            0xE9 => Ok(op!(Jump, Value::IndirectReg16(Reg16::HL))),

            // jr d
            0x18 => Ok(op!(JumpRelative, self.get_d())),

            // jr cc,d
            0x20 => Ok(op!(JumpRelativeConditional, NotZero, self.get_d())),
            0x28 => Ok(op!(JumpRelativeConditional, Zero, self.get_d())),
            0x30 => Ok(op!(JumpRelativeConditional, NotCarry, self.get_d())),
            0x38 => Ok(op!(JumpRelativeConditional, Carry, self.get_d())),

            // call nn
            0xCD => Ok(op!(Call, self.get_nn())),

            // call cc, nn
            0xC4 => Ok(op!(CallConditional, NotZero, self.get_nn())),
            0xCC => Ok(op!(CallConditional, Zero, self.get_nn())),
            0xD4 => Ok(op!(CallConditional, NotCarry, self.get_nn())),
            0xDC => Ok(op!(CallConditional, Carry, self.get_nn())),

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
            0xC0 => Ok(op!(ReturnConditional, NotZero)),
            0xC8 => Ok(op!(ReturnConditional, Zero)),
            0xD0 => Ok(op!(ReturnConditional, NotCarry)),
            0xD8 => Ok(op!(ReturnConditional, Carry)),

            // reti
            0xD9 => Ok(op!(ReturnI)),

            _ => Err(Error::UnknownOpcode(current)),
        };
        Some(decode_res.map(|opcode| (opcode, self.current_opcode_bytes.clone())))
    }
}

#[cfg(test)]
macro_rules! op_gen {
    [$($e: expr),*] => {
        OpcodeGenerator::from(vec![$($e),*].into_iter())
    }
}

#[cfg(test)]
mod test_decode {
    use super::{
        condition::Condition::{Carry, NotCarry, NotZero, Zero},
        register::{self, Register, Register16Bits, Register8Bits},
        {Opcode, OpcodeGenerator, Reg16, Store, Value},
    };

    #[test]
    fn test_jump() {
        assert_eq!(
            op_gen![0xc3, 0x50, 0x01].next(),
            Some(Ok((op!(Jump, 0x150_u16.into()), vec![0xc3, 0x50, 0x01])))
        );
        assert_eq!(
            op_gen![0xca, 0x55, 0x00].next(),
            Some(Ok((
                op!(JumpConditional, Zero, 0x55),
                vec![0xca, 0x55, 0x00]
            )))
        );
        assert_eq!(
            op_gen![0xc2, 0x55, 0x00].next(),
            Some(Ok((
                op!(JumpConditional, NotZero, 0x55),
                vec![0xc2, 0x55, 0x00]
            )))
        );
        assert_eq!(
            op_gen![0xda, 0x55, 0x00].next(),
            Some(Ok((
                op!(JumpConditional, Carry, 0x55),
                vec![0xda, 0x55, 0x00]
            )))
        );
        assert_eq!(
            op_gen![0xd2, 0x55, 0x00].next(),
            Some(Ok((
                op!(JumpConditional, NotCarry, 0x55),
                vec![0xd2, 0x55, 0x00]
            )))
        );
    }

    #[test]
    fn test_jump_relative() {
        assert_eq!(
            op_gen![0x18, 0x42].next(),
            Some(Ok((op!(JumpRelative, 0x42), vec![0x18, 0x42])))
        );
        assert_eq!(
            op_gen![0x28, 0x42].next(),
            Some(Ok((
                op!(JumpRelativeConditional, Zero, 0x42),
                vec![0x28, 0x42]
            )))
        );
        assert_eq!(
            op_gen![0x20, 0x42].next(),
            Some(Ok((
                op!(JumpRelativeConditional, NotZero, 0x42),
                vec![0x20, 0x42]
            )))
        );
        assert_eq!(
            op_gen![0x38, 0x42].next(),
            Some(Ok((
                op!(JumpRelativeConditional, Carry, 0x42),
                vec![0x38, 0x42]
            )))
        );
        assert_eq!(
            op_gen![0x30, 0x42].next(),
            Some(Ok((
                op!(JumpRelativeConditional, NotCarry, 0x42),
                vec![0x30, 0x42]
            )))
        );
        assert_eq!(
            op_gen![0x18, (-24_i8).to_le_bytes()[0]].next(),
            Some(Ok((
                Opcode::JumpRelative(-24),
                vec![0x18, (-24_i8).to_le_bytes()[0]]
            )))
        );
        assert_eq!(
            op_gen![0x20, (-24_i8).to_le_bytes()[0]].next(),
            Some(Ok((
                op!(JumpRelativeConditional, NotZero, -24),
                vec![0x20, (-24_i8).to_le_bytes()[0]]
            )))
        );
        assert_eq!(
            op_gen![0x28, (-24_i8).to_le_bytes()[0]].next(),
            Some(Ok((
                op!(JumpRelativeConditional, Zero, -24),
                vec![0x28, (-24_i8).to_le_bytes()[0]]
            )))
        );
        assert_eq!(
            op_gen![0x30, (-24_i8).to_le_bytes()[0]].next(),
            Some(Ok((
                op!(JumpRelativeConditional, NotCarry, -24),
                vec![0x30, (-24_i8).to_le_bytes()[0]]
            )))
        );
        assert_eq!(
            op_gen![0x38, (-24_i8).to_le_bytes()[0]].next(),
            Some(Ok((
                op!(JumpRelativeConditional, Carry, -24),
                vec![0x38, (-24_i8).to_le_bytes()[0]]
            )))
        );
    }

    #[test]
    fn test_nop() {
        assert_eq!(op_gen![0x0].next(), Some(Ok((op!(Nop), vec![0x0]))));
    }

    #[test]
    fn test_halt() {
        assert_eq!(op_gen![0x76].next(), Some(Ok((op!(Halt), vec![0x76]))))
    }

    #[test]
    fn test_stop() {
        assert_eq!(
            op_gen![0x10, 0x00].next(),
            Some(Ok((op!(Stop), vec![0x10, 0x00])))
        );
    }

    #[test]
    fn test_ld() {
        use register::RegisterSpecial;

        assert_eq!(
            op_gen![0x8, 0x34, 0x12].next(),
            Some(Ok((
                op!(
                    Ld,
                    Store::Indirect16(0x1234),
                    Value::Register(RegisterSpecial::SP.into())
                ),
                vec![0x8, 0x34, 0x12]
            )))
        );
        assert_eq!(
            op_gen![0x11, 0x50, 0x01].next(),
            Some(Ok((
                op!(Ld, Register::from(Reg16::DE).into(), Value::Nn(0x150)),
                vec![0x11, 0x50, 0x01]
            )))
        );
    }

    #[test]
    fn test_ldi_ldd() {
        assert_eq!(
            op_gen![0x2a].next(),
            Some(Ok((op!(LdiInto, register8!(A).into()), vec![0x2a])))
        );
        assert_eq!(
            op_gen![0x22].next(),
            Some(Ok((op!(LdiFrom, register8!(A).into()), vec![0x22])))
        );
    }

    #[test]
    fn test_ldd() {
        use register::{Register, Register8Bits};

        assert_eq!(
            op_gen![0x3a].next(),
            Some(Ok((op!(LddInto, register8!(A).into()), vec![0x3a])))
        );
        assert_eq!(
            op_gen![0x32].next(),
            Some(Ok((op!(LddFrom, register8!(A).into()), vec![0x32])))
        );
    }

    #[test]
    fn test_ldh() {
        assert_eq!(
            op_gen![0xe0, 0xb0].next(),
            Some(Ok((op!(LdhInto, 0xb0), vec![0xe0, 0xb0])))
        );
        assert_eq!(
            op_gen![0xf0, 0x4f].next(),
            Some(Ok((op!(LdhFrom, 0x4f), vec![0xf0, 0x4f])))
        );
        assert_eq!(
            op_gen![0xf8, 0xcd].next(),
            Some(Ok((op!(Ldhl, -0x33), vec![0xf8, 0xcd])))
        )
    }

    #[test]
    fn test_push() {
        assert_eq!(
            op_gen![0xc5].next(),
            Some(Ok((op!(Push, Reg16::BC), vec![0xc5])))
        )
    }

    #[test]
    fn test_pop() {
        assert_eq!(
            op_gen![0xd1].next(),
            Some(Ok((op!(Pop, Reg16::DE), vec![0xd1])))
        )
    }

    #[test]
    fn test_add() {
        use register::RegisterSpecial;

        assert_eq!(
            op_gen![0x39].next(),
            Some(Ok((
                op!(Add, register16!(HL).into(), register_special!(SP).into()),
                vec![0x39]
            )))
        )
    }

    #[test]
    fn test_adc() {
        assert_eq!(
            op_gen![0x89].next(),
            Some(Ok((op!(Adc, register8!(C).into()), vec![0x89])))
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            op_gen![0x97].next(),
            Some(Ok((op!(Sub, register8!(A).into()), vec![0x97])))
        );
    }

    #[test]
    fn test_sbc() {
        assert_eq!(
            op_gen![0x9d].next(),
            Some(Ok((op!(Sbc, register8!(L).into()), vec![0x9d])))
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            op_gen![0xa6].next(),
            Some(Ok((op!(And, Value::IndirectReg16(Reg16::HL)), vec![0xa6])))
        );
        assert_eq!(
            op_gen![0xa2].next(),
            Some(Ok((op!(And, register8!(D).into()), vec![0xa2])))
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            op_gen![0xb0].next(),
            Some(Ok((op!(Or, register8!(B).into()), vec![0xb0])))
        )
    }

    #[test]
    fn test_xor() {
        assert_eq!(
            op_gen![0xaa].next(),
            Some(Ok((op!(Xor, register8!(D).into()), vec![0xaa])))
        );
    }

    #[test]
    fn test_cp() {
        assert_eq!(
            op_gen![0xbc].next(),
            Some(Ok((op!(Cp, register8!(H).into()), vec![0xbc])))
        );
        assert_eq!(
            op_gen![0xfe, 0x42].next(),
            Some(Ok((op!(Cp, 0x42_u8.into()), vec![0xfe, 0x42])))
        );
    }

    #[test]
    fn test_inc() {
        assert_eq!(
            op_gen![0xc].next(),
            Some(Ok((op!(Inc, register8!(C).into()), vec![0xc])))
        );
    }

    #[test]
    fn test_dec() {
        assert_eq!(
            op_gen![0xb].next(),
            Some(Ok((op!(Dec, register16!(BC).into()), vec![0xb])))
        );
    }

    #[test]
    fn test_daa() {
        assert_eq!(op_gen![0x27].next(), Some(Ok((op!(Daa), vec![0x27]))))
    }

    #[test]
    fn test_cpl() {
        assert_eq!(op_gen![0x2f].next(), Some(Ok((op!(Cpl), vec![0x2f]))))
    }

    #[test]
    fn test_ccf() {
        assert_eq!(op_gen![0x3f].next(), Some(Ok((op!(Ccf), vec![0x3f]))))
    }

    #[test]
    fn test_scf() {
        assert_eq!(op_gen![0x37].next(), Some(Ok((op!(Scf), vec![0x37]))))
    }

    #[test]
    fn test_di() {
        assert_eq!(op_gen![0xf3].next(), Some(Ok((op!(Di), vec![0xf3]))))
    }

    #[test]
    fn test_ei() {
        assert_eq!(op_gen![0xfb].next(), Some(Ok((op!(Ei), vec![0xfb]))))
    }

    #[test]
    fn test_rlca() {
        assert_eq!(op_gen![0x07].next(), Some(Ok((op!(Rlca), vec![0x07]))))
    }

    #[test]
    fn test_rla() {
        assert_eq!(op_gen![0x17].next(), Some(Ok((op!(Rla), vec![0x17]))))
    }

    #[test]
    fn test_rrca() {
        assert_eq!(op_gen![0x0f].next(), Some(Ok((op!(Rrca), vec![0x0f]))))
    }

    #[test]
    fn test_rra() {
        assert_eq!(op_gen![0x1f].next(), Some(Ok((op!(Rra), vec![0x1f]))))
    }

    #[test]
    fn test_call() {
        assert_eq!(
            op_gen![0xcd, 0x42, 0x42].next(),
            Some(Ok((op!(Call, 0x4242), vec![0xcd, 0x42, 0x42])))
        );
        assert_eq!(
            op_gen![0xc4, 0x34, 0x12].next(),
            Some(Ok((
                op!(CallConditional, NotZero, 0x1234),
                vec![0xc4, 0x34, 0x12]
            )))
        );
        assert_eq!(
            op_gen![0xcc, 0x34, 0x12].next(),
            Some(Ok((
                op!(CallConditional, Zero, 0x1234),
                vec![0xcc, 0x34, 0x12]
            )))
        );
        assert_eq!(
            op_gen![0xd4, 0x34, 0x12].next(),
            Some(Ok((
                op!(CallConditional, NotCarry, 0x1234),
                vec![0xd4, 0x34, 0x12]
            )))
        );
        assert_eq!(
            op_gen![0xdc, 0x34, 0x12].next(),
            Some(Ok((
                op!(CallConditional, Carry, 0x1234),
                vec![0xdc, 0x34, 0x12]
            )))
        );
    }

    #[test]
    fn test_restart() {
        assert_eq!(
            op_gen![0xe7].next(),
            Some(Ok((op!(Restart, 0x20), vec![0xe7])))
        )
    }

    #[test]
    fn test_return() {
        assert_eq!(op_gen![0xc9].next(), Some(Ok((op!(Return), vec![0xc9]))));
        assert_eq!(
            op_gen![0xc0].next(),
            Some(Ok((op!(ReturnConditional, NotZero), vec![0xc0])))
        );
        assert_eq!(
            op_gen![0xc8].next(),
            Some(Ok((op!(ReturnConditional, Zero), vec![0xc8])))
        );
        assert_eq!(
            op_gen![0xd0].next(),
            Some(Ok((op!(ReturnConditional, NotCarry), vec![0xd0])))
        );
        assert_eq!(
            op_gen![0xd8].next(),
            Some(Ok((op!(ReturnConditional, Carry), vec![0xd8])))
        );
        assert_eq!(op_gen![0xd9].next(), Some(Ok((op!(ReturnI), vec![0xd9]))));
    }
}

#[cfg(test)]
mod test_decode_cb_prefix {
    use super::register::{Register, Register8Bits};
    use super::{Opcode, OpcodeGenerator, Reg16, Store};

    #[test]
    fn test_swap() {
        assert_eq!(
            op_gen![0xcb, 0x33].next(),
            Some(Ok((op!(Swap, register8!(E).into()), vec![0xcb, 0x33])))
        )
    }

    #[test]
    fn test_rlc() {
        assert_eq!(
            op_gen![0xcb, 0x07].next(),
            Some(Ok((op!(Rlc, register8!(A).into()), vec![0xcb, 0x07])))
        )
    }

    #[test]
    fn test_rl() {
        assert_eq!(
            op_gen![0xcb, 0x14].next(),
            Some(Ok((op!(Rl, register8!(H).into()), vec![0xcb, 0x14])))
        )
    }

    #[test]
    fn test_rrc() {
        assert_eq!(
            op_gen![0xcb, 0x0a].next(),
            Some(Ok((op!(Rrc, register8!(D).into()), vec![0xcb, 0x0a])))
        )
    }

    #[test]
    fn test_rr() {
        assert_eq!(
            op_gen![0xcb, 0x1b].next(),
            Some(Ok((op!(Rr, register8!(E).into()), vec![0xcb, 0x1b])))
        )
    }

    #[test]
    fn test_sla() {
        assert_eq!(
            op_gen![0xcb, 0x25].next(),
            Some(Ok((op!(Sla, register8!(L).into()), vec![0xcb, 0x25])))
        )
    }

    #[test]
    fn test_sra() {
        assert_eq!(
            op_gen![0xcb, 0x2e].next(),
            Some(Ok((
                op!(Sra, Store::IndirectReg16(Reg16::HL)),
                vec![0xcb, 0x2e]
            )))
        )
    }

    #[test]
    fn test_srl() {
        assert_eq!(
            op_gen![0xcb, 0x3c].next(),
            Some(Ok((op!(Srl, register8!(H).into()), vec![0xcb, 0x3c])))
        )
    }

    #[test]
    fn test_bit() {
        assert_eq!(
            op_gen![0xcb, 0x52].next(),
            Some(Ok((op!(Bit, 2, register8!(D).into()), vec![0xcb, 0x52])))
        )
    }

    #[test]
    fn test_set() {
        assert_eq!(
            op_gen![0xcb, 0xfd].next(),
            Some(Ok((op!(Set, 7, register8!(L).into()), vec![0xcb, 0xfd])))
        )
    }

    #[test]
    fn test_res() {
        assert_eq!(
            op_gen![0xcb, 0x80].next(),
            Some(Ok((op!(Res, 0, register8!(B).into()), vec![0xcb, 0x80])))
        )
    }
}
