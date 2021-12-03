#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Until {
    Step(usize),
    Instruction(String),
    Frame(usize),
    Second(usize),
    Null,
}
