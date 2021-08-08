use crate::error::Error;

const WRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Wram {
    data: [u8; WRAM_SIZE],
}

impl Default for Wram {
    fn default() -> Self {
        Self::new()
    }
}

impl Wram {
    pub fn new() -> Self {
        Wram {
            data: [0; WRAM_SIZE],
        }
    }

    pub fn read(&self, address: usize) -> Result<u8, Error> {
        match self.data.get(address) {
            Some(data) => Ok(*data),
            None => Err(Error::InvalidRelativeAddress(address)),
        }
    }

    pub fn write(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if address < WRAM_SIZE {
            self.data[address] = data;
            Ok(())
        } else {
            Err(Error::InvalidRelativeAddress(address))
        }
    }
}

#[cfg(test)]
mod test_wram {
    use super::Wram;

    #[test]
    fn test_read_wram() {
        let wram = Wram::default();

        assert!(wram.read(0x10).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut wram = Wram::default();

        assert!(wram.write(0x10, 2).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut wram = Wram::default();

        assert!(wram.write(0x42, 42).is_ok());

        let read = wram.read(0x42);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }
}
