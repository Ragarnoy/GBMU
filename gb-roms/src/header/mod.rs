mod cartridge_type;
mod destination_code;
mod error;
mod flag;
mod license_code;
mod size;

use std::convert::{From, TryFrom, TryInto};

use cartridge_type::CartridgeType;
use destination_code::DestinationCode;
pub use error::Error;
use flag::{CgbFlag, SgbFlag};
use license_code::{NewLicenseCode, OldLicenseCode};
use size::{RamSize, RomSize};

#[derive(Debug)]
pub struct Header {
	pub entry_point: [u8; 4],
	pub nitendo_logo: [u8; 48],
	pub title: Title,
	pub new_license_code: NewLicenseCode,
	pub sgb_flag: SgbFlag,
	pub cartridge_type: CartridgeType,
	pub rom_size: RomSize,
	pub ram_size: RamSize,
	pub destination_code: DestinationCode,
	pub old_license_code: OldLicenseCode,
	pub rom_version: u8,
	pub header_checksum: u8,
	pub global_checksum: u16,
}

impl Header {
	pub fn from_file(file: std::fs::File) -> Result<Self, Error> {
		use std::io::{Read, Seek, SeekFrom};

		let mut f = file;
		let mut chunk = [0_u8; 80];

		f.seek(SeekFrom::Start(0x100))
			.expect("cannot seek file to header");
		f.read(&mut chunk).expect("cannot read header");
		Header::from_chunk(chunk)
	}

	pub fn from_chunk(chunk: [u8; 80]) -> Result<Self, Error> {
		RawHeader::from(&chunk).try_into()
	}
}

impl TryFrom<RawHeader> for Header {
	type Error = Error;

	fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
		Ok(Self {
			entry_point: raw.entry_point,
			nitendo_logo: raw.nitendo_logo,
			title: raw.title.try_into()?,
			new_license_code: NewLicenseCode::try_from(
				String::from_utf8(raw.new_license_code.into())?.as_str(),
			)?,
			sgb_flag: raw.sgb_flag.try_into()?,
			cartridge_type: raw.cartridge_type.try_into()?,
			rom_size: raw.rom_size.try_into()?,
			ram_size: raw.ram_size.try_into()?,
			destination_code: raw.destination_code.try_into()?,
			old_license_code: raw.old_license_code.try_into()?,
			rom_version: raw.rom_version,
			header_checksum: raw.header_checksum,
			global_checksum: u16::from_le_bytes(raw.global_checksum),
		})
	}
}

#[derive(Debug)]
pub enum Title {
	Simple(String),
	Advanced {
		title: String,
		manufacturer: String,
		cbg_flag: CgbFlag,
	},
}

impl TryFrom<[u8; 16]> for Title {
	type Error = Error;

	fn try_from(raw: [u8; 16]) -> Result<Self, Self::Error> {
		if raw[15] == 0 {
			Ok(Title::Simple(String::from_utf8(raw.into())?))
		} else {
			Ok(Title::Advanced {
				title: String::from_utf8(raw[0..10].into())?,
				manufacturer: String::from_utf8(raw[10..14].into())?,
				cbg_flag: raw[15].try_into()?,
			})
		}
	}
}

/// The cartridge Header is at 0x100-0x14f
#[derive(Debug)]
#[repr(C)]
pub struct RawHeader {
	pub entry_point: [u8; 4],
	pub nitendo_logo: [u8; 48],
	pub title: [u8; 16],
	pub new_license_code: [u8; 2],
	pub sgb_flag: u8,
	pub cartridge_type: u8,
	pub rom_size: u8,
	pub ram_size: u8,
	pub destination_code: u8,
	pub old_license_code: u8,
	pub rom_version: u8,
	pub header_checksum: u8,
	pub global_checksum: [u8; 2],
}

impl From<&[u8; 80]> for RawHeader {
	fn from(chunk: &[u8; 80]) -> Self {
		Self {
			entry_point: <[u8; 4]>::try_from(&chunk[..4]).unwrap(),
			nitendo_logo: <[u8; 48]>::try_from(&chunk[4..52]).unwrap(),
			title: <[u8; 16]>::try_from(&chunk[52..68]).unwrap(),
			new_license_code: <[u8; 2]>::try_from(&chunk[68..70]).unwrap(),
			sgb_flag: chunk[70],
			cartridge_type: chunk[71],
			rom_size: chunk[72],
			ram_size: chunk[73],
			destination_code: chunk[74],
			old_license_code: chunk[75],
			rom_version: chunk[76],
			header_checksum: chunk[77],
			global_checksum: <[u8; 2]>::try_from(&chunk[78..80]).unwrap(),
		}
	}
}

#[test]
fn test_raw_header_size() {
	assert_eq!(std::mem::size_of::<RawHeader>(), 80)
}
