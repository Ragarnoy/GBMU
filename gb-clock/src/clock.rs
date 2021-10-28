use crate::ticker::{cycle, Ticker};
use gb_bus::Bus;
use std::iter::IntoIterator;

/// Ensure that the various process unit execute their instructions in the right order.
pub struct Clock {
    curr_frame_cycle: usize,
}

impl Clock {
    /// The amount of cycles to execute per frame.
    pub const CYCLES_PER_FRAME: usize = 17556;

    /// A single clock cycle, during which each [Ticker] will tick 1 or 4 times depending on their [Tick](crate::Tick) type.
    ///
    /// Its return value indicate if the current frame is incomplete.
    pub fn cycle(&mut self, addr_bus: &mut impl Bus<u8>, tickers: Vec<&mut dyn Ticker>) -> bool {
        for ticker in tickers.into_iter() {
            cycle(ticker, addr_bus);
        }
        self.curr_frame_cycle += 1;
        !self.frame_ready()
    }

    /// Indicate if the current frame has been completed or not.
    pub fn frame_ready(&mut self) -> bool {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        self.curr_frame_cycle == 0
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            curr_frame_cycle: 0,
        }
    }
}
