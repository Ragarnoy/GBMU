use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::Ticker;

pub struct Timer {
    system_clock: u16,
}

impl Timer {
    pub fn div(&self) -> u8 {
        self.system_clock.to_be_bytes()[1]
    }
}

impl Ticker for Timer {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick<B>(&mut self, _addr_bus: &mut B)
    where
        B: Bus<u8> + Bus<u16>,
    {
        self.system_clock += u16::from(self.cycle_count() as u8);
    }
}

impl FileOperation<IORegArea> for Timer {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::DivTimer => Ok(self.div()),
            _ => Err(Error::new_bus_error(addr)),
        }
    }

    fn write(&mut self, _v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::DivTimer => self.system_clock = 0,
            _ => return Err(Error::new_bus_error(addr)),
        }
        Ok(())
    }
}
