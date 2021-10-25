use crate::{Address, Area, Error, FileOperation, IORegArea};

/// A Char Device yield current setted byte
#[derive(Default)]
pub struct CharDevice(pub u8);

impl FileOperation<Area> for CharDevice {
    fn write(&mut self, v: u8, _addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        Ok(self.0)
    }
}

impl FileOperation<IORegArea> for CharDevice {
    fn write(&mut self, v: u8, _addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        Ok(self.0)
    }
}

#[test]
fn test_chardev_fileop() {
    use crate::address::Address;
    use crate::Area;

    let dev = CharDevice(42);
    let mut op: Box<dyn FileOperation<Area>> = Box::new(dev);

    assert_eq!(
        op.read(Box::new(Address::from_offset(Area::Bios, 35, 24))),
        Ok(42)
    );
    assert_eq!(
        op.write(5, Box::new(Address::from_offset(Area::Bios, 4, 4))),
        Ok(())
    );
    assert_eq!(
        op.read(Box::new(Address::from_offset(Area::Bios, 5, 2))),
        Ok(5)
    );
}
