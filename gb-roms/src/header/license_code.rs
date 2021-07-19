use super::error::Error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum NewLicenseCode {
	Absolute,
	Acclaim,
	Accolade,
	Activision,
	AmericanSammy,
	Angel,
	Atlus,
	BAi,
	Bandai,
	Banpresto,
	Broderbund,
	BulletProof,
	Capcom,
	Chunsoft,
	ElectronicArts,
	Hector,
	HiTechEntertainment,
	Hudson,
	HudsonSoft,
	Infogrames,
	Interplay,
	Irem,
	Kaneko,
	KemcoJapan,
	Konami,
	Kss,
	LJN,
	Lozc,
	LucasArts,
	Malibu,
	Matchbox,
	Mattel,
	MiltonBradley,
	Misawa,
	Nintendo,
	NintendoRnD1,
	None,
	Ocean,
	OceanAcclaim,
	PCMComplete,
	PackInSoft,
	Pow,
	SanX,
	Sci,
	Sculptured,
	Seta,
	THQ,
	Taito,
	Titus,
	TokumaShotenIntermedia,
	TsukudaOriginal,
	UbiSoft,
	Varie,
	Viacom,
	VideoSystem,
	Virgin,
	YonezawaSpal,
}

// Convert New license code raw byte into enum
// value from [new licensee code](https://gbdev.io/pandocs/The_Cartridge_Header.html#0144-0145---new-licensee-code)
impl TryFrom<&str> for NewLicenseCode {
	type Error = Error;

	fn try_from(v: &str) -> Result<Self, Self::Error> {
		match v {
			"50" => Ok(NewLicenseCode::Absolute),
			"51" => Ok(NewLicenseCode::Acclaim),
			"79" => Ok(NewLicenseCode::Accolade),
			"52" => Ok(NewLicenseCode::Activision),
			"53" => Ok(NewLicenseCode::AmericanSammy),
			"46" => Ok(NewLicenseCode::Angel),
			"42" => Ok(NewLicenseCode::Atlus),

			"19" => Ok(NewLicenseCode::BAi),
			"39" => Ok(NewLicenseCode::Banpresto),
			"32" => Ok(NewLicenseCode::Bandai),
			"47" => Ok(NewLicenseCode::BulletProof),
			"72" => Ok(NewLicenseCode::Broderbund),

			"08" => Ok(NewLicenseCode::Capcom),
			"91" => Ok(NewLicenseCode::Chunsoft),

			"13" | "69" => Ok(NewLicenseCode::ElectronicArts),

			"35" => Ok(NewLicenseCode::Hector),
			"55" => Ok(NewLicenseCode::HiTechEntertainment),
			"38" => Ok(NewLicenseCode::Hudson),
			"18" => Ok(NewLicenseCode::HudsonSoft),

			"70" => Ok(NewLicenseCode::Infogrames),
			"71" => Ok(NewLicenseCode::Interplay),
			"49" => Ok(NewLicenseCode::Irem),

			"97" => Ok(NewLicenseCode::Kaneko),
			"28" => Ok(NewLicenseCode::KemcoJapan),
			"34" | "54" | "A4" => Ok(NewLicenseCode::Konami),
			"20" => Ok(NewLicenseCode::Kss),

			"56" => Ok(NewLicenseCode::LJN),
			"83" => Ok(NewLicenseCode::Lozc),
			"64" => Ok(NewLicenseCode::LucasArts),

			"44" => Ok(NewLicenseCode::Malibu),
			"57" => Ok(NewLicenseCode::Matchbox),
			"58" => Ok(NewLicenseCode::Mattel),
			"59" => Ok(NewLicenseCode::MiltonBradley),
			"80" => Ok(NewLicenseCode::Misawa),

			"31" => Ok(NewLicenseCode::Nintendo),
			"01" => Ok(NewLicenseCode::NintendoRnD1),
			"00" | "\0\0" => Ok(NewLicenseCode::None),

			"67" => Ok(NewLicenseCode::Ocean),
			"33" | "93" => Ok(NewLicenseCode::OceanAcclaim),

			"99" => Ok(NewLicenseCode::PackInSoft),
			"24" => Ok(NewLicenseCode::PCMComplete),
			"22" => Ok(NewLicenseCode::Pow),

			"25" => Ok(NewLicenseCode::SanX),
			"73" => Ok(NewLicenseCode::Sculptured),
			"75" => Ok(NewLicenseCode::Sci),
			"29" => Ok(NewLicenseCode::Seta),

			"37" => Ok(NewLicenseCode::Taito),
			"78" => Ok(NewLicenseCode::THQ),
			"60" => Ok(NewLicenseCode::Titus),
			"86" => Ok(NewLicenseCode::TokumaShotenIntermedia),
			"87" => Ok(NewLicenseCode::TsukudaOriginal),

			"41" => Ok(NewLicenseCode::UbiSoft),

			"95" => Ok(NewLicenseCode::Varie),
			"30" => Ok(NewLicenseCode::Viacom),
			"92" => Ok(NewLicenseCode::VideoSystem),
			"61" => Ok(NewLicenseCode::Virgin),

			"96" => Ok(NewLicenseCode::YonezawaSpal),
			_ => Err(Error::InvalidNewLicenseCode(v.into())),
		}
	}
}

