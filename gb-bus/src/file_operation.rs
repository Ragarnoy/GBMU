use crate::Error;
use std::convert::From;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation<A: Into<u16>> {
    fn write(&mut self, v: u8, addr: Box<dyn Address<A>>) -> Result<(), Error> {
        let _v = v;
        Err(Error::new_segfault(addr))
    }

    fn read(&self, addr: Box<dyn Address<A>>) -> Result<u8, Error>;
}

pub trait Address<A> {
    /// Return the relative address in the current area
    fn get_address(&self) -> usize;

    /// Return the current area type
    fn area_type(&self) -> A;
}

impl<A: PartialEq + Eq> PartialEq for dyn Address<A> {
    fn eq(&self, other: &Self) -> bool {
        self.get_address() == other.get_address() && self.area_type() == other.area_type()
    }
}

impl<A: Into<u16>> From<Box<dyn Address<A>>> for u16 {
    fn from(addr: Box<dyn Address<A>>) -> Self {
        (addr.get_address() as u16) + addr.area_type().into()
    }
}
