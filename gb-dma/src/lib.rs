use gb_bus::{Address, Area, Bus, Error, FileOperation, IORegArea, Lock};
use gb_clock::{Tick, Ticker};

#[derive(Default)]
pub struct Dma {
    oam_register: u8,
    oam_transfer: Option<u16>,
}

impl Dma {
    pub fn new() -> Dma {
        Dma {
            oam_register: 0,
            oam_transfer: None,
        }
    }
}

impl FileOperation<IORegArea> for Dma {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        if let IORegArea::OamDma = addr.area_type() {
            Ok(self.oam_register)
        } else {
            Err(Error::SegmentationFault(addr.into()))
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        if let IORegArea::OamDma = addr.area_type() {
            self.oam_register = v;
            self.oam_transfer = Some(0);
            Ok(())
        } else {
            Err(Error::SegmentationFault(addr.into()))
        }
    }
}

impl Ticker for Dma {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if let Some(step) = self.oam_transfer {
            if step == 0 {
                adr_bus.lock(Area::Oam, Lock::Dma);
            }
            let src: u8 = adr_bus
                .read(((self.oam_register as u16) << 8) + step, Some(Lock::Dma))
                .expect("memory unavailable during OAM DMA");
            if adr_bus.write(0xFE00 + step, src, Some(Lock::Dma)).is_err() {
                log::error!("failed to write data during OAM DMA");
            }
            let next_step = step + 1;
            if next_step <= 160 {
                self.oam_transfer = Some(next_step);
            } else {
                self.oam_transfer = None;
                adr_bus.unlock(Area::Oam);
            }
        }
    }
}
