use crate::{
    area::Area,
    error::Error,
    file_operation::{Address, FileOperation},
};

pub struct SimpleRW<const SIZE: usize> {
    store: [u8; SIZE],
}

impl<const SIZE: usize> FileOperation<Area> for SimpleRW<SIZE> {
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        let address = addr.get_address();
        self.store[address] = v;
        Ok(())
    }

    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        Ok(self.store[address])
    }
}
