use gb_bus::{Address, Area, Bus, Error, FileOperation, IORegArea, Source};
use gb_clock::{Tick, Ticker};
use gb_ppu::PPUMem;

pub struct Dma {
    pub state: State,
    ppu_mem: PPUMem,
}

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Default, Clone, Copy)]
pub struct State {
    oam_register: u8,
    oam_transfer: Option<u16>,
}

impl Dma {
    pub fn new(ppu_mem: PPUMem) -> Self {
        Self::with_state(State::default(), ppu_mem)
    }

    pub fn with_state(state: State, ppu_mem: PPUMem) -> Self {
        Self { state, ppu_mem }
    }
}

impl<A> FileOperation<A, IORegArea> for Dma
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, _addr: A, _source: Option<Source>) -> Result<u8, Error> {
        Ok(self.state.oam_register)
    }

    fn write(&mut self, v: u8, _addr: A, _source: Option<Source>) -> Result<(), Error> {
        self.state.oam_register = v;
        self.state.oam_transfer = Some(0);
        Ok(())
    }
}

impl Ticker for Dma {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if let Some(step) = self.state.oam_transfer {
            let src: u8 = adr_bus
                .read(
                    ((self.state.oam_register as u16) << 8) + step,
                    Some(Source::Dma),
                )
                .expect("mem.stateory unavailable during OAM DMA");
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
                self.state.oam_transfer = Some(next_step);
            } else {
                self.state.oam_transfer = None;
            }
        }
    }
}
