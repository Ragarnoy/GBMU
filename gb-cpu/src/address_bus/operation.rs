use super::{Error, Position};

/// RomOperation basic trait to implement for a ROM Emulator.
/// Rom is generally Read-only so `write` is not often used
pub trait RomOperation {
    /// writing to rom can be use full for MBC controller to set their own registry
    fn write(&mut self, _v: u8, addr: Position) -> Result<(), Error> {
        Err(Error::SegmentationFault(addr.absolute))
    }

    /// read one byte of data from rom
    fn read(&self, addr: Position) -> Result<u8, Error>;
}

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Position) -> Result<(), Error>;
    fn read(&self, addr: Position) -> Result<u8, Error>;
}
