#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Until {
    Cycle(usize),
    Step(usize),
    Frame(usize),
    Second(usize),
    Null,
}
