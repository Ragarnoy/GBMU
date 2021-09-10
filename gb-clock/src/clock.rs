use crate::Ticker;
use gb_bus::Bus;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Clock<B: Bus<u8> + Bus<u16>> {
    curr_frame_cycle: usize,
    phantom_bus: PhantomData<B>,
}

impl<B: Bus<u8> + Bus<u16>> Clock<B> {
    const CYCLES_PER_FRAME: usize = 17556;

    pub fn cycle(&mut self, adr_bus: &mut B, process_units: &mut Vec<&mut dyn Ticker<B>>) {
        for ticker in process_units {
            for _ in 0..ticker.cycle_count().into() {
                ticker.tick(adr_bus);
            }
        }
        self.curr_frame_cycle += 1;
    }

    pub fn frame(&mut self, adr_bus: &mut B, process_units: &mut Vec<&mut dyn Ticker<B>>) {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        while self.curr_frame_cycle < Self::CYCLES_PER_FRAME {
            self.cycle(adr_bus, process_units);
        }
        self.curr_frame_cycle = 0;
    }
}
