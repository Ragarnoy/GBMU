use crate::bus::Bus;
use crate::error::Error;

pub const NOMBC_MAX_SIZE: usize = 32_768;

pub struct NoMbc {
    data: Vec<u8>,
}

impl Bus<usize> for NoMbc {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        self.data[address]
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        Err(Error::IllegalSet(address, data))
    }
}

impl NoMbc {
    pub fn new(data: Vec<u8>) -> Self {
        NoMbc { data }
    }
}

impl Default for NoMbc {
    fn default() -> Self {
        NoMbc::new(vec![0; NOMBC_MAX_SIZE])
    }
}

#[cfg(test)]
mod test_nombc {
    use super::NoMbc;
    use crate::bus::Bus;

    #[test]
    fn test_read_nombc() {
        let nombc = NoMbc::default();

        assert_eq!(nombc.get(0x10), 0);
    }

    #[test]
    fn test_write_read_nombc() {
        let mut nombc = NoMbc::default();

        match nombc.set(0x42, 42) {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }
}
