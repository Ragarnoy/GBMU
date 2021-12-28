use gb_bus::{Addr, Address, Area, Error, FileOperation};

pub enum BiosType {
    Dmg,
    Cgb,
}

pub struct Bios {
    pub(crate) container: Vec<u8>,
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

pub fn dmg() -> Bios {
    Bios::from_bytes(
        BiosType::Dmg,
        include_bytes!("../../../assets/bios/dmg_boot.bin"),
    )
}

pub fn cgb() -> Bios {
    Bios::from_bytes(
        BiosType::Cgb,
        include_bytes!("../../../assets/bios/cgb_boot.bin"),
    )
}

impl FileOperation<Addr<Area>, Area> for Bios {
    fn read(&self, addr: Addr<Area>) -> Result<u8, Error> {
        self.container
            .get(addr.get_address())
            .map_or_else(|| Err(Error::new_segfault(addr.into())), |v| Ok(*v))
    }
}
