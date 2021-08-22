use crate::Area;
use crate::Error;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Box<dyn Address>) -> Result<(), Error> {
        let _v = v;
        Err(Error::SegmentationFault(addr))
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

impl std::fmt::Debug for dyn Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}({})", self.area_type(), self.get_address())
    }
}
