pub mod area;

use super::error::Error;
use area::Area;
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

    pub fn read_u8(&self, area: Area) -> Result<u8, Error> {
        match area {
            Area::A => Ok(self.a),
            Area::F => Ok(self.f),
            Area::B => Ok(self.b),
            Area::C => Ok(self.c),
            Area::D => Ok(self.d),
            Area::E => Ok(self.e),
            Area::H => Ok(self.h),
            Area::L => Ok(self.l),
            _ => Err(Error::InvalidRegister(area)),
        }
    }

    pub fn read_u16(&self, area: Area) -> Result<u16, Error> {
        match area {
            Area::SP => Ok(self.sp),
            Area::PC => Ok(self.pc),
            Area::AF => Ok((self.a as u16) << 8 | self.f as u16),
            Area::BC => Ok((self.b as u16) << 8 | self.c as u16),
            Area::DE => Ok((self.d as u16) << 8 | self.e as u16),
            Area::HL => Ok((self.h as u16) << 8 | self.l as u16),
            _ => Err(Error::InvalidRegister(area)),
        }
    }

    pub fn write_u8(&mut self, area: Area, data: u8) -> Result<(), Error> {
        match area {
            Area::A => {
                self.a = data;
                Ok(())
            }
            Area::F => {
                self.f = data;
                Ok(())
            }
            Area::B => {
                self.b = data;
                Ok(())
            }
            Area::C => {
                self.c = data;
                Ok(())
            }
            Area::D => {
                self.d = data;
                Ok(())
            }
            Area::E => {
                self.e = data;
                Ok(())
            }
            Area::H => {
                self.h = data;
                Ok(())
            }
            Area::L => {
                self.l = data;
                Ok(())
            }
            _ => Err(Error::InvalidRegister(area)),
        }
    }

    pub fn write_u16(&mut self, area: Area, data: u16) -> Result<(), Error> {
        match area {
            Area::SP => {
                self.sp = data;
                Ok(())
            }
            Area::PC => {
                self.pc;
                Ok(())
            }
            Area::AF => {
                self.a = (data >> 8) as u8;
                self.f = data as u8;
                Ok(())
            }
            Area::BC => {
                self.b = (data >> 8) as u8;
                self.c = data as u8;
                Ok(())
            }
            Area::DE => {
                self.d = (data >> 8) as u8;
                self.e = data as u8;
                Ok(())
            }
            Area::HL => {
                self.h = (data >> 8) as u8;
                self.l = data as u8;
                Ok(())
            }
            _ => Err(Error::InvalidRegister(area)),
        }
    }
}
