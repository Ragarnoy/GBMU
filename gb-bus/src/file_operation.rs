use crate::Area;
use crate::Error;
use std::convert::From;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Box<dyn Address>) -> Result<(), Error> {
        let _v = v;
        Err(Error::SegmentationFault(addr.into()))
    }

    fn read(&self, addr: Box<dyn Address>) -> Result<u8, Error>;
}

pub trait Address {
    /// Return the relative address in the current area
    fn get_address(&self) -> usize;

    /// Return the current area type
    fn area_type(&self) -> Area;
}

impl PartialEq for dyn Address {
    fn eq(&self, other: &Self) -> bool {
        self.get_address() == other.get_address() && self.area_type() == other.area_type()
    }
}

impl From<Box<dyn Address>> for u16 {
    fn from(addr: Box<dyn Address>) -> Self {
        (addr.get_address() as u16) + u16::from(addr.area_type())
    }
}
