#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidAddress(usize),
}
