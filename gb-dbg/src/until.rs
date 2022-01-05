#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Until {
    Step(usize),
    Instruction(usize),
    Frame(usize),
    Second(usize),
    Null,
}
