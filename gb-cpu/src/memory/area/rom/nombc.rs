use crate::bus::Bus;
use crate::error::Error;

pub struct NoMbc {
    data: Vec<u8>,
}

impl Bus<usize> for NoMbc {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> u8 {
        *self.data.get(address).unwrap()
    }

    fn set(&mut self, _address: usize, _data: u8) -> Self::Result {
        Ok(())
    }
}

impl NoMbc {
    pub fn new(data: Vec<u8>) -> Self {
        NoMbc { data }
    }
}
