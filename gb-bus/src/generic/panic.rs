use crate::{Address, Area, Error, FileOperation, IORegArea, InternalLock, MemoryLock, Source};

/// A device that always panic when interracting with it
#[derive(Default)]
pub struct PanicDevice;

impl<A> FileOperation<A, Area> for PanicDevice
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        panic!(
            "writing to a panic device, v={:x}, addr={:?}",
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        panic!("reading to a panic device, addr={:?}", u16::from(addr));
    }
}

impl<A> FileOperation<A, IORegArea> for PanicDevice
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        panic!(
            "writing to a panic device, v={:x}, addr={:?}",
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        panic!("reading to a panic device, addr={:?}", u16::from(addr));
    }
}

impl MemoryLock for PanicDevice {
    fn lock(&mut self, _area: Area, _lock: Source) {}

    fn unlock(&mut self, _area: Area) {}

    fn is_available(&self, _area: Area, _lock_key: Option<Source>) -> bool {
        true
    }
}

impl<A> InternalLock<A, Area> for PanicDevice
where
    u16: From<A>,
    A: Address<Area>,
{
}

#[test]
#[should_panic]
fn test_reading_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice;
    let op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(op.read(Addr::from_offset(Area::Rom, 35, 24), None), Ok(42));
}

#[test]
#[should_panic]
fn test_writing_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice;
    let mut op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(
        op.write(5, Addr::from_offset(Area::Rom, 4, 4), None),
        Ok(())
    );
}
