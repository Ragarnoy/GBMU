use crate::bus::Bus;
use crate::error::Error;

pub struct NoMbc {
    data: Vec<u8>,
}

impl Bus<usize> for NoMbc {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        *self.data.get(address).unwrap()
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        Err(Error::SetError(address, data))
    }
}

impl NoMbc {
    pub fn new(data: Vec<u8>) -> Self {
        NoMbc { data }
    }
}
