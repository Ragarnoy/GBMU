use crate::interfaces::{
    Read8BitsReg, Read8BitsRegExt, ReadFlagReg, Write8BitsReg, Write8BitsRegExt, WriteFlagReg,
};

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Registers {
    /// Accumulator & Flags
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    /// Stack Pointer
    pub sp: u16,
    /// Program Counter / Pointer
    pub pc: u16,
}

const HIGH: usize = 0;
const LOW: usize = 1;

impl Read8BitsReg for Registers {
    fn a(&self) -> u8 {
        self.af.to_be_bytes()[HIGH]
    }

    fn b(&self) -> u8 {
        self.bc.to_be_bytes()[HIGH]
    }

    fn c(&self) -> u8 {
        self.bc.to_be_bytes()[LOW]
    }

    fn d(&self) -> u8 {
        self.de.to_be_bytes()[HIGH]
    }

    fn e(&self) -> u8 {
        self.de.to_be_bytes()[LOW]
    }

    fn h(&self) -> u8 {
        self.hl.to_be_bytes()[HIGH]
    }

    fn l(&self) -> u8 {
        self.hl.to_be_bytes()[LOW]
    }
}

impl Read8BitsRegExt for Registers {
    fn f(&self) -> u8 {
        self.af.to_be_bytes()[LOW]
    }
}

impl Write8BitsReg for Registers {
    fn set_a(&mut self, value: u8) {
        self.af = u16::from_le_bytes([self.f(), value]);
    }

    fn set_b(&mut self, value: u8) {
        self.bc = u16::from_le_bytes([self.c(), value]);
    }

    fn set_c(&mut self, value: u8) {
        self.bc = u16::from_le_bytes([value, self.b()]);
    }

    fn set_d(&mut self, value: u8) {
        self.de = u16::from_le_bytes([self.e(), value]);
    }

    fn set_e(&mut self, value: u8) {
        self.de = u16::from_le_bytes([value, self.d()]);
    }

    fn set_h(&mut self, value: u8) {
        self.hl = u16::from_le_bytes([self.l(), value]);
    }

    fn set_l(&mut self, value: u8) {
        self.hl = u16::from_le_bytes([value, self.h()]);
    }
}

impl Write8BitsRegExt for Registers {
    fn set_f(&mut self, value: u8) {
        self.af = u16::from_le_bytes([value, self.a()]);
    }
}

impl ReadFlagReg for Registers {
    fn zero(&self) -> bool {
        self.f() >> 7 == 0b1
    }

    fn subtraction(&self) -> bool {
        (self.f() >> 6 & 1) == 1
    }

    fn half_carry(&self) -> bool {
        (self.f() >> 5 & 1) == 1
    }

    fn carry(&self) -> bool {
        (self.f() >> 4 & 1) == 1
    }

    fn raw(&self) -> u8 {
        self.f()
    }
}

pub const ZERO_MASK: u8 = 0b1000_0000;
pub const SUBSTRACTION_MASK: u8 = 0b100_0000;
pub const HALF_CARRY_MASK: u8 = 0b10_0000;
pub const CARRY_MASK: u8 = 0b1_0000;

impl WriteFlagReg for Registers {
    fn set_zero(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | ZERO_MASK)
        } else {
            self.set_f(self.f() & !ZERO_MASK)
        }
    }

    fn set_subtraction(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | SUBSTRACTION_MASK)
        } else {
            self.set_f(self.f() & !SUBSTRACTION_MASK)
        }
    }

    fn set_half_carry(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | HALF_CARRY_MASK)
        } else {
            self.set_f(self.f() & !HALF_CARRY_MASK)
        }
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.set_f(self.f() | CARRY_MASK)
        } else {
            self.set_f(self.f() & !CARRY_MASK)
        }
    }

    fn set_raw(&mut self, value: u8) {
        self.set_f(value)
    }
}

impl Registers {
    pub const DMG: Registers = Registers {
        af: 0x01B0,
        bc: 0x0013,
        de: 0x00D8,
        hl: 0x014D,
        sp: 0xFFFE,
        pc: 0x0100,
    };
}