#[test]
fn test_convert_new_license_code() {
	assert_eq!(NewLicenseCode::try_from("00"), Ok(NewLicenseCode::None));
	assert_eq!(NewLicenseCode::try_from("\0\0"), Ok(NewLicenseCode::None));
	assert_eq!(
		NewLicenseCode::try_from("01"),
		Ok(NewLicenseCode::NintendoRnD1)
	);
	assert_eq!(NewLicenseCode::try_from("08"), Ok(NewLicenseCode::Capcom));
	assert_eq!(
		NewLicenseCode::try_from("13"),
		Ok(NewLicenseCode::ElectronicArts)
	);
	assert_eq!(
		NewLicenseCode::try_from("18"),
		Ok(NewLicenseCode::HudsonSoft)
	);
	assert_eq!(NewLicenseCode::try_from("19"), Ok(NewLicenseCode::BAi));
	assert_eq!(NewLicenseCode::try_from("20"), Ok(NewLicenseCode::Kss));
	assert_eq!(NewLicenseCode::try_from("22"), Ok(NewLicenseCode::Pow));
	assert_eq!(
		NewLicenseCode::try_from("24"),
		Ok(NewLicenseCode::PCMComplete)
	);
	assert_eq!(NewLicenseCode::try_from("25"), Ok(NewLicenseCode::SanX));
	assert_eq!(
		NewLicenseCode::try_from("28"),
		Ok(NewLicenseCode::KemcoJapan)
	);
	assert_eq!(NewLicenseCode::try_from("29"), Ok(NewLicenseCode::Seta));
	assert_eq!(NewLicenseCode::try_from("30"), Ok(NewLicenseCode::Viacom));
	assert_eq!(NewLicenseCode::try_from("31"), Ok(NewLicenseCode::Nintendo));
	assert_eq!(NewLicenseCode::try_from("32"), Ok(NewLicenseCode::Bandai));
	assert_eq!(
		NewLicenseCode::try_from("33"),
		Ok(NewLicenseCode::OceanAcclaim)
	);
	assert_eq!(NewLicenseCode::try_from("34"), Ok(NewLicenseCode::Konami));
	assert_eq!(NewLicenseCode::try_from("35"), Ok(NewLicenseCode::Hector));
	assert_eq!(NewLicenseCode::try_from("37"), Ok(NewLicenseCode::Taito));
	assert_eq!(NewLicenseCode::try_from("38"), Ok(NewLicenseCode::Hudson));
	assert_eq!(
		NewLicenseCode::try_from("39"),
		Ok(NewLicenseCode::Banpresto)
	);
	assert_eq!(NewLicenseCode::try_from("41"), Ok(NewLicenseCode::UbiSoft));
	assert_eq!(NewLicenseCode::try_from("42"), Ok(NewLicenseCode::Atlus));
	assert_eq!(NewLicenseCode::try_from("44"), Ok(NewLicenseCode::Malibu));
	assert_eq!(NewLicenseCode::try_from("46"), Ok(NewLicenseCode::Angel));
	assert_eq!(
		NewLicenseCode::try_from("47"),
		Ok(NewLicenseCode::BulletProof)
	);
	assert_eq!(NewLicenseCode::try_from("49"), Ok(NewLicenseCode::Irem));
	assert_eq!(NewLicenseCode::try_from("50"), Ok(NewLicenseCode::Absolute));
	assert_eq!(NewLicenseCode::try_from("51"), Ok(NewLicenseCode::Acclaim));
	assert_eq!(
		NewLicenseCode::try_from("52"),
		Ok(NewLicenseCode::Activision)
	);
	assert_eq!(
		NewLicenseCode::try_from("53"),
		Ok(NewLicenseCode::AmericanSammy)
	);
	assert_eq!(NewLicenseCode::try_from("54"), Ok(NewLicenseCode::Konami));
	assert_eq!(
		NewLicenseCode::try_from("55"),
		Ok(NewLicenseCode::HiTechEntertainment)
	);
	assert_eq!(NewLicenseCode::try_from("56"), Ok(NewLicenseCode::LJN));
	assert_eq!(NewLicenseCode::try_from("57"), Ok(NewLicenseCode::Matchbox));
	assert_eq!(NewLicenseCode::try_from("58"), Ok(NewLicenseCode::Mattel));
	assert_eq!(
		NewLicenseCode::try_from("59"),
		Ok(NewLicenseCode::MiltonBradley)
	);
	assert_eq!(NewLicenseCode::try_from("60"), Ok(NewLicenseCode::Titus));
	assert_eq!(NewLicenseCode::try_from("61"), Ok(NewLicenseCode::Virgin));
	assert_eq!(
		NewLicenseCode::try_from("64"),
		Ok(NewLicenseCode::LucasArts)
	);
	assert_eq!(NewLicenseCode::try_from("67"), Ok(NewLicenseCode::Ocean));
	assert_eq!(
		NewLicenseCode::try_from("69"),
		Ok(NewLicenseCode::ElectronicArts)
	);
	assert_eq!(
		NewLicenseCode::try_from("70"),
		Ok(NewLicenseCode::Infogrames)
	);
	assert_eq!(
		NewLicenseCode::try_from("71"),
		Ok(NewLicenseCode::Interplay)
	);
	assert_eq!(
		NewLicenseCode::try_from("72"),
		Ok(NewLicenseCode::Broderbund)
	);
	assert_eq!(
		NewLicenseCode::try_from("73"),
		Ok(NewLicenseCode::Sculptured)
	);
	assert_eq!(NewLicenseCode::try_from("75"), Ok(NewLicenseCode::Sci));
	assert_eq!(NewLicenseCode::try_from("78"), Ok(NewLicenseCode::THQ));
	assert_eq!(NewLicenseCode::try_from("79"), Ok(NewLicenseCode::Accolade));
	assert_eq!(NewLicenseCode::try_from("80"), Ok(NewLicenseCode::Misawa));
	assert_eq!(NewLicenseCode::try_from("83"), Ok(NewLicenseCode::Lozc));
	assert_eq!(
		NewLicenseCode::try_from("86"),
		Ok(NewLicenseCode::TokumaShotenIntermedia)
	);
	assert_eq!(
		NewLicenseCode::try_from("87"),
		Ok(NewLicenseCode::TsukudaOriginal)
	);
	assert_eq!(NewLicenseCode::try_from("91"), Ok(NewLicenseCode::Chunsoft));
	assert_eq!(
		NewLicenseCode::try_from("92"),
		Ok(NewLicenseCode::VideoSystem)
	);
	assert_eq!(
		NewLicenseCode::try_from("93"),
		Ok(NewLicenseCode::OceanAcclaim)
	);
	assert_eq!(NewLicenseCode::try_from("95"), Ok(NewLicenseCode::Varie));
	assert_eq!(
		NewLicenseCode::try_from("96"),
		Ok(NewLicenseCode::YonezawaSpal)
	);
	assert_eq!(NewLicenseCode::try_from("97"), Ok(NewLicenseCode::Kaneko));
	assert_eq!(
		NewLicenseCode::try_from("99"),
		Ok(NewLicenseCode::PackInSoft)
	);
	assert_eq!(NewLicenseCode::try_from("A4"), Ok(NewLicenseCode::Konami));
}

