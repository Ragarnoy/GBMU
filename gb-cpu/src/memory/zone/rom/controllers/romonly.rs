use crate::error::Error;

pub struct RomOnly {}

impl RomOnly {
    pub fn read(memory: &Vec<u8>, address: usize) -> Result<u8, Error> {
        match memory.get(address) {
            Some(value) => Ok(*value),
            None => Err(Error::InvalidRelativeAddress(address)),
        }
    }

    pub fn write(memory: &mut Vec<u8>, address: usize, data: u8) -> Result<(), Error> {
        match memory.get_mut(address) {
            Some(value) => Ok(*value = data),
            None => Err(Error::InvalidRelativeAddress(address)),
        }
    }
}
