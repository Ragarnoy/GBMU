mod error;
// mod register;

use error::Error;
use modular_bitfield::{
	bitfield,
	specifiers::{B2, B3},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
	Jump(u16),
}

#[bitfield]
#[derive(Debug, PartialEq, Eq)]
pub struct OpcodeBits {
	z: B3,
	y: B3,
	x: B2,
}

pub struct OpcodeGenerator<'a, It>
where
	It: Iterator<Item = &'a u8>,
{
	stream: It,
}

impl<'a, It> OpcodeGenerator<'a, It>
where
	It: Iterator<Item = &'a u8>,
{
	fn decode_x(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.x() {
			// 0 => ,
			// 1 => ,
			// 2 => ,
			3 => self.decode_3_z(v, o),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_3_z(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.z() {
			3 => self.decode_3_3_y(v, o),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}

	fn decode_3_3_y(&mut self, v: u8, o: OpcodeBits) -> Result<Opcode, Error> {
		match o.y() {
			0 => unimplemented!(),
			_ => Err(Error::UnknownOpcode(v)),
		}
	}
}

impl<'a, It> Iterator for OpcodeGenerator<'a, It>
where
	It: Iterator<Item = &'a u8>,
{
	type Item = Result<Opcode, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.stream.next()?;
		Some(self.decode_x(*current, OpcodeBits::from_bytes([*current])))
	}
}

#[test]
fn test_convert_opcode() {
	let op: Option<Result<Opcode, Error>> = OpcodeGenerator {
		stream: vec![0x53].iter(),
	}
	.next();
	assert_eq!(op, Some(Ok(Opcode::Jump(0x150))))
}
