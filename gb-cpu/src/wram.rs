use super::error::Error;

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
            None => Err(Error::InvalidAddress(address)),
        }
    }
    pub fn write(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if address < WRAM_SIZE {
            self.data[address] = data;
            Ok(())
        } else {
            Err(Error::InvalidAddress(address))
        }
    }
}
