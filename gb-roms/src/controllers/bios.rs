use gb_bus::{Address, Area, Error, FileOperation};

pub enum BiosType {
    Dmg,
    Cgb,
}

pub struct Bios {
    container: Vec<u8>,
    pub bios_type: BiosType,
}

impl Bios {
    pub fn from_bytes(bios: BiosType, bytes: &[u8]) -> Self {
        Self {
            bios_type: bios,
            container: Vec::from(bytes),
        }
    }
}

pub fn dmg_bios() -> Bios {
    Bios::from_bytes(
        BiosType::Dmg,
        include_bytes!("../../../roms/bios/dmg_boot.bin"),
    )
}

pub fn cgb_bios() -> Bios {
    Bios::from_bytes(
        BiosType::Cgb,
        include_bytes!("../../../roms/bios/cgb_boot.bin"),
    )
}

impl FileOperation<Area> for Bios {
    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        self.container
            .get(addr.get_address())
            .map_or_else(|| Err(Error::new_segfault(addr)), |v| Ok(*v))
    }
}
