mod cartridge_type;
mod destination_code;
mod license_code;
mod size;

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
