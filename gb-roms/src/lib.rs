use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
enum NewLicenseCode {
	None,
	NintendoRnD1,
	Capcom,
	ElectronicArts,
	HudsonSoft,
	BAi,
	Kss,
	Pow,
	PCMComplete,
	SanX,
	KemcoJapan,
	Seta,
	Viacom,
	Nintendo,
	Bandai,
	OceanAcclaim,
	Konami,
	Hector,
	Taito,
	Hudson,
	Banpresto,
	UbiSoft,
	Atlus,
	Malibu,
	Angel,
	BulletProof,
	Irem,
	Absolute,
	Acclaim,
	Activision,
	AmericanSammy,
	HiTechEntertainment,
	LJN,
	Matchbox,
	Mattel,
	MiltonBradley,
	Titus,
	Virgin,
	LucasArts,
	Ocean,
	Infogrames,
	Interplay,
	Broderbund,
	Sculptured,
	Sci,
	THQ,
	Accolade,
	Misawa,
	Lozc,
	TokumaShotenIntermedia,
	TsukudaOriginal,
	Chunsoft,
	VideoSystem,
	Varie,
	YonezawaSpal,
	Kaneko,
	PackInSoft,
}

// Convert New license code raw byte into enum
// value from [new licensee code](https://gbdev.io/pandocs/The_Cartridge_Header.html#0144-0145---new-licensee-code)
impl TryFrom<u8> for NewLicenseCode {
	type Error = String;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0x50 => Ok(NewLicenseCode::Absolute),
			0x51 => Ok(NewLicenseCode::Acclaim),
			0x79 => Ok(NewLicenseCode::Accolade),
			0x52 => Ok(NewLicenseCode::Activision),
			0x53 => Ok(NewLicenseCode::AmericanSammy),
			0x46 => Ok(NewLicenseCode::Angel),
			0x42 => Ok(NewLicenseCode::Atlus),

			0x19 => Ok(NewLicenseCode::BAi),
			0x39 => Ok(NewLicenseCode::Banpresto),
			0x32 => Ok(NewLicenseCode::Bandai),
			0x47 => Ok(NewLicenseCode::BulletProof),
			0x72 => Ok(NewLicenseCode::Broderbund),

			0x08 => Ok(NewLicenseCode::Capcom),
			0x91 => Ok(NewLicenseCode::Chunsoft),

			0x13 | 0x69 => Ok(NewLicenseCode::ElectronicArts),

			0x35 => Ok(NewLicenseCode::Hector),
			0x55 => Ok(NewLicenseCode::HiTechEntertainment),
			0x38 => Ok(NewLicenseCode::Hudson),
			0x18 => Ok(NewLicenseCode::HudsonSoft),

			0x70 => Ok(NewLicenseCode::Infogrames),
			0x71 => Ok(NewLicenseCode::Interplay),
			0x49 => Ok(NewLicenseCode::Irem),

			0x97 => Ok(NewLicenseCode::Kaneko),
			0x28 => Ok(NewLicenseCode::KemcoJapan),
			0x34 | 0x54 | 0xA4 => Ok(NewLicenseCode::Konami),
			0x20 => Ok(NewLicenseCode::Kss),

			0x56 => Ok(NewLicenseCode::LJN),
			0x83 => Ok(NewLicenseCode::Lozc),
			0x64 => Ok(NewLicenseCode::LucasArts),

			0x44 => Ok(NewLicenseCode::Malibu),
			0x57 => Ok(NewLicenseCode::Matchbox),
			0x58 => Ok(NewLicenseCode::Mattel),
			0x59 => Ok(NewLicenseCode::MiltonBradley),
			0x80 => Ok(NewLicenseCode::Misawa),

			0x31 => Ok(NewLicenseCode::Nintendo),
			0x01 => Ok(NewLicenseCode::NintendoRnD1),
			0x00 => Ok(NewLicenseCode::None),

			0x67 => Ok(NewLicenseCode::Ocean),
			0x33 | 0x93 => Ok(NewLicenseCode::OceanAcclaim),

			0x99 => Ok(NewLicenseCode::PackInSoft),
			0x24 => Ok(NewLicenseCode::PCMComplete),
			0x22 => Ok(NewLicenseCode::Pow),

