mod area;
mod consts;

use crate::error::Error;

use crate::bus::Bus;
use area::{Area, Wram};

#[derive(Debug, Default)]
pub struct Memory {
    pub wram: Wram,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
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
            consts::WRAM_MIN..=consts::WRAM_MAX => Ok(self.wram.get(Area::Wram.relative(address))),
            _ => Err(Error::SegmentationFault(address)),
        }
    }

    fn set(&mut self, address: u16, data: Self::Data) -> Self::Result {
        match address {
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                self.wram.set(Area::Wram.relative(address), data);
                Ok(())
            }
            _ => Err(Error::InvalidAbsoluteAddress(address)),
        }
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
