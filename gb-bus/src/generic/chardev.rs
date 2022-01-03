use crate::{Address, Area, Error, FileOperation, IORegArea, InternalLock, Lock, MemoryLock};

/// A Char Device yield current setted byte
#[derive(Default)]
pub struct CharDevice(pub u8);

impl<A> FileOperation<A, Area> for CharDevice
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, v: u8, _addr: A) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: A) -> Result<u8, Error> {
        Ok(self.0)
    }
}

impl<A> FileOperation<A, IORegArea> for CharDevice
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, _addr: A) -> Result<(), Error> {
        self.0 = v;
        Ok(())
    }

    fn read(&self, _addr: A) -> Result<u8, Error> {
        Ok(self.0)
    }
}

impl MemoryLock for CharDevice {
    fn lock(&mut self, _area: Area, _lock: Lock) {}

    fn unlock(&mut self, _area: Area) {}

    fn is_available(&self, _area: Area, _lock_key: Option<Lock>) -> bool {
        true
    }
}

impl<A> InternalLock<A, Area> for CharDevice
where
    u16: From<A>,
    A: Address<Area>,
{
}

#[test]
fn test_chardev_fileop() {
    use crate::address::Addr;
    use crate::Area;

    let dev = CharDevice(42);
    let mut op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(op.read(Addr::from_offset(Area::Rom, 35, 24)), Ok(42));
    assert_eq!(op.write(5, Addr::from_offset(Area::Rom, 4, 4)), Ok(()));
    assert_eq!(op.read(Addr::from_offset(Area::Rom, 5, 2)), Ok(5));
}
