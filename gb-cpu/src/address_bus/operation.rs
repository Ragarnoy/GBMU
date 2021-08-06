use super::{Address, Error};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        let _v = v;
        Err(Error::SegmentationFault(addr))
    }

    fn read(&self, addr: Address) -> Result<u8, Error>;
}

/// A Char Device yield current setted byte
pub struct CharDevice(pub u8);

impl FileOperation for CharDevice {
    fn write(&mut self, v: u8, _addr: Address) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: Address) -> Result<u8, Error> {
        Ok(self.0)
    }
}

#[test]
fn test_chardev_fileop() {
    use super::Area;

    let dev = CharDevice(42);
    let mut op: Box<dyn FileOperation> = Box::new(dev);

    assert_eq!(op.read(Address::from_offset(Area::Bios, 35, 24)), Ok(42));
    assert_eq!(op.write(5, Address::from_offset(Area::Bios, 4, 4)), Ok(()));
    assert_eq!(op.read(Address::from_offset(Area::Bios, 5, 2)), Ok(5));
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

impl FileOperation for RandomDevice {
    fn read(&self, _addr: Address) -> Result<u8, Error> {
        Ok(self.gen.borrow_mut().gen::<u8>())
    }

    fn write(&mut self, _v: u8, addr: Address) -> Result<(), Error> {
        Err(Error::SegmentationFault(addr))
    }
}
