use gb_bus::Address;
use gb_bus::{FileOperation, IORegArea};

pub struct Hdma {}

impl Hdma {}

impl<A> FileOperation<A, IORegArea> for Hdma
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, gb_bus::Error> {
        Ok(0)
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), gb_bus::Error> {
        Ok(())
    }
}
