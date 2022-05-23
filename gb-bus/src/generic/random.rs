use crate::{Address, Area, Error, FileOperation, IORegArea, Source};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;

/// A Random Device that yield random bytes
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

impl<A> FileOperation<A, Area> for RandomDevice
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, _addr: A, _source: Option<Source>) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        Err(Error::new_segfault(addr.into()))
    }
}

impl<A> FileOperation<A, IORegArea> for RandomDevice
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, _addr: A, _source: Option<Source>) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        Err(Error::new_segfault(addr.into()))
    }
}
