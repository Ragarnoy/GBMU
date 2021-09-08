use crate::interfaces::{
    Read8BitsReg, Read8BitsRegExt, ReadFlagReg, Write8BitsReg, Write8BitsRegExt, WriteFlagReg,
};

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

impl Read8BitsReg for Registers {
    fn a(&self) -> u8 {
        self.af.to_be_bytes()[0]
    }

    fn b(&self) -> u8 {
        self.bc.to_be_bytes()[0]
    }

    fn c(&self) -> u8 {
        self.bc.to_be_bytes()[1]
    }

    fn d(&self) -> u8 {
        self.de.to_be_bytes()[0]
    }

    fn e(&self) -> u8 {
        self.de.to_be_bytes()[1]
    }

    fn h(&self) -> u8 {
        self.hl.to_be_bytes()[0]
    }

    fn l(&self) -> u8 {
        self.hl.to_be_bytes()[1]
    }
}

impl Read8BitsRegExt for Registers {
    fn f(&self) -> u8 {
        self.af.to_be_bytes()[1]
    }
}

impl Write8BitsReg for Registers {
    fn set_a(&mut self, value: u8) {
        self.af = u16::from_be_bytes([value, self.f()]);
    }

    fn set_b(&mut self, value: u8) {
        self.bc = u16::from_be_bytes([value, self.c()]);
    }

    fn set_c(&mut self, value: u8) {
        self.bc = u16::from_be_bytes([self.b(), value]);
    }

    fn set_d(&mut self, value: u8) {
        self.de = u16::from_be_bytes([value, self.e()]);
    }

    fn set_e(&mut self, value: u8) {
        self.de = u16::from_be_bytes([self.d(), value]);
    }

    fn set_h(&mut self, value: u8) {
        self.hl = u16::from_be_bytes([value, self.l()]);
    }

    fn set_l(&mut self, value: u8) {
        self.hl = u16::from_be_bytes([self.h(), value]);
    }
}

impl Write8BitsRegExt for Registers {
    fn set_f(&mut self, value: u8) {
        self.af = u16::from_be_bytes([self.a(), value]);
    }
}

impl ReadFlagReg for Registers {
    fn zero(&self) -> bool {
        self.f() >> 7 == 0b1
    }

    fn substraction(&self) -> bool {
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

impl WriteFlagReg for Registers {
    fn set_zero(&mut self, value: bool) {
        if value {
            self.af |= 0b1000_0000;
        } else {
            self.af &= !0b1000_0000;
        }
    }

    fn set_subtraction(&mut self, value: bool) {
        if value {
            self.af |= 0b100_0000;
        } else {
            self.af &= !0b100_0000;
        }
    }

    fn set_half_carry(&mut self, value: bool) {
        if value {
            self.af |= 0b10_0000;
        } else {
            self.af &= !0b10_0000;
        }
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.af |= 0b1_0000;
        } else {
            self.af &= !0b1_0000;
        }
    }

    fn raw(&mut self, value: u8) {
        self.set_f(value)
    }
}
