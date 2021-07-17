#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	InvalidRegisterValue(u8),
}
