use crate::{Address, Error};

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        let _v = v;
        Err(Error::SegmentationFault(addr))
    }

    fn read(&self, addr: Address) -> Result<u8, Error>;
}
