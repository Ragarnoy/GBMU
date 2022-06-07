use gb_bus::{Address, Area, Error, FileOperation, Source};

pub const DMG_BIOS_SIZE: usize = 256;
pub const CGB_BIOS_SIZE: usize = 2304;

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

impl<A> FileOperation<A, Area> for Bios
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        self.container
            .get(addr.get_address())
            .map_or_else(|| Err(Error::new_segfault(addr.into())), |v| Ok(*v))
    }
}
