mod zone;

use crate::error::Error;
use crate::getset::*;

use crate::address;
use crate::address::area::Area;
use crate::address::consts::*;
use zone::wram::Wram;
use zone::rom::Rom;

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

#[cfg(test)]
mod test_memory {
    use super::Memory;

    #[test]
    fn test_invalid_read() {
        let memory = Memory::default();

        assert!(memory.read(0xfea1).is_err())
    }

    #[test]
    fn test_invalid_write() {
        let mut memory = Memory::default();

        assert!(memory.write(0xfea1, 42).is_err())
    }

    #[test]
    fn test_read_wram() {
        let memory = Memory::default();

        assert!(memory.read(0xc010).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut memory = Memory::default();

        assert!(memory.write(0xc010, 42).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut memory = Memory::default();

        assert!(memory.write(0xc010, 42).is_ok());

        let read = memory.read(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }
}
