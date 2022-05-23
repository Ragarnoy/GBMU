use crate::Tick;
use gb_bus::Bus;

/// Define the behavior of a process unit on a single tick.
///
/// Each component implementing this trait will tick a fixed count T of times per clock cycle.
pub trait Ticker {
    /// Return a [Tick] to identify the count T of tick amount per clock cycle.
    fn cycle_count(&self) -> Tick;

    /// The behavior called T times per clock cycle.
    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>);
}

/// Execute X cycle depending of [Tick] type of the implementation of [Ticker]
pub fn cycle(ticker: &mut dyn Ticker, adr_bus: &mut dyn Bus<u8>) {
    for _ in 0..ticker.cycle_count().into() {
        ticker.tick(adr_bus);
    }
}
