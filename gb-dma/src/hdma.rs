use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};

enum Mode {
    Gdma,
    Hdma,
}

#[derive(Default)]
pub struct Hdma {
    src: u16,
    dest: u16,
    pub active: bool,
    len: u8,
    mode: Option<Mode>,
}

impl Hdma {
    pub fn active(&self) -> bool {
        self.active
    }
}

impl<A> FileOperation<A, IORegArea> for Hdma
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, gb_bus::Error> {
        match addr.area_type() {
            IORegArea::Hdma1 => Ok((self.src >> 8) as u8),
            IORegArea::Hdma2 => Ok(self.src as u8),
            IORegArea::Hdma3 => Ok((self.dest >> 8) as u8),
            IORegArea::Hdma4 => Ok(self.dest as u8),
            IORegArea::Hdma5 => Ok(if self.active { self.len } else { 0xFF }),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), gb_bus::Error> {
        Ok(())
    }
}

impl Ticker for Hdma {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if !self.active {
            return;
        }
    }
}
