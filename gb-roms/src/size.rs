use std::convert::{From, TryFrom};

const REF_SIZE: usize = 32_000;
const REF_BANK: usize = 2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum RomSize {
	KByte32,
	KByte64,
	KByte128,
	KByte256,
	KByte512,
	MByte1,
	MByte2,
	MByte4,
	MByte8,
}

impl RomSize {
	pub fn get_rom_size(&self) -> usize {
		REF_SIZE << *self as usize
	}

	pub fn get_bank_amounts(&self) -> usize {
		REF_BANK << *self as usize
	}
}

#[test]
fn test_rom_size() {
	assert_eq!(RomSize::KByte32.get_rom_size(), 32_000);
	assert_eq!(RomSize::KByte64.get_rom_size(), 64_000);
	assert_eq!(RomSize::KByte128.get_rom_size(), 128_000);
	assert_eq!(RomSize::KByte256.get_rom_size(), 256_000);
	assert_eq!(RomSize::KByte512.get_rom_size(), 512_000);
	assert_eq!(RomSize::MByte1.get_rom_size(), 1_024_000);
	assert_eq!(RomSize::MByte2.get_rom_size(), 2_048_000);
	assert_eq!(RomSize::MByte4.get_rom_size(), 4_096_000);
	assert_eq!(RomSize::MByte8.get_rom_size(), 8_192_000);
}

#[test]
fn test_bank_amous() {
	assert_eq!(RomSize::KByte32.get_bank_amounts(), 2);
	assert_eq!(RomSize::KByte64.get_bank_amounts(), 4);
	assert_eq!(RomSize::KByte128.get_bank_amounts(), 8);
	assert_eq!(RomSize::KByte256.get_bank_amounts(), 16);
	assert_eq!(RomSize::KByte512.get_bank_amounts(), 32);
	assert_eq!(RomSize::MByte1.get_bank_amounts(), 64);
	assert_eq!(RomSize::MByte2.get_bank_amounts(), 128);
	assert_eq!(RomSize::MByte4.get_bank_amounts(), 256);
	assert_eq!(RomSize::MByte8.get_bank_amounts(), 512);
}

impl TryFrom<u8> for RomSize {
	type Error = String;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0x00 => Ok(RomSize::KByte32),
			0x01 => Ok(RomSize::KByte64),
			0x02 => Ok(RomSize::KByte128),
			0x03 => Ok(RomSize::KByte256),
			0x04 => Ok(RomSize::KByte512),
			0x05 => Ok(RomSize::MByte1),
			0x06 => Ok(RomSize::MByte2),
			0x07 => Ok(RomSize::MByte4),
			0x08 => Ok(RomSize::MByte8),
			_ => Err(format!("unknown Rom size identifier {:02x}", v)),
		}
	}
}

#[test]
fn test_convert_rom_size() {
	assert_eq!(RomSize::try_from(0x00), Ok(RomSize::KByte32));
	assert_eq!(RomSize::try_from(0x01), Ok(RomSize::KByte64));
	assert_eq!(RomSize::try_from(0x02), Ok(RomSize::KByte128));
	assert_eq!(RomSize::try_from(0x03), Ok(RomSize::KByte256));
	assert_eq!(RomSize::try_from(0x04), Ok(RomSize::KByte512));
	assert_eq!(RomSize::try_from(0x05), Ok(RomSize::MByte1));
	assert_eq!(RomSize::try_from(0x06), Ok(RomSize::MByte2));
	assert_eq!(RomSize::try_from(0x07), Ok(RomSize::MByte4));
	assert_eq!(RomSize::try_from(0x08), Ok(RomSize::MByte8));
}
