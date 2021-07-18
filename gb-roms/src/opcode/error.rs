#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	InvalidRegisterValue(u8),
	UnknownOpcode(u8),
	InvalideOpcode(u8),
}
