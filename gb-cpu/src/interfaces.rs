pub trait Read8BitsReg {
    fn a(&self) -> u8;
    fn b(&self) -> u8;
    fn c(&self) -> u8;
    fn d(&self) -> u8;
    fn e(&self) -> u8;
    fn h(&self) -> u8;
    fn l(&self) -> u8;
}

pub trait Read8BitsRegExt: Read8BitsReg {
    fn f(&self) -> u8;
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

pub trait Write8BitsRegExt: Write8BitsReg {
    fn set_f(&mut self, value: u8);
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

