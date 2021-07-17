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

#[derive(Debug, PartialEq, Eq)]
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
		println!("chunk: {:?}", chunk);
		Header::from_chunk(chunk)
	}

	pub fn from_chunk(chunk: [u8; 80]) -> Result<Self, Error> {
		RawHeader::from(&chunk).try_into()
	}
}

#[cfg(test)]
mod test_from_chunk {
	use super::{
		CartridgeType, CgbFlag, DestinationCode, Header, NewLicenseCode, OldLicenseCode, RamSize,
		RomSize, SgbFlag, Title,
	};

	#[test]
	fn cgb_header() {
		assert_eq!(
			Header::from_chunk([
				0, 195, 198, 5, 206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13,
				0, 8, 17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187,
				103, 99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62, 80, 79, 75, 69,
				77, 79, 78, 95, 83, 76, 86, 65, 65, 88, 70, 128, 48, 49, 3, 16, 6, 3, 1, 51, 0, 41,
				251, 140
			]),
			Ok(Header {
				entry_point: [0, 195, 198, 5],
				nitendo_logo: [
					206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8, 17, 31,
					136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103, 99,
					110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62
				],
				title: Title::Advanced {
					title: "POKEMON_SL".to_string(),
					manufacturer: "VAAX".to_string(),
					cgb_flag: CgbFlag::RetroCompatible
				},
				new_license_code: NewLicenseCode::NintendoRnD1,
				sgb_flag: SgbFlag::Supported,
				cartridge_type: CartridgeType::Mbc3TimerRamBattery2,
				rom_size: RomSize::MByte2,
				ram_size: RamSize::KByte32,
				destination_code: DestinationCode::Other,
				old_license_code: OldLicenseCode::UseNewLicenseCode,
				rom_version: 0,
				header_checksum: 41,
				global_checksum: 36091
			})
		);
	}

	#[test]
	fn basic_header() {
		assert_eq!(
			Header::from_chunk([
				0, 195, 80, 1, 206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0,
				8, 17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103,
				99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62, 84, 69, 84, 82, 73,
				83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 11, 137, 181
			]),
			Ok(Header {
				entry_point: [0, 195, 80, 1],
				nitendo_logo: [
					206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8, 17, 31,
					136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103, 99,
					110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62
				],
				title: Title::Simple("TETRIS".to_string()),
				new_license_code: NewLicenseCode::None,
				sgb_flag: SgbFlag::Unsupported,
				cartridge_type: CartridgeType::RomOnly,
				rom_size: RomSize::KByte32,
				ram_size: RamSize::NoRamOrUnused,
				destination_code: DestinationCode::Japan,
				old_license_code: OldLicenseCode::Nintendo,
				rom_version: 0,
				header_checksum: 11,
				global_checksum: 46473
			})
		);
	}

	#[test]
	fn advanced_title_with_manufacturer_garbadge() {
		assert_eq!(
			Header::from_chunk([
				0, 195, 80, 1, 206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0,
				8, 17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103,
				99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62, 90, 69, 76, 68, 65,
				32, 68, 73, 78, 0, 0, 65, 90, 55, 80, 192, 48, 49, 0, 27, 6, 2, 1, 51, 0, 226, 224,
				153
			]),
			Ok(Header {
				entry_point: [0, 195, 80, 1],
				nitendo_logo: [
					206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8, 17, 31,
					136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103, 99,
					110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62
				],
				title: Title::Advanced {
					title: "ZELDA DIN".to_string(),
					manufacturer: "AZ7".to_string(),
					cgb_flag: CgbFlag::CgbOnly
				},
				new_license_code: NewLicenseCode::NintendoRnD1,
				sgb_flag: SgbFlag::Unsupported,
				cartridge_type: CartridgeType::Mbc5RamBattery,
				rom_size: RomSize::MByte2,
				ram_size: RamSize::KByte8,
				destination_code: DestinationCode::Other,
				old_license_code: OldLicenseCode::UseNewLicenseCode,
				rom_version: 0,
				header_checksum: 226,
				global_checksum: 39392
			})
		);
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

#[derive(Debug, PartialEq, Eq)]
pub enum Title {
	Simple(String),
	Advanced {
		title: String,
		manufacturer: String,
		cgb_flag: CgbFlag,
	},
}

impl TryFrom<[u8; 16]> for Title {
	type Error = Error;

	fn try_from(raw: [u8; 16]) -> Result<Self, Self::Error> {
		if raw[15] == 0 {
			Ok(Title::Simple(
				String::from_utf8(raw.into())?
					.trim_end_matches(char::from(0))
					.to_string(),
			))
		} else {
			Ok(Title::Advanced {
				title: String::from_utf8(raw[0..10].into())?
					.trim_end_matches(char::from(0))
					.to_string(),
				manufacturer: String::from_utf8(raw[10..14].into())?
					.trim_matches(char::from(0))
					.to_string(),
				cgb_flag: raw[15].try_into()?,
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
