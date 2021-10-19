use gb_bus::Bus;
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

    fn tick<B>(&mut self, addr_bus: &mut B)
    where
        B: Bus<u8> + Bus<u16>,
    {
        self.system_clock += self.cycle_count().into();
    }
}
