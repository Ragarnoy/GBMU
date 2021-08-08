#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRelativeAddress(usize),
    InvalidAbsoluteAddress(u16),
    InvalidPC(u16),
}
