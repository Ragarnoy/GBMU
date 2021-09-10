use crate::Ticker;
use gb_bus::Bus;
use std::marker::PhantomData;

const CYCLES_PER_FRAME: usize = 17556;

#[derive(Default)]
pub struct Clock<B: Bus<u8> + Bus<u16>> {
    phantom_bus: PhantomData<B>,
}

impl<B: Bus<u8> + Bus<u16>> Clock<B> {
    pub fn cycle(&self, adr_bus: &mut B, process_units: &mut Vec<&mut dyn Ticker<B>>) {
        for ticker in process_units {
            for _ in 0..ticker.cycle_count().into() {
                ticker.tick(adr_bus);
            }
        }
    }

    pub fn frame(&self, adr_bus: &mut B, process_units: &mut Vec<&mut dyn Ticker<B>>) {
        for _ in 0..CYCLES_PER_FRAME {
            self.cycle(adr_bus, process_units)
        }
    }
}