			0x25 => Ok(NewLicenseCode::SanX),
			0x73 => Ok(NewLicenseCode::Sculptured),
			0x75 => Ok(NewLicenseCode::Sci),
			0x29 => Ok(NewLicenseCode::Seta),

			0x37 => Ok(NewLicenseCode::Taito),
			0x78 => Ok(NewLicenseCode::THQ),
			0x60 => Ok(NewLicenseCode::Titus),
			0x86 => Ok(NewLicenseCode::TokumaShotenIntermedia),
			0x87 => Ok(NewLicenseCode::TsukudaOriginal),

			0x41 => Ok(NewLicenseCode::UbiSoft),

			0x95 => Ok(NewLicenseCode::Varie),
			0x30 => Ok(NewLicenseCode::Viacom),
			0x92 => Ok(NewLicenseCode::VideoSystem),
			0x61 => Ok(NewLicenseCode::Virgin),

			0x96 => Ok(NewLicenseCode::YonezawaSpal),

			_ => Err(format!("unknow publisher with id {:2x}", v)),
		}
	}
}

#[test]
fn test_convertion_new_license_code() {
	assert_eq!(NewLicenseCode::try_from(0x00), Ok(NewLicenseCode::None));
	assert_eq!(
		NewLicenseCode::try_from(0x01),
		Ok(NewLicenseCode::NintendoRnD1)
	);
	assert_eq!(NewLicenseCode::try_from(0x08), Ok(NewLicenseCode::Capcom));
	assert_eq!(
		NewLicenseCode::try_from(0x13),
		Ok(NewLicenseCode::ElectronicArts)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x18),
		Ok(NewLicenseCode::HudsonSoft)
	);
	assert_eq!(NewLicenseCode::try_from(0x19), Ok(NewLicenseCode::BAi));
	assert_eq!(NewLicenseCode::try_from(0x20), Ok(NewLicenseCode::Kss));
	assert_eq!(NewLicenseCode::try_from(0x22), Ok(NewLicenseCode::Pow));
	assert_eq!(
		NewLicenseCode::try_from(0x24),
		Ok(NewLicenseCode::PCMComplete)
	);
	assert_eq!(NewLicenseCode::try_from(0x25), Ok(NewLicenseCode::SanX));
	assert_eq!(
		NewLicenseCode::try_from(0x28),
		Ok(NewLicenseCode::KemcoJapan)
	);
	assert_eq!(NewLicenseCode::try_from(0x29), Ok(NewLicenseCode::Seta));
	assert_eq!(NewLicenseCode::try_from(0x30), Ok(NewLicenseCode::Viacom));
	assert_eq!(NewLicenseCode::try_from(0x31), Ok(NewLicenseCode::Nintendo));
	assert_eq!(NewLicenseCode::try_from(0x32), Ok(NewLicenseCode::Bandai));
	assert_eq!(
		NewLicenseCode::try_from(0x33),
		Ok(NewLicenseCode::OceanAcclaim)
	);
	assert_eq!(NewLicenseCode::try_from(0x34), Ok(NewLicenseCode::Konami));
	assert_eq!(NewLicenseCode::try_from(0x35), Ok(NewLicenseCode::Hector));
	assert_eq!(NewLicenseCode::try_from(0x37), Ok(NewLicenseCode::Taito));
	assert_eq!(NewLicenseCode::try_from(0x38), Ok(NewLicenseCode::Hudson));
	assert_eq!(
		NewLicenseCode::try_from(0x39),
		Ok(NewLicenseCode::Banpresto)
	);
	assert_eq!(NewLicenseCode::try_from(0x41), Ok(NewLicenseCode::UbiSoft));
	assert_eq!(NewLicenseCode::try_from(0x42), Ok(NewLicenseCode::Atlus));
	assert_eq!(NewLicenseCode::try_from(0x44), Ok(NewLicenseCode::Malibu));
	assert_eq!(NewLicenseCode::try_from(0x46), Ok(NewLicenseCode::Angel));
	assert_eq!(
		NewLicenseCode::try_from(0x47),
		Ok(NewLicenseCode::BulletProof)
	);
	assert_eq!(NewLicenseCode::try_from(0x49), Ok(NewLicenseCode::Irem));
	assert_eq!(NewLicenseCode::try_from(0x50), Ok(NewLicenseCode::Absolute));
	assert_eq!(NewLicenseCode::try_from(0x51), Ok(NewLicenseCode::Acclaim));
	assert_eq!(
		NewLicenseCode::try_from(0x52),
		Ok(NewLicenseCode::Activision)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x53),
		Ok(NewLicenseCode::AmericanSammy)
	);
	assert_eq!(NewLicenseCode::try_from(0x54), Ok(NewLicenseCode::Konami));
	assert_eq!(
		NewLicenseCode::try_from(0x55),
		Ok(NewLicenseCode::HiTechEntertainment)
	);
	assert_eq!(NewLicenseCode::try_from(0x56), Ok(NewLicenseCode::LJN));
	assert_eq!(NewLicenseCode::try_from(0x57), Ok(NewLicenseCode::Matchbox));
	assert_eq!(NewLicenseCode::try_from(0x58), Ok(NewLicenseCode::Mattel));
	assert_eq!(
		NewLicenseCode::try_from(0x59),
		Ok(NewLicenseCode::MiltonBradley)
	);
	assert_eq!(NewLicenseCode::try_from(0x60), Ok(NewLicenseCode::Titus));
	assert_eq!(NewLicenseCode::try_from(0x61), Ok(NewLicenseCode::Virgin));
	assert_eq!(
		NewLicenseCode::try_from(0x64),
		Ok(NewLicenseCode::LucasArts)
	);
	assert_eq!(NewLicenseCode::try_from(0x67), Ok(NewLicenseCode::Ocean));
	assert_eq!(
		NewLicenseCode::try_from(0x69),
		Ok(NewLicenseCode::ElectronicArts)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x70),
		Ok(NewLicenseCode::Infogrames)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x71),
		Ok(NewLicenseCode::Interplay)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x72),
		Ok(NewLicenseCode::Broderbund)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x73),
		Ok(NewLicenseCode::Sculptured)
	);
	assert_eq!(NewLicenseCode::try_from(0x75), Ok(NewLicenseCode::Sci));
	assert_eq!(NewLicenseCode::try_from(0x78), Ok(NewLicenseCode::THQ));
	assert_eq!(NewLicenseCode::try_from(0x79), Ok(NewLicenseCode::Accolade));
	assert_eq!(NewLicenseCode::try_from(0x80), Ok(NewLicenseCode::Misawa));
	assert_eq!(NewLicenseCode::try_from(0x83), Ok(NewLicenseCode::Lozc));
	assert_eq!(
		NewLicenseCode::try_from(0x86),
		Ok(NewLicenseCode::TokumaShotenIntermedia)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x87),
		Ok(NewLicenseCode::TsukudaOriginal)
	);
	assert_eq!(NewLicenseCode::try_from(0x91), Ok(NewLicenseCode::Chunsoft));
	assert_eq!(
		NewLicenseCode::try_from(0x92),
		Ok(NewLicenseCode::VideoSystem)
	);
	assert_eq!(
		NewLicenseCode::try_from(0x93),
		Ok(NewLicenseCode::OceanAcclaim)
	);
	assert_eq!(NewLicenseCode::try_from(0x95), Ok(NewLicenseCode::Varie));
	assert_eq!(
		NewLicenseCode::try_from(0x96),
		Ok(NewLicenseCode::YonezawaSpal)
	);
	assert_eq!(NewLicenseCode::try_from(0x97), Ok(NewLicenseCode::Kaneko));
	assert_eq!(
		NewLicenseCode::try_from(0x99),
		Ok(NewLicenseCode::PackInSoft)
	);
	assert_eq!(NewLicenseCode::try_from(0xA4), Ok(NewLicenseCode::Konami));
}

/// The cartridge Header is at 0x100-0x14f
#[repr(C)]
struct RawHeader {
	pub entry_point: [u8; 4],
	pub nitendo_logo: [u8; 48],
	pub title: RawTitle,
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

#[repr(C)]
union RawTitle {
	title: [u8; 16],
	cgb_title: RawCGBTitle,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RawCGBTitle {
	title: [u8; 11],
	manufacturer_code: [u8; 4],
	cgb_flag: u8,
}

#[test]
fn test_raw_cgb_title_size() {
	assert_eq!(std::mem::size_of::<RawCGBTitle>(), 16)
}

#[test]
fn test_raw_header_size() {
	assert_eq!(std::mem::size_of::<RawHeader>(), 80)
}
