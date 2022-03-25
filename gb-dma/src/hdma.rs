use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};

#[derive(PartialEq)]
pub enum HdmaMode {
    Gdma,
    Hdma,
}

#[derive(Default)]
pub struct Hdma {
    src: u16,
    dest: u16,
    active: bool,
    len: u8,
    mode: Option<HdmaMode>,
}

impl Hdma {
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn mode(&self) -> &Option<HdmaMode> {
        &self.mode
    }

    fn data_transfer(&mut self, adr_bus: &mut dyn Bus<u8>) {}
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
            IORegArea::Hdma5 => Ok(self.len | if self.active { 0x00 } else { 0x80 }),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), gb_bus::Error> {
        match addr.area_type() {
            IORegArea::Hdma1 => {
                self.src = ((v as u16) << 8) | (self.src & 0x00FF);
                Ok(())
            }
            IORegArea::Hdma2 => {
                self.src = (self.src & 0xFF00) | ((v & 0xF0) as u16);
                Ok(())
            }
            IORegArea::Hdma3 => {
                self.dest = 0x8000 | (((v & 0x1f) as u16) << 8) | (self.dest & 0xFF);
                Ok(())
            }
            IORegArea::Hdma4 => {
                self.dest = (self.dest & 0xFF00) | ((v & 0xF0) as u16);
                Ok(())
            }
            IORegArea::Hdma5 => {
                if self.active && self.mode == Some(HdmaMode::Hdma) {
                    if v & 0x80 == 0 {
                        self.active = false;
                    };
                    return Ok(());
                }
                self.active = true;
                self.len = v & 0x7F;
                self.mode = match v & 0x80 {
                    0 => Some(HdmaMode::Gdma),
                    _ => Some(HdmaMode::Hdma),
                };
                Ok(())
            }
            _ => Err(Error::SegmentationFault(addr.into())),
        }
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
        self.data_transfer(adr_bus);
    }
}