#[derive(Debug, PartialEq, Eq)]
pub enum OldLicenseCode {
	AWave,
	Absolute,
	Acclaim,
	Accolade,
	Activision,
	Altron,
	AmericanSammy,
	Angel,
	Ape,
	Arc,
	AsciiOrNexoft,
	AskKodansha,
	Asmik,
	Athena,
	Atlus,
	Bandai,
	Banpresto,
	Broderbund,
	BulletProofSoftware,
	Capcom,
	ChunSoft,
	Clary,
	Coconuts,
	CopyaSystems,
	CultureBrainO,
	DataEast,
	ElectroBrain,
	ElectronicArts,
	EliteSystems,
	Enix,
	EntertainmentI,
	EpicSonyRecords,
	Epoch,
	ExtremeEntertainment,
	Gametek,
	Gremlin,
	Hal,
	Hector,
	HoriElectric,
	HotB,
	Hudsonsoft,
	Human,
	IMax,
	Igs,
	Imagineer,
	Infogrames,
	Interplay,
	Irem,
	ItcEntertainment,
	Jaleco,
	Kaneko,
	Kawada,
	Kemco,
	KingRecords,
	Koei,
	Konami,
	KotobukiSystems,
	Ljn,
	Lozc,
	Malibu,
	Matchbox,
	Meldac,
	Microprose,
	MiltonBradley,
	Mindscape,
	MisawaEntertainment,
	Namco,
	Natsume,
	NaxatSoft,
	Ncs,
	NihonBussan,
	Nintendo,
	None,
	Nova,
	Ocean,
	ParkPlace,
	PcmComplete,
	PonyCanyon,
	PonyCanyonOr,
	Quest,
	Romstar,
	Sammy,
	SanX,
	SculpteredSoft,
	UseNewLicenseCode,
	Seta,
	SigmaEnterprises,
	Snk,
	Sofel,
	SonyImagesoft,
	SpectrumHoloby,
	Squaresoft,
	Sunsoft,
	THq,
	Taito,
	Takara,
	TechnosJapan,
	Tecmo,
	TheSalesCurve,
	Titus,
	ToeiAnimation,
	Toho,
	TokumaShotenI,
	Tomy,
	TonkinHouse,
	Towachiki,
	Tradewest,
	TriffixEntertainment,
	Tsuburava,
	USGold,
	UbiSoft,
	Ufl,
	Ultra,
	Use,
	Uutaka,
	Vap,
	Varie,
	VicTokai,
	VideoSystem,
	Virgin,
	Yanoman,
	YonezawaSPal,
}

