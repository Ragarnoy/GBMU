#[derive(Debug)]
pub enum Error {
    InvalidAbsoluteAddress(u16),
    InvalidPC(u16),
}
