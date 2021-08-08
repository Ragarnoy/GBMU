use super::error::Error;

use super::address;
use super::address::area::Area;
use super::address::consts::*;
use super::wram::Wram;

#[derive(Debug, Default)]
pub struct Memory {
    pub wram: Wram,
}

impl Memory {
    pub fn read(&self, address: u16) -> Result<u8, Error> {
        match address {
            WRAM_MIN..=WRAM_MAX => self.wram.read(address::relative(Area::Wram, address)),
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            WRAM_MIN..=WRAM_MAX => self
                .wram
                .write(address::relative(Area::Wram, address), data),
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }
}
