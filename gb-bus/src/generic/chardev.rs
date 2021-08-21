use crate::{Address, Error, FileOperation};

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
    use crate::Area;

    let dev = CharDevice(42);
    let mut op: Box<dyn FileOperation> = Box::new(dev);

    assert_eq!(op.read(Address::from_offset(Area::Bios, 35, 24)), Ok(42));
    assert_eq!(op.write(5, Address::from_offset(Area::Bios, 4, 4)), Ok(()));
    assert_eq!(op.read(Address::from_offset(Area::Bios, 5, 2)), Ok(5));
}
