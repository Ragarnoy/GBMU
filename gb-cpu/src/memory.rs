mod area;
mod consts;

use std::fs::File;

use crate::error::Error;
use crate::getset::*;
use consts::*;

use area::Area;
use area::wram::Wram;
use area::rom::Rom;

#[derive(Debug)]
pub struct Memory {
    pub wram: Wram,
    pub rom: Rom
}

impl Memory {
    pub fn new(bios: File, cartrige: File) -> Self {
        Memory{
            rom: Rom::new(bios, cartrige),
            wram: Wram::default()
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, Error> {
        match address {
            WRAM_MIN..=WRAM_MAX => Ok(self.wram.get(Area::Wram.relative(address))),
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            WRAM_MIN..=WRAM_MAX => Ok(self
                .wram
                .set(Area::Wram.relative(address), data)),
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }
}

//#[cfg(test)]
// mod test_memory {
//     use super::Memory;
//
//     #[test]
//     fn test_invalid_read() {
//         let memory = Memory::default();
//
//         assert!(memory.read(0xfea1).is_err())
//     }
//
//     #[test]
//     fn test_invalid_write() {
//         let mut memory = Memory::default();
//
//         assert!(memory.write(0xfea1, 42).is_err())
//     }
//
//     #[test]
//     fn test_read_wram() {
//         let memory = Memory::default();
//
//         assert!(memory.read(0xc010).is_ok());
//     }
//
//     #[test]
//     fn test_write_wram() {
//         let mut memory = Memory::default();
//
//         assert!(memory.write(0xc010, 42).is_ok());
//     }
//
//     #[test]
//     fn test_write_read_wram() {
//         let mut memory = Memory::default();
//
//         assert!(memory.write(0xc010, 42).is_ok());
//
//         let read = memory.read(0xc010);
//
//         assert!(read.is_ok());
//         assert_eq!(read.unwrap(), 42);
//     }
// }
