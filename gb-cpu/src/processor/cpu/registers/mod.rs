pub mod area;

use area::*;
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

impl RWRegister<_8Bits> for Registers {
    type Output = u8;

    fn read(&self, area: _8Bits) -> u8 {
        match area {
            _8Bits::B => self.b,
            _8Bits::C => self.c,
            _8Bits::D => self.d,
            _8Bits::E => self.e,
            _8Bits::H => self.h,
            _8Bits::L => self.l,
        }
    }

    fn write(&mut self, area: _8Bits, data: u8) {
        match area {
            _8Bits::B => self.b = data,
            _8Bits::C => self.c = data,
            _8Bits::D => self.d = data,
            _8Bits::E => self.e = data,
            _8Bits::H => self.h = data,
            _8Bits::L => self.l = data,
        };
    }
}

impl RWRegister<_16Bits> for Registers {
    type Output = u16;

    fn read(&self, area: _16Bits) -> u16 {
        match area {
            _16Bits::SP => self.sp,
            _16Bits::PC => self.pc,
            _16Bits::BC => (self.b as u16) << 8 | self.c as u16,
            _16Bits::DE => (self.d as u16) << 8 | self.e as u16,
            _16Bits::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    fn write(&mut self, area: _16Bits, data: u16){
        match area{
            _16Bits::SP => {
                self.sp = data;
            },
            _16Bits::PC => {
                self.pc = data;
            },
            _16Bits::BC => {
                self.b = (data >> 8) as u8;
                self.c = data as u8;
            },
            _16Bits::DE => {
                self.d = (data >> 8) as u8;
                self.e = data as u8;
            },
            _16Bits::HL => {
                self.h = (data >> 8) as u8;
                self.l = data as u8;
            },
        }
    }
}


impl Registers {
    pub fn next_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}

#[cfg(test)]
mod test_registers{
    use super::Registers;
    use super::area::*;

    #[test]
    fn test_valid_write_read_8bits() {
        let mut registers = Registers::default();


        registers.write(_8Bits::C, 42);
        let value: u8 = registers.read(_8Bits::C);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_valid_write_read_16bits() {
        let mut registers = Registers::default();


        registers.write(_16Bits::BC, 42);
        let value: u16 = registers.read(_16Bits::BC);
        assert_eq!(value, 42);
    }
}
