use crate::{Address, Error, FileOperation};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;

/// A Random Device that yeild random bytes
pub struct RandomDevice {
    gen: RefCell<SmallRng>,
}

impl Default for RandomDevice {
    fn default() -> Self {
        Self {
            gen: RefCell::new(SmallRng::from_entropy()),
        }
    }
}

impl FileOperation for RandomDevice {
    fn read(&self, _addr: Box<dyn Address>) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: Box<dyn Address>) -> Result<(), Error> {
        Err(Error::new_segfault(addr))
    }
}
