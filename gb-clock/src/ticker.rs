use crate::Tick;

/// Define the behavior of a process unit on a single tick.
///
/// Each component implementing this trait will tick a fixed count T of times per clock cycle.
pub trait Ticker {
    /// Return a [Tick] to identify the count T of tick amount per clock cycle.
    fn cycle_count(&self) -> Tick;

    /// The behavior called T times per clock cycle.
    fn tick(&mut self);
}
