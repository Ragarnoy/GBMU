#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Until {
    Step(u16),
    Frame(u16),
    Second(u16),
    Instant,
}
