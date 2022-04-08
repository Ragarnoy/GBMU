use gb_bus::{Address, Area, Bus, Error, FileOperation, IORegArea, Source};
use gb_clock::{Tick, Ticker};
use gb_ppu::PPUMem;

pub struct Dma {
    oam_register: u8,
    oam_transfer: Option<u16>,
    ppu_mem: PPUMem,
}

impl Dma {
    pub fn new(ppu_mem: PPUMem) -> Dma {
        Dma {
            oam_register: 0,
            oam_transfer: None,
            ppu_mem,
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Dma
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, _addr: A, _source: Option<Source>) -> Result<u8, Error> {
        Ok(self.oam_register)
    }

    fn write(&mut self, v: u8, _addr: A, _source: Option<Source>) -> Result<(), Error> {
        self.oam_register = v;
        self.oam_transfer = Some(0);
        Ok(())
    }
}

impl Ticker for Dma {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if let Some(step) = self.oam_transfer {
            if step == 0 {
                self.ppu_mem.lock(Area::Oam, Source::Dma);
            }
            let src: u8 = adr_bus
                .read(((self.oam_register as u16) << 8) + step, Some(Source::Dma))
                .expect("memory unavailable during OAM DMA");
            if adr_bus
                .write(0xFE00 + step, src, Some(Source::Dma))
                .is_err()
            {
                log::error!(
                    "failed to write data '{:x}' at '{:x}' during OAM DMA",
                    src,
                    0xFE00 + step
                );
            }
            let next_step = step + 1;
            if next_step < 160 {
                self.oam_transfer = Some(next_step);
            } else {
                self.oam_transfer = None;
                self.ppu_mem.unlock(Area::Oam);
            }
        }
    }
}
