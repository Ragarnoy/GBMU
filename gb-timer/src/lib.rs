use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::Ticker;

#[derive(Default)]
pub struct Timer {
    system_clock: u16,
    tima: u8,
    tma: u8,
    tac: u8,
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
            IORegArea::TimerCounter => Ok(self.tima),
            IORegArea::TimerModulo => Ok(self.tma),
            IORegArea::TimerControl => Ok(self.tac),
            _ => Err(Error::new_bus_error(addr)),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::DivTimer => self.system_clock = 0,
            IORegArea::TimerCounter => self.tima = v,
            IORegArea::TimerModulo => self.tma = v,
            IORegArea::TimerControl => self.tac = v,
            _ => return Err(Error::new_bus_error(addr)),
        }
        Ok(())
    }
}
