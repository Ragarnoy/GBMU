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

    pub fn div(&self) -> u8 {
        self.system_clock.to_le_bytes()[1]
    }

    fn edge_detector_timer(&self) -> bool {
        let mask: u16 = match self.tac & 0b11 {
            0b01 => 0x10,
            0b10 => 0x40,
            0b11 => 0x100,
            _ => 0x400,
        };
        self.system_clock & mask != 0
    }
}

impl Ticker for Timer {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        let old_bit = self.edge_detector_timer();
        self.system_clock = self.system_clock.wrapping_add(4);
        let new_bit = self.edge_detector_timer();

        log::trace!(
            "timer={:x?}, old_bit={}, new_bit={}",
            self,
            old_bit,
            new_bit
        );
        if (self.tac & 0b100) != 0 && old_bit && !new_bit {
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
            IORegArea::DivTimer => Ok(self.div()),
            IORegArea::TimerCounter => Ok(self.tima),
            IORegArea::TimerModulo => Ok(self.tma),
            IORegArea::TimerControl => Ok(!0b111 | self.tac),
            _ => Err(Error::bus_error(addr)),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::DivTimer => self.system_clock = 0,
            IORegArea::TimerCounter => self.tima = v,
            IORegArea::TimerModulo => self.tma = v,
            IORegArea::TimerControl => self.tac = v & 0b111,
            _ => return Err(Error::bus_error(addr)),
        }
        Ok(())
    }
}
