mod error;
mod register;

pub enum Opcode {}

pub struct OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	stream: It,
}

impl<It> Iterator for OpcodeGenerator<It>
where
	It: Iterator<Item = u8>,
{
	type Item = Opcode;

	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!();
	}
}