impl TryFrom<u8> for OldLicenseCode {
	type Error = Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0xF0 => Ok(OldLicenseCode::AWave),
			0x50 => Ok(OldLicenseCode::Absolute),
			0x51 | 0xB0 => Ok(OldLicenseCode::Acclaim),
			0x79 => Ok(OldLicenseCode::Accolade),
			0x52 => Ok(OldLicenseCode::Activision),
			0xDF => Ok(OldLicenseCode::Altron),
			0x53 => Ok(OldLicenseCode::AmericanSammy),
			0x46 | 0xCF => Ok(OldLicenseCode::Angel),
			0x8E => Ok(OldLicenseCode::Ape),
			0x99 => Ok(OldLicenseCode::Arc),
			0xB1 => Ok(OldLicenseCode::AsciiOrNexoft),
			0xD4 => Ok(OldLicenseCode::AskKodansha),
			0xE8 => Ok(OldLicenseCode::Asmik),
			0xE7 => Ok(OldLicenseCode::Athena),
			0x42 | 0xEB => Ok(OldLicenseCode::Atlus),
			0x32 | 0xA2 | 0xB2 => Ok(OldLicenseCode::Bandai),
			0x39 | 0x9D | 0xD9 => Ok(OldLicenseCode::Banpresto),
			0x72 | 0xAA => Ok(OldLicenseCode::Broderbund),
			0x8B => Ok(OldLicenseCode::BulletProofSoftware),
			0x08 | 0x38 => Ok(OldLicenseCode::Capcom),
			0x91 => Ok(OldLicenseCode::ChunSoft),
			0x1D => Ok(OldLicenseCode::Clary),
			0x0B => Ok(OldLicenseCode::Coconuts),
			0xD7 => Ok(OldLicenseCode::CopyaSystems),
			0xBA => Ok(OldLicenseCode::CultureBrainO),
			0xC5 => Ok(OldLicenseCode::DataEast),
			0x6F => Ok(OldLicenseCode::ElectroBrain),
			0x13 | 0x69 => Ok(OldLicenseCode::ElectronicArts),
			0x0C | 0x6E => Ok(OldLicenseCode::EliteSystems),
			0xB4 => Ok(OldLicenseCode::Enix),
			0x3C => Ok(OldLicenseCode::EntertainmentI),
			0xEC => Ok(OldLicenseCode::EpicSonyRecords),
			0xE5 => Ok(OldLicenseCode::Epoch),
			0xF3 => Ok(OldLicenseCode::ExtremeEntertainment),
			0x54 => Ok(OldLicenseCode::Gametek),
			0x3E => Ok(OldLicenseCode::Gremlin),
			0xB6 => Ok(OldLicenseCode::Hal),
			0x35 => Ok(OldLicenseCode::Hector),
			0xA1 => Ok(OldLicenseCode::HoriElectric),
			0x09 => Ok(OldLicenseCode::HotB),
			0x18 => Ok(OldLicenseCode::Hudsonsoft),
			0xDE => Ok(OldLicenseCode::Human),
			0x8F => Ok(OldLicenseCode::IMax),
			0xEE => Ok(OldLicenseCode::Igs),
			0x9C => Ok(OldLicenseCode::Imagineer),
			0x30 | 0x70 => Ok(OldLicenseCode::Infogrames),
			0x71 => Ok(OldLicenseCode::Interplay),
			0x49 => Ok(OldLicenseCode::Irem),
			0x19 => Ok(OldLicenseCode::ItcEntertainment),
			0x0A | 0xE0 => Ok(OldLicenseCode::Jaleco),
			0x97 => Ok(OldLicenseCode::Kaneko),
			0xA6 => Ok(OldLicenseCode::Kawada),
			0x7F | 0xC2 => Ok(OldLicenseCode::Kemco),
			0xEA => Ok(OldLicenseCode::KingRecords),
			0xC8 => Ok(OldLicenseCode::Koei),
			0x34 | 0xA4 => Ok(OldLicenseCode::Konami),
			0x28 => Ok(OldLicenseCode::KotobukiSystems),
			0x56 | 0xDB | 0xFF => Ok(OldLicenseCode::Ljn),
			0x83 => Ok(OldLicenseCode::Lozc),
			0x44 | 0x4D => Ok(OldLicenseCode::Malibu),
			0x57 => Ok(OldLicenseCode::Matchbox),
			0xCD => Ok(OldLicenseCode::Meldac),
			0x7C => Ok(OldLicenseCode::Microprose),
			0x59 => Ok(OldLicenseCode::MiltonBradley),
			0x5A => Ok(OldLicenseCode::Mindscape),
			0x80 => Ok(OldLicenseCode::MisawaEntertainment),
			0xAF => Ok(OldLicenseCode::Namco),
			0xE9 => Ok(OldLicenseCode::Natsume),
			0x5C | 0xD6 => Ok(OldLicenseCode::NaxatSoft),
			0xDD => Ok(OldLicenseCode::Ncs),
			0x9A => Ok(OldLicenseCode::NihonBussan),
			0x01 | 0x31 => Ok(OldLicenseCode::Nintendo),
			0x00 => Ok(OldLicenseCode::None),
			0x9F => Ok(OldLicenseCode::Nova),
			0x67 => Ok(OldLicenseCode::Ocean),
			0x55 => Ok(OldLicenseCode::ParkPlace),
			0x24 => Ok(OldLicenseCode::PcmComplete),
			0xB9 => Ok(OldLicenseCode::PonyCanyon),
			0xCE => Ok(OldLicenseCode::PonyCanyonOr),
			0xD2 => Ok(OldLicenseCode::Quest),
			0x5B => Ok(OldLicenseCode::Romstar),
			0xBF => Ok(OldLicenseCode::Sammy),
			0x25 => Ok(OldLicenseCode::SanX),
			0x73 => Ok(OldLicenseCode::SculpteredSoft),
			0x33 => Ok(OldLicenseCode::UseNewLicenseCode),
			0x29 => Ok(OldLicenseCode::Seta),
			0xD3 => Ok(OldLicenseCode::SigmaEnterprises),
			0xB7 => Ok(OldLicenseCode::Snk),
			0xD1 => Ok(OldLicenseCode::Sofel),
			0xBD => Ok(OldLicenseCode::SonyImagesoft),
			0x47 => Ok(OldLicenseCode::SpectrumHoloby),
			0xC3 => Ok(OldLicenseCode::Squaresoft),
			0xBB => Ok(OldLicenseCode::Sunsoft),
			0x78 => Ok(OldLicenseCode::THq),
			0xC0 => Ok(OldLicenseCode::Taito),
			0xD0 => Ok(OldLicenseCode::Taito),
			0xA7 => Ok(OldLicenseCode::Takara),
			0xA9 => Ok(OldLicenseCode::TechnosJapan),
			0x9B => Ok(OldLicenseCode::Tecmo),
			0x75 => Ok(OldLicenseCode::TheSalesCurve),
			0x60 => Ok(OldLicenseCode::Titus),
			0xAC => Ok(OldLicenseCode::ToeiAnimation),
			0xAD => Ok(OldLicenseCode::Toho),
			0x86 | 0xC4 => Ok(OldLicenseCode::TokumaShotenI),
			0xDA => Ok(OldLicenseCode::Tomy),
			0xC6 => Ok(OldLicenseCode::TonkinHouse),
			0xE1 => Ok(OldLicenseCode::Towachiki),
			0x5D => Ok(OldLicenseCode::Tradewest),
			0x7A => Ok(OldLicenseCode::TriffixEntertainment),
			0x93 => Ok(OldLicenseCode::Tsuburava),
			0x4F => Ok(OldLicenseCode::USGold),
			0x41 => Ok(OldLicenseCode::UbiSoft),
			0xC9 => Ok(OldLicenseCode::Ufl),
			0xCA => Ok(OldLicenseCode::Ultra),
			0xCC => Ok(OldLicenseCode::Use),
			0xE2 => Ok(OldLicenseCode::Uutaka),
			0xCB => Ok(OldLicenseCode::Vap),
			0x95 | 0xE3 => Ok(OldLicenseCode::Varie),
			0x8C => Ok(OldLicenseCode::VicTokai),
			0x92 => Ok(OldLicenseCode::VideoSystem),
			0x1F | 0x4A | 0x61 => Ok(OldLicenseCode::Virgin),
			0x1A => Ok(OldLicenseCode::Yanoman),
			0x96 => Ok(OldLicenseCode::YonezawaSPal),

