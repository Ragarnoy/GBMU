pub mod area;
mod consts;

use crate::error::Error;

use crate::bus::Bus;
use area::rom::{Mbc, NoMbc};
use area::{Area, Wram};

type Rom = Box<dyn Bus<usize, Item = u8, Result = Result<(), Error>, Data = u8>>;

pub struct Memory {
    pub wram: Wram,
    pub rom: Rom,
}

impl Bus<u16> for Memory {
    type Item = Result<u8, Error>;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: u16) -> Result<u8, Error> {
        match address {
            consts::ROM_MIN..=consts::ROM_MAX => Ok(self.rom.get(Area::Rom.relative(address))),
            consts::WRAM_MIN..=consts::WRAM_MAX => Ok(self.wram.get(Area::Wram.relative(address))),
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }

    fn set(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                Ok(self.wram.set(Area::Wram.relative(address), data))
            }
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
    }
}

impl Memory {
    pub fn new(mbc: Mbc, data: Vec<u8>) -> Self {
        let rom: Rom = match mbc {
            Mbc::NoMbc => Box::new(NoMbc::new(data)),
        };

        Memory {
            rom,
            wram: Wram::default(),
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
