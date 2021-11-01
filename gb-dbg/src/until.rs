#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Until {
    Step(usize),
    Frame(usize),
    Second(usize),
    Null,
}