			_ => Err(Error::InvalidOldLicenseCode(v)),
		}
	}
}

#[test]
fn test_convert_old_license_code() {
	assert_eq!(OldLicenseCode::try_from(0x00), Ok(OldLicenseCode::None));
	assert_eq!(OldLicenseCode::try_from(0x01), Ok(OldLicenseCode::Nintendo));
	assert_eq!(OldLicenseCode::try_from(0x08), Ok(OldLicenseCode::Capcom));
	assert_eq!(OldLicenseCode::try_from(0x09), Ok(OldLicenseCode::HotB));
	assert_eq!(OldLicenseCode::try_from(0x0A), Ok(OldLicenseCode::Jaleco));
	assert_eq!(OldLicenseCode::try_from(0x0B), Ok(OldLicenseCode::Coconuts));
	assert_eq!(
		OldLicenseCode::try_from(0x0C),
		Ok(OldLicenseCode::EliteSystems)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x13),
		Ok(OldLicenseCode::ElectronicArts)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x18),
		Ok(OldLicenseCode::Hudsonsoft)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x19),
		Ok(OldLicenseCode::ItcEntertainment)
	);
	assert_eq!(OldLicenseCode::try_from(0x1A), Ok(OldLicenseCode::Yanoman));
	assert_eq!(OldLicenseCode::try_from(0x1D), Ok(OldLicenseCode::Clary));
	assert_eq!(OldLicenseCode::try_from(0x1F), Ok(OldLicenseCode::Virgin));
	assert_eq!(
		OldLicenseCode::try_from(0x24),
		Ok(OldLicenseCode::PcmComplete)
	);
	assert_eq!(OldLicenseCode::try_from(0x25), Ok(OldLicenseCode::SanX));
	assert_eq!(
		OldLicenseCode::try_from(0x28),
		Ok(OldLicenseCode::KotobukiSystems)
	);
	assert_eq!(OldLicenseCode::try_from(0x29), Ok(OldLicenseCode::Seta));
	assert_eq!(
		OldLicenseCode::try_from(0x30),
		Ok(OldLicenseCode::Infogrames)
	);
	assert_eq!(OldLicenseCode::try_from(0x31), Ok(OldLicenseCode::Nintendo));
	assert_eq!(OldLicenseCode::try_from(0x32), Ok(OldLicenseCode::Bandai));
	assert_eq!(
		OldLicenseCode::try_from(0x33),
		Ok(OldLicenseCode::UseNewLicenseCode)
	);
	assert_eq!(OldLicenseCode::try_from(0x34), Ok(OldLicenseCode::Konami));
	assert_eq!(OldLicenseCode::try_from(0x35), Ok(OldLicenseCode::Hector));
	assert_eq!(OldLicenseCode::try_from(0x38), Ok(OldLicenseCode::Capcom));
	assert_eq!(
		OldLicenseCode::try_from(0x39),
		Ok(OldLicenseCode::Banpresto)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x3C),
		Ok(OldLicenseCode::EntertainmentI)
	);
	assert_eq!(OldLicenseCode::try_from(0x3E), Ok(OldLicenseCode::Gremlin));
	assert_eq!(OldLicenseCode::try_from(0x41), Ok(OldLicenseCode::UbiSoft));
	assert_eq!(OldLicenseCode::try_from(0x42), Ok(OldLicenseCode::Atlus));
	assert_eq!(OldLicenseCode::try_from(0x44), Ok(OldLicenseCode::Malibu));
	assert_eq!(OldLicenseCode::try_from(0x46), Ok(OldLicenseCode::Angel));
	assert_eq!(
		OldLicenseCode::try_from(0x47),
		Ok(OldLicenseCode::SpectrumHoloby)
	);
	assert_eq!(OldLicenseCode::try_from(0x49), Ok(OldLicenseCode::Irem));
	assert_eq!(OldLicenseCode::try_from(0x4A), Ok(OldLicenseCode::Virgin));
	assert_eq!(OldLicenseCode::try_from(0x4D), Ok(OldLicenseCode::Malibu));
	assert_eq!(OldLicenseCode::try_from(0x4F), Ok(OldLicenseCode::USGold));
	assert_eq!(OldLicenseCode::try_from(0x50), Ok(OldLicenseCode::Absolute));
	assert_eq!(OldLicenseCode::try_from(0x51), Ok(OldLicenseCode::Acclaim));
	assert_eq!(
		OldLicenseCode::try_from(0x52),
		Ok(OldLicenseCode::Activision)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x53),
		Ok(OldLicenseCode::AmericanSammy)
	);
	assert_eq!(OldLicenseCode::try_from(0x54), Ok(OldLicenseCode::Gametek));
	assert_eq!(
		OldLicenseCode::try_from(0x55),
		Ok(OldLicenseCode::ParkPlace)
	);
	assert_eq!(OldLicenseCode::try_from(0x56), Ok(OldLicenseCode::Ljn));
	assert_eq!(OldLicenseCode::try_from(0x57), Ok(OldLicenseCode::Matchbox));
	assert_eq!(
		OldLicenseCode::try_from(0x59),
		Ok(OldLicenseCode::MiltonBradley)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x5A),
		Ok(OldLicenseCode::Mindscape)
	);
	assert_eq!(OldLicenseCode::try_from(0x5B), Ok(OldLicenseCode::Romstar));
	assert_eq!(
		OldLicenseCode::try_from(0x5C),
		Ok(OldLicenseCode::NaxatSoft)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x5D),
		Ok(OldLicenseCode::Tradewest)
	);
	assert_eq!(OldLicenseCode::try_from(0x60), Ok(OldLicenseCode::Titus));
	assert_eq!(OldLicenseCode::try_from(0x61), Ok(OldLicenseCode::Virgin));
	assert_eq!(OldLicenseCode::try_from(0x67), Ok(OldLicenseCode::Ocean));
	assert_eq!(
		OldLicenseCode::try_from(0x69),
		Ok(OldLicenseCode::ElectronicArts)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x6E),
		Ok(OldLicenseCode::EliteSystems)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x6F),
		Ok(OldLicenseCode::ElectroBrain)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x70),
		Ok(OldLicenseCode::Infogrames)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x71),
		Ok(OldLicenseCode::Interplay)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x72),
		Ok(OldLicenseCode::Broderbund)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x73),
		Ok(OldLicenseCode::SculpteredSoft)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x75),
		Ok(OldLicenseCode::TheSalesCurve)
	);
	assert_eq!(OldLicenseCode::try_from(0x78), Ok(OldLicenseCode::THq));
	assert_eq!(OldLicenseCode::try_from(0x79), Ok(OldLicenseCode::Accolade));
	assert_eq!(
		OldLicenseCode::try_from(0x7A),
		Ok(OldLicenseCode::TriffixEntertainment)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x7C),
		Ok(OldLicenseCode::Microprose)
	);
	assert_eq!(OldLicenseCode::try_from(0x7F), Ok(OldLicenseCode::Kemco));
	assert_eq!(
		OldLicenseCode::try_from(0x80),
		Ok(OldLicenseCode::MisawaEntertainment)
	);
	assert_eq!(OldLicenseCode::try_from(0x83), Ok(OldLicenseCode::Lozc));
	assert_eq!(
		OldLicenseCode::try_from(0x86),
		Ok(OldLicenseCode::TokumaShotenI)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x8B),
		Ok(OldLicenseCode::BulletProofSoftware)
	);
	assert_eq!(OldLicenseCode::try_from(0x8C), Ok(OldLicenseCode::VicTokai));
	assert_eq!(OldLicenseCode::try_from(0x8E), Ok(OldLicenseCode::Ape));
	assert_eq!(OldLicenseCode::try_from(0x8F), Ok(OldLicenseCode::IMax));
	assert_eq!(OldLicenseCode::try_from(0x91), Ok(OldLicenseCode::ChunSoft));
	assert_eq!(
		OldLicenseCode::try_from(0x92),
		Ok(OldLicenseCode::VideoSystem)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x93),
		Ok(OldLicenseCode::Tsuburava)
	);
	assert_eq!(OldLicenseCode::try_from(0x95), Ok(OldLicenseCode::Varie));
	assert_eq!(
		OldLicenseCode::try_from(0x96),
		Ok(OldLicenseCode::YonezawaSPal)
	);
	assert_eq!(OldLicenseCode::try_from(0x97), Ok(OldLicenseCode::Kaneko));
	assert_eq!(OldLicenseCode::try_from(0x99), Ok(OldLicenseCode::Arc));
	assert_eq!(
		OldLicenseCode::try_from(0x9A),
		Ok(OldLicenseCode::NihonBussan)
	);
	assert_eq!(OldLicenseCode::try_from(0x9B), Ok(OldLicenseCode::Tecmo));
	assert_eq!(
		OldLicenseCode::try_from(0x9C),
		Ok(OldLicenseCode::Imagineer)
	);
	assert_eq!(
		OldLicenseCode::try_from(0x9D),
		Ok(OldLicenseCode::Banpresto)
	);
	assert_eq!(OldLicenseCode::try_from(0x9F), Ok(OldLicenseCode::Nova));
	assert_eq!(
		OldLicenseCode::try_from(0xA1),
		Ok(OldLicenseCode::HoriElectric)
	);
	assert_eq!(OldLicenseCode::try_from(0xA2), Ok(OldLicenseCode::Bandai));
	assert_eq!(OldLicenseCode::try_from(0xA4), Ok(OldLicenseCode::Konami));
	assert_eq!(OldLicenseCode::try_from(0xA6), Ok(OldLicenseCode::Kawada));
	assert_eq!(OldLicenseCode::try_from(0xA7), Ok(OldLicenseCode::Takara));
	assert_eq!(
		OldLicenseCode::try_from(0xA9),
		Ok(OldLicenseCode::TechnosJapan)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xAA),
		Ok(OldLicenseCode::Broderbund)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xAC),
		Ok(OldLicenseCode::ToeiAnimation)
	);
	assert_eq!(OldLicenseCode::try_from(0xAD), Ok(OldLicenseCode::Toho));
	assert_eq!(OldLicenseCode::try_from(0xAF), Ok(OldLicenseCode::Namco));
	assert_eq!(OldLicenseCode::try_from(0xB0), Ok(OldLicenseCode::Acclaim));
	assert_eq!(
		OldLicenseCode::try_from(0xB1),
		Ok(OldLicenseCode::AsciiOrNexoft)
	);
	assert_eq!(OldLicenseCode::try_from(0xB2), Ok(OldLicenseCode::Bandai));
	assert_eq!(OldLicenseCode::try_from(0xB4), Ok(OldLicenseCode::Enix));
	assert_eq!(OldLicenseCode::try_from(0xB6), Ok(OldLicenseCode::Hal));
	assert_eq!(OldLicenseCode::try_from(0xB7), Ok(OldLicenseCode::Snk));
	assert_eq!(
		OldLicenseCode::try_from(0xB9),
		Ok(OldLicenseCode::PonyCanyon)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xBA),
		Ok(OldLicenseCode::CultureBrainO)
	);
	assert_eq!(OldLicenseCode::try_from(0xBB), Ok(OldLicenseCode::Sunsoft));
	assert_eq!(
		OldLicenseCode::try_from(0xBD),
		Ok(OldLicenseCode::SonyImagesoft)
	);
	assert_eq!(OldLicenseCode::try_from(0xBF), Ok(OldLicenseCode::Sammy));
	assert_eq!(OldLicenseCode::try_from(0xC0), Ok(OldLicenseCode::Taito));
	assert_eq!(OldLicenseCode::try_from(0xC2), Ok(OldLicenseCode::Kemco));
	assert_eq!(
		OldLicenseCode::try_from(0xC3),
		Ok(OldLicenseCode::Squaresoft)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xC4),
		Ok(OldLicenseCode::TokumaShotenI)
	);
	assert_eq!(OldLicenseCode::try_from(0xC5), Ok(OldLicenseCode::DataEast));
	assert_eq!(
		OldLicenseCode::try_from(0xC6),
		Ok(OldLicenseCode::TonkinHouse)
	);
	assert_eq!(OldLicenseCode::try_from(0xC8), Ok(OldLicenseCode::Koei));
	assert_eq!(OldLicenseCode::try_from(0xC9), Ok(OldLicenseCode::Ufl));
	assert_eq!(OldLicenseCode::try_from(0xCA), Ok(OldLicenseCode::Ultra));
	assert_eq!(OldLicenseCode::try_from(0xCB), Ok(OldLicenseCode::Vap));
	assert_eq!(OldLicenseCode::try_from(0xCC), Ok(OldLicenseCode::Use));
	assert_eq!(OldLicenseCode::try_from(0xCD), Ok(OldLicenseCode::Meldac));
	assert_eq!(
		OldLicenseCode::try_from(0xCE),
		Ok(OldLicenseCode::PonyCanyonOr)
	);
	assert_eq!(OldLicenseCode::try_from(0xCF), Ok(OldLicenseCode::Angel));
	assert_eq!(OldLicenseCode::try_from(0xD0), Ok(OldLicenseCode::Taito));
	assert_eq!(OldLicenseCode::try_from(0xD1), Ok(OldLicenseCode::Sofel));
	assert_eq!(OldLicenseCode::try_from(0xD2), Ok(OldLicenseCode::Quest));
	assert_eq!(
		OldLicenseCode::try_from(0xD3),
		Ok(OldLicenseCode::SigmaEnterprises)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xD4),
		Ok(OldLicenseCode::AskKodansha)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xD6),
		Ok(OldLicenseCode::NaxatSoft)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xD7),
		Ok(OldLicenseCode::CopyaSystems)
	);
	assert_eq!(
		OldLicenseCode::try_from(0xD9),
		Ok(OldLicenseCode::Banpresto)
	);
	assert_eq!(OldLicenseCode::try_from(0xDA), Ok(OldLicenseCode::Tomy));
	assert_eq!(OldLicenseCode::try_from(0xDB), Ok(OldLicenseCode::Ljn));
	assert_eq!(OldLicenseCode::try_from(0xDD), Ok(OldLicenseCode::Ncs));
	assert_eq!(OldLicenseCode::try_from(0xDE), Ok(OldLicenseCode::Human));
	assert_eq!(OldLicenseCode::try_from(0xDF), Ok(OldLicenseCode::Altron));
	assert_eq!(OldLicenseCode::try_from(0xE0), Ok(OldLicenseCode::Jaleco));
	assert_eq!(
		OldLicenseCode::try_from(0xE1),
		Ok(OldLicenseCode::Towachiki)
	);
	assert_eq!(OldLicenseCode::try_from(0xE2), Ok(OldLicenseCode::Uutaka));
	assert_eq!(OldLicenseCode::try_from(0xE3), Ok(OldLicenseCode::Varie));
	assert_eq!(OldLicenseCode::try_from(0xE5), Ok(OldLicenseCode::Epoch));
	assert_eq!(OldLicenseCode::try_from(0xE7), Ok(OldLicenseCode::Athena));
	assert_eq!(OldLicenseCode::try_from(0xE8), Ok(OldLicenseCode::Asmik));
	assert_eq!(OldLicenseCode::try_from(0xE9), Ok(OldLicenseCode::Natsume));
	assert_eq!(
		OldLicenseCode::try_from(0xEA),
		Ok(OldLicenseCode::KingRecords)
	);
	assert_eq!(OldLicenseCode::try_from(0xEB), Ok(OldLicenseCode::Atlus));
	assert_eq!(
		OldLicenseCode::try_from(0xEC),
		Ok(OldLicenseCode::EpicSonyRecords)
	);
	assert_eq!(OldLicenseCode::try_from(0xEE), Ok(OldLicenseCode::Igs));
	assert_eq!(OldLicenseCode::try_from(0xF0), Ok(OldLicenseCode::AWave));
	assert_eq!(
		OldLicenseCode::try_from(0xF3),
		Ok(OldLicenseCode::ExtremeEntertainment)
	);
	assert_eq!(OldLicenseCode::try_from(0xFF), Ok(OldLicenseCode::Ljn));
}
