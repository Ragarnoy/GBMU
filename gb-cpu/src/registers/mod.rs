use std::fmt;

#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pub pc: u16,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registers {{ a: {}, f: {}, b: {}, c: {}, d: {}, e: {}, h: {}, l: {}, sp: {}, cp: {} }}",
            self.a,
            self.f,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc)
    }
}

impl Registers {
    pub fn next_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}
