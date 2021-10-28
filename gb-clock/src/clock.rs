use crate::ticker::{cycle, Ticker};
use gb_bus::Bus;

/// Ensure that the various process unit execute their instructions in the right order.
pub struct Clock {
    pub curr_frame_cycle: usize,
}

/// A single clock cycle, during which each [Ticker] will tick 1 or 4 times depending on their [Tick](crate::Tick) type.
///
/// Its return value indicate if the current frame is incomplete.
#[macro_export]
macro_rules! cycle {
    ($self:expr, $bus:expr, $ticker:ident) => {{
        $self.tick($bus, $ticker);
        $self.curr_frame_cycle += 1;
        !$self.frame_ready()
    }};
    ($self:expr, $bus:expr, $ticker:ident, $($tickers:ident),+) => {{
            $self.tick($bus, $ticker);
            cycle!($self, $bus, $($tickers), +)
    }};
}

impl Clock {
    /// The amount of cycles to execute per frame.
    pub const CYCLES_PER_FRAME: usize = 17556;

    /// Execute a tick for this component.
    pub fn tick(&self, addr_bus: &mut impl Bus<u8>, ticker: &mut dyn Ticker) {
        cycle(ticker, addr_bus);
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
