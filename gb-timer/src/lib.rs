use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::Ticker;
#[cfg(test)]
mod test_timer;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Timer {
    pub system_clock: u16,
    pub tima: u8,
    tma: u8,
    tac: u8,
    last_and_result: bool,
}

impl Timer {
    const TIMER_INT_MASK: u8 = 0b100;
    const TAC_MASK: u8 = 0b111;
    const TAC_ENABLED: u8 = 0b100;
    const TICK: gb_clock::Tick = gb_clock::Tick::TCycle;
    const INC_PER_TICK: u16 = 1;

    pub fn div(&self) -> u8 {
        self.system_clock.to_le_bytes()[1]
    }

    fn edge_detector_timer(&self) -> bool {
        let mask: u16 = match self.tac & 0b11 {
            0b00 => 1 << 9,
            0b01 => 1 << 3,
            0b10 => 1 << 5,
            0b11 => 1 << 7,
            _ => unreachable!("WTF"),
        };
        self.system_clock & mask != 0
    }

    fn increment_tima(&mut self, addr_bus: &mut dyn Bus<u8>) {
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

impl Ticker for Timer {
    fn cycle_count(&self) -> gb_clock::Tick {
        Self::TICK
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        self.system_clock = self.system_clock.wrapping_add(Self::INC_PER_TICK);
        let edge_bit = self.edge_detector_timer();
        let timer_enable = self.tac & Self::TAC_ENABLED != 0;
        let current_and_result = edge_bit && timer_enable;

        #[cfg(feature = "trace")]
        log::trace!(
            "timer={:x?}, last_and_result={}, current_and_result={}",
            self,
            self.last_and_result,
            current_and_result
        );
        if self.last_and_result && !current_and_result {
            self.increment_tima(addr_bus);
        }
        self.last_and_result = current_and_result;
    }
}

impl<A> FileOperation<A, IORegArea> for Timer
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::Div => Ok(self.div()),
            IORegArea::Tima => Ok(self.tima),
            IORegArea::Tma => Ok(self.tma),
            IORegArea::Tac => Ok(!Self::TAC_MASK | self.tac),
            _ => Err(Error::bus_error(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::Div => self.system_clock = 0,
            IORegArea::Tima => self.tima = v,
            IORegArea::Tma => self.tma = v,
            IORegArea::Tac => self.tac = v & Self::TAC_MASK,
            _ => return Err(Error::bus_error(addr.into())),
        }
        Ok(())
    }
}
