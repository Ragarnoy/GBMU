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

pub trait Read8BitsReg {
    fn a(&self) -> u8;
    fn b(&self) -> u8;
    fn c(&self) -> u8;
    fn d(&self) -> u8;
    fn e(&self) -> u8;
    fn h(&self) -> u8;
    fn l(&self) -> u8;
}

pub trait Write8BitsReg {
    fn set_a(&mut self, value: u8);
    fn set_b(&mut self, value: u8);
    fn set_c(&mut self, value: u8);
    fn set_d(&mut self, value: u8);
    fn set_e(&mut self, value: u8);
    fn set_h(&mut self, value: u8);
    fn set_l(&mut self, value: u8);
}

pub trait ReadFlagReg {
    fn zero(&self) -> bool;
    fn substraction(&self) -> bool;
    fn half_carry(&self) -> bool;
    fn carry(&self) -> bool;
    fn raw(&self) -> u8;
}

pub trait WriteFlagReg {
    fn set_zero(&mut self, value: bool);
    fn set_subtraction(&mut self, value: bool);
    fn set_half_carry(&mut self, value: bool);
    fn set_carry(&mut self, value: bool);
    fn raw(&mut self, value: u8);
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

impl Write8BitsReg for Registers {
    fn set_a(&mut self, value: u8) {
        self.af = u16::from_be_bytes([value, self.af.to_be_bytes()[1]]);
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
