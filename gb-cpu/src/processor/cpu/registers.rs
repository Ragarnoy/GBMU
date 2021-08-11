pub mod area;

use crate::getset::*;
use area::{Bits16, Bits8};
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
impl Set<Bits8> for Registers {
    type Result = ();
    type Data = u8;

    fn set(&mut self, area: Bits8, data: u8) {
        match area {
            Bits8::B => self.b = data,
            Bits8::C => self.c = data,
            Bits8::D => self.d = data,
            Bits8::E => self.e = data,
            Bits8::H => self.h = data,
            Bits8::L => self.l = data,
        };
    }
}

impl Get<Bits8> for Registers {
    type Item = u8;

    fn get(&self, area: Bits8) -> u8 {
        match area {
            Bits8::B => self.b,
            Bits8::C => self.c,
            Bits8::D => self.d,
            Bits8::E => self.e,
            Bits8::H => self.h,
            Bits8::L => self.l,
        }
    }
}

impl Get<Bits16> for Registers {
    type Item = u16;

    fn get(&self, area: Bits16) -> u16 {
        match area {
            Bits16::SP => self.sp,
            Bits16::PC => self.pc,
            Bits16::BC => (self.b as u16) << 8 | self.c as u16,
            Bits16::DE => (self.d as u16) << 8 | self.e as u16,
            Bits16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }
}

impl Set<Bits16> for Registers {
    type Result = ();
    type Data = u16;

    fn set(&mut self, area: Bits16, data: u16) {
        match area {
            Bits16::SP => {
                self.sp = data;
            }
            Bits16::PC => {
                self.pc = data;
            }
            Bits16::BC => {
                self.b = (data >> 8) as u8;
                self.c = data as u8;
            }
            Bits16::DE => {
                self.d = (data >> 8) as u8;
                self.e = data as u8;
            }
            Bits16::HL => {
                self.h = (data >> 8) as u8;
                self.l = data as u8;
            }
        }
    }
}

impl Registers {
    pub fn next_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}

#[cfg(test)]
mod test_registers {
    use super::area::*;
    use super::Registers;
    use crate::getset::*;

    #[test]
    fn test_valid_write_read_8bits() {
        let mut registers = Registers::default();

        registers.set(Bits8::C, 42);
        let value = registers.get(Bits8::C);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_valid_write_read_16bits() {
        let mut registers = Registers::default();

        registers.set(Bits16::BC, 42);
        let value = registers.get(Bits16::BC);
        assert_eq!(value, 42);
    }
}
