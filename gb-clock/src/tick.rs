#[derive(PartialEq, Eq)]
/// Identify a type of a tick.
pub enum Tick {
    /// a CPU tick, executing once per clock cycle.
    TCycle,
    /// a PPU tick, executing 4 times per clock cycle.
    MCycle,
}

impl From<Tick> for u8 {
    fn from(cycle: Tick) -> u8 {
        match cycle {
            Tick::TCycle => 1,
            Tick::MCycle => 4,
        }
    }
}
