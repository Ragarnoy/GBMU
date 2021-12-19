/// Identify a type of a tick.
#[derive(Debug, PartialEq, Eq)]
pub enum Tick {
    /// a CPU tick, executing once per clock cycle.
    MCycle,
    /// a PPU tick, executing 4 times per clock cycle.
    TCycle,
}

impl From<Tick> for u8 {
    fn from(cycle: Tick) -> u8 {
        match cycle {
            Tick::TCycle => 4,
            Tick::MCycle => 1,
        }
    }
}
