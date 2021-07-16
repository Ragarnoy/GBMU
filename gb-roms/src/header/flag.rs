use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum CgbFlag {
	// Rom support Cgb + old Game Boys
	RetroCompatible,
	// Rom support Cgb only
	CgbOnly,
}

impl TryFrom<u8> for CgbFlag {
	type Error = String;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0x80 => Ok(CgbFlag::RetroCompatible),
			0xC0 => Ok(CgbFlag::CgbOnly),
			_ => Err(format!("unknown cgb flag {:02x}", v)),
		}
	}
}

#[test]
fn test_convert_cgb_flag() {
	assert_eq!(CgbFlag::try_from(0x80), Ok(CgbFlag::RetroCompatible));
	assert_eq!(CgbFlag::try_from(0xC0), Ok(CgbFlag::CgbOnly));
}

#[derive(Debug, PartialEq, Eq)]
pub enum SgbFlag {
	Unsupported,
	Supported,
}

impl TryFrom<u8> for SgbFlag {
	type Error = String;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0x00 => Ok(SgbFlag::Unsupported),
			0x03 => Ok(SgbFlag::Supported),
			_ => Err(format!("unknown sgb flag {:02x}", v)),
		}
	}
}

#[test]
fn test_convert_sgb_flag() {
	assert_eq!(SgbFlag::try_from(0x00), Ok(SgbFlag::Unsupported));
	assert_eq!(SgbFlag::try_from(0x03), Ok(SgbFlag::Supported));
}
