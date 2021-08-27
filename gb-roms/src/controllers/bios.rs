use gb_bus::FileOperation;
pub const BIOS_SIZE: usize = 0x100;

pub struct Bios {
    container: [u8; BIOS_SIZE],
}

impl Bios {
    from_bytes(bytes: [u8; BIOS_SIZE]) -> Self {
    }
}

impl FileOperation for Bios {
}
