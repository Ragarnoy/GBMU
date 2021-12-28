use crate::{Addr, Area, Error, FileOperation, IORegArea};
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

impl FileOperation<Addr<Area>, Area> for RandomDevice {
    fn read(&self, _addr: Addr<Area>) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: Addr<Area>) -> Result<(), Error> {
        Err(Error::new_segfault(addr.into()))
    }
}

impl FileOperation<Addr<IORegArea>, IORegArea> for RandomDevice {
    fn read(&self, _addr: Addr<IORegArea>) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: Addr<IORegArea>) -> Result<(), Error> {
        Err(Error::new_segfault(addr.into()))
    }
}
