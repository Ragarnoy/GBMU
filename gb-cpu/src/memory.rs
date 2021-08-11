pub mod area;
mod consts;

use crate::error::Error;
use crate::bus::Bus;
use area::{Area, Wram, Rom};
use area::rom::{Mbc, NoMbc};

pub struct Memory {
    pub wram: Wram,
    pub rom: Box<dyn Rom<Item = u8, Result = Result<(), Error>>>,
}

impl Memory {
    pub fn new(mbc: Mbc, data: Vec<u8>) -> Self {
        let rom: Box<dyn Rom<Item = u8, Result = Result<(), Error>>> = match mbc {
            Mbc::NoMbc => Box::new(NoMbc::new(data)),
        };

        Memory {
            rom,
            wram: Wram::default(),
        }
    }
}

impl Bus<u16> for Memory {
    type Item = Result<u8, Error>;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: u16) -> Self::Item {
        match address {
            consts::ROM_MIN..=consts::ROM_MAX => Ok(self.rom.get(Area::Rom.relative(address))),
            consts::WRAM_MIN..=consts::WRAM_MAX => Ok(self.wram.get(Area::Wram.relative(address))),
            _ => Err(Error::SegmentationFault(address)),
        }
    }

    fn set(&mut self, address: u16, data: Self::Data) -> Self::Result {
        match address {
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                self.wram.set(Area::Wram.relative(address), data);
                Ok(())
            },
            consts::ROM_MIN..=consts::ROM_MAX => {
                self.wram.set(Area::Rom.relative(address), data);
                Ok(())
            }
            _ => Err(Error::SegmentationFault(address)),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new(Mbc::NoMbc, vec![5])
    }
}


#[cfg(test)]
mod test_memory {
    use super::Bus;
    use super::Memory;

    #[test]
    fn test_invalid_read() {
        let memory = Memory::default();

        assert!(memory.get(0xfea1).is_err())
    }

    #[test]
    fn test_invalid_write() {
        let mut memory = Memory::default();

        assert!(memory.set(0xfea1, 42).is_err())
    }

    #[test]
    fn test_read_wram() {
        let memory = Memory::default();

        assert!(memory.get(0xc010).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut memory = Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut memory = Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());

        let read = memory.get(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }
}
