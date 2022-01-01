use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::Ticker;
#[cfg(test)]
mod test_timer;

#[derive(Default, Debug)]
pub struct Timer {
    system_clock: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    const TIMER_INT_MASK: u8 = 0b100;
    const TAC_MASK: u8 = 0b111;
    const TAC_ENABLED: u8 = 0b100;
    const TICK: gb_clock::Tick = gb_clock::Tick::MCycle;
    const INC_PER_TICK: u16 = 4;

    pub fn div(&self) -> u8 {
        self.system_clock.to_le_bytes()[1]
    }

    fn edge_detector_timer(&self) -> bool {
        let mask: u16 = match self.tac & 0b11 {
            0b01 => 0xf,
            0b10 => 0x3f,
            0b11 => 0xff,
            _ => 0x3ff,
        };
        self.system_clock & mask != 0
    }
}

impl Ticker for Timer {
    fn cycle_count(&self) -> gb_clock::Tick {
        Self::TICK
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        let old_bit = self.edge_detector_timer();
        self.system_clock = self.system_clock.wrapping_add(Self::INC_PER_TICK);
        let new_bit = self.edge_detector_timer();

        #[cfg(feature = "trace")]
        log::trace!(
            "timer={:x?}, old_bit={}, new_bit={}",
            self,
            old_bit,
            new_bit
        );
        if (self.tac & Self::TAC_ENABLED) != 0 && old_bit && !new_bit {
            let (new_tima, overflowing) = self.tima.overflowing_add(1);
            if overflowing {
                let int_mask = addr_bus.read(0xff0f, None).unwrap_or_else(|e| {
                    log::warn!("cannot read IF register: {:?}", e);
                    0
                });
                if let Err(err) = addr_bus.write(0xff0f, int_mask | Timer::TIMER_INT_MASK, None) {
                    log::warn!("failed to update interrupt bitfield: {:?}", err);
                }
                self.tima = self.tma
            } else {
                self.tima = new_tima
            }
        }
    }
}

impl FileOperation<IORegArea> for Timer {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::Div => Ok(self.div()),
            IORegArea::Tima => Ok(self.tima),
            IORegArea::Tma => Ok(self.tma),
            IORegArea::Tac => Ok(!Self::TAC_MASK | self.tac),
            _ => Err(Error::bus_error(addr)),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::Div => self.system_clock = 0,
            IORegArea::Tima => self.tima = v,
            IORegArea::Tma => self.tma = v,
            IORegArea::Tac => self.tac = v & Self::TAC_MASK,
            _ => return Err(Error::bus_error(addr)),
        }
        Ok(())
    }
}
