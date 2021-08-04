use super::{Error, Position};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;

/// RomOperation basic trait to implement for a ROM Emulator.
/// Rom is generally Read-only so `write` is not often used
pub trait RomOperation {
    /// writing to rom can be use full for MBC controller to set their own registry
    fn write_rom(&mut self, _v: u8, addr: Position) -> Result<(), Error> {
        Err(Error::SegmentationFault(addr.absolute))
    }

    /// read one byte of data from rom
    fn read_rom(&self, addr: Position) -> Result<u8, Error>;
}

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Position) -> Result<(), Error>;
    fn read(&self, addr: Position) -> Result<u8, Error>;
}

/// A Char Device yield current setted byte
pub struct CharDevice(pub u8);

impl RomOperation for CharDevice {
    fn write_rom(&mut self, v: u8, _addr: Position) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read_rom(&self, _addr: Position) -> Result<u8, Error> {
        Ok(self.0)
    }
}

#[test]
fn test_chardev_rom() {
    let dev = CharDevice(42);
    let mut op: Box<dyn RomOperation> = Box::new(dev);

    assert_eq!(op.read_rom(Position::new(35, 24)), Ok(42));
    assert_eq!(op.write_rom(5, Position::new(4, 4)), Ok(()));
    assert_eq!(op.read_rom(Position::new(5, 7)), Ok(5));
}

impl FileOperation for CharDevice {
    fn write(&mut self, v: u8, _addr: Position) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: Position) -> Result<u8, Error> {
        Ok(self.0)
    }
}

#[test]
fn test_chardev_fileop() {
    let dev = CharDevice(42);
    let mut op: Box<dyn FileOperation> = Box::new(dev);

    assert_eq!(op.read(Position::new(35, 24)), Ok(42));
    assert_eq!(op.write(5, Position::new(4, 4)), Ok(()));
    assert_eq!(op.read(Position::new(5, 7)), Ok(5));
}

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

impl RomOperation for RandomDevice {
    fn read_rom(&self, _addr: Position) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }
}

impl FileOperation for RandomDevice {
    fn read(&self, _addr: Position) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: Position) -> Result<(), Error> {
        Err(Error::SegmentationFault(addr.absolute))
    }
}
