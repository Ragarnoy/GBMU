use super::area::{Bits16, Bits8, Flag};
use super::flags::Flags;
use crate::bus::Bus;

#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pub pc: u16,
}

impl Bus<Bits8> for Registers {
    type Result = ();
    type Data = u8;
    type Item = u8;

    fn get(&self, area: Bits8) -> Self::Item {
        match area {
            Bits8::A => self.a,
            Bits8::B => self.b,
            Bits8::C => self.c,
            Bits8::D => self.d,
            Bits8::E => self.e,
            Bits8::H => self.h,
            Bits8::L => self.l,
        }
    }

    fn set(&mut self, area: Bits8, data: Self::Data) -> Self::Result {
        match area {
            Bits8::A => self.a = data,
            Bits8::B => self.b = data,
            Bits8::C => self.c = data,
            Bits8::D => self.d = data,
            Bits8::E => self.e = data,
            Bits8::H => self.h = data,
            Bits8::L => self.l = data,
        };
    }
}

impl Bus<Bits16> for Registers {
    type Item = u16;
    type Result = ();
    type Data = u16;

    fn set(&mut self, area: Bits16, data: Self::Data) -> Self::Result {
        match area {
            Bits16::AF => {
                self.a = (data >> 8) as u8;
                self.f = Flags::from_bytes([data as u8]);
            }
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

    fn get(&self, area: Bits16) -> Self::Item {
        match area {
            Bits16::SP => self.sp,
            Bits16::PC => self.pc,
            Bits16::AF => (self.a as u16) << 8 | self.f.into_bytes()[0] as u16,
            Bits16::BC => (self.b as u16) << 8 | self.c as u16,
            Bits16::DE => (self.d as u16) << 8 | self.e as u16,
            Bits16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }
}

impl Bus<Flag> for Registers {
    type Result = ();
    type Data = bool;
    type Item = bool;

    fn get(&self, flag: Flag) -> Self::Item {
        self.f.get(flag)
    }

    fn set(&mut self, flag: Flag, data: Self::Data) -> Self::Result {
        self.f.set(flag, data)
    }
}

impl Registers {
    pub fn next_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}

#[cfg(test)]
mod test_registers {
    use super::Registers;
    use super::{Bits16, Bits8};
    use crate::bus::Bus;

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

    #[test]
    fn test_valid_write_read_af_register() {
        let mut registers = Registers::default();

        registers.set(Bits16::AF, 0xFFFF);
        let value = registers.get(Bits16::AF);
        assert_eq!(value, 0xFFFF);
    }
}
