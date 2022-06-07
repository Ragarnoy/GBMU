use crate::{Address, Area, Error, FileOperation, IORegArea, Source};

/// A device that always panic when interacting with it
#[derive(Default)]
pub struct PanicDevice(pub Option<&'static str>);

impl PanicDevice {
    pub fn named(name: &'static str) -> Self {
        Self(Some(name))
    }
}

impl<A> FileOperation<A, Area> for PanicDevice
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        panic!(
            "writing to a panic device ({}), v={:x}, addr={:?}",
            self.0.unwrap_or("no_name"),
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        panic!(
            "reading to a panic device ({}), addr={:?}",
            self.0.unwrap_or("no_name"),
            u16::from(addr)
        );
    }
}

impl<A> FileOperation<A, IORegArea> for PanicDevice
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        panic!(
            "writing to a panic device ({}), v={:x}, addr={:?}",
            self.0.unwrap_or("no_name"),
            v,
            u16::from(addr)
        );
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        panic!(
            "reading to a panic device ({}), addr={:?}",
            self.0.unwrap_or("no_name"),
            u16::from(addr)
        );
    }
}

#[test]
#[should_panic]
fn test_reading_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice::default();
    let op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(op.read(Addr::from_offset(Area::Rom, 35, 24), None), Ok(42));
}

#[test]
#[should_panic]
fn test_writing_panic_device() {
    use crate::address::Addr;
    use crate::Area;

    let dev = PanicDevice::default();
    let mut op: Box<dyn FileOperation<Addr<Area>, Area>> = Box::new(dev);

    assert_eq!(
        op.write(5, Addr::from_offset(Area::Rom, 4, 4), None),
        Ok(())
    );
}
