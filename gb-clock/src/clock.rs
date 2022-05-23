/// Ensure that the various process unit execute their instructions in the right order.
#[derive(Default)]
pub struct Clock {
    pub curr_frame_cycle: usize,
}

/// A single clock cycle, during which each [Ticker] will tick 1 or 4 times depending on their [Tick](crate::Tick) type.
///
/// Its return value indicate if the current frame is incomplete.
#[macro_export]
macro_rules! not_counted_cycles {
    ($clock:expr, $addr_bus:expr, $($tickers:expr),+) => {{
       $(
            gb_clock::cycle($tickers, $addr_bus);
        )+
        !$clock.frame_ready()
    }};
}

#[macro_export]
macro_rules! counted_cycles {
    ($clock:expr, $addr_bus:expr, $($tickers:expr),+) => {{
        gb_clock::not_counted_cycles!($clock, $addr_bus, $($tickers),+);
        $clock.inc_frame()
    }};
}

impl Clock {
    /// The amount of cycles to execute per frame.
    pub const CYCLES_PER_FRAME: usize = 17556;

    pub fn inc_frame(&mut self) -> bool {
        self.curr_frame_cycle += 1;
        !self.frame_ready()
    }

    /// Indicate if the current frame has been completed or not.
    pub fn frame_ready(&mut self) -> bool {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        self.curr_frame_cycle == 0
    }
}
