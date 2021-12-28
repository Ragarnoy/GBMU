use crate::{Addr, Area, Error, FileOperation, IORegArea, InternalLock, Lock, MemoryLock};

/// A device that always panic when interracting with it
#[derive(Default)]
pub struct PanicDevice;

impl FileOperation<Addr<Area>, Area> for PanicDevice {
    fn write(&mut self, v: u8, addr: Addr<Area>) -> Result<(), Error> {
        panic!(
            "writing to a panic device, v={:x}, addr={:?}",
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: Addr<Area>) -> Result<u8, Error> {
        panic!("reading to a panic device, addr={:?}", u16::from(addr));
    }
}

impl FileOperation<Addr<IORegArea>, IORegArea> for PanicDevice {
    fn write(&mut self, v: u8, addr: Addr<IORegArea>) -> Result<(), Error> {
        panic!(
            "writing to a panic device, v={:x}, addr={:?}",
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: Addr<IORegArea>) -> Result<u8, Error> {
        panic!("reading to a panic device, addr={:?}", u16::from(addr));
    }
}

impl MemoryLock for PanicDevice {
    fn lock(&mut self, _area: Area, _lock: Lock) {}

    fn unlock(&mut self, _area: Area) {}

    fn is_available(&self, _area: Area, _lock_key: Option<Lock>) -> bool {
        true
    }
}

impl InternalLock<Addr<Area>, Area> for PanicDevice {}

#[test]
#[should_panic]
fn test_reading_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice;
    let op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(op.read(Addr::from_offset(Area::Rom, 35, 24)), Ok(42));
}

#[test]
#[should_panic]
fn test_writing_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice;
    let mut op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(op.write(5, Addr::from_offset(Area::Rom, 4, 4)), Ok(()));
}
