#[derive(Default, Debug, PartialEq, Eq)]
pub struct Flag {
    pub half_carry: bool,
    pub carry: bool,
    pub negative: bool,
    pub zero: bool,
}
