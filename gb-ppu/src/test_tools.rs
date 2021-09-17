use gb_bus::{Address, Area};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TestAddress {
    pub relative: u16,
    pub absolute: u16,
    pub area: Area,
}

impl TestAddress {
    pub fn root_vram() -> Self {
        TestAddress {
            relative: 0x0000,
            absolute: 0x8000,
            area: Area::Vram,
        }
    }

    pub fn root_oam() -> Self {
        TestAddress {
            relative: 0x0000,
            absolute: 0xFE00,
            area: Area::Oam,
        }
    }
}

impl Address<Area> for TestAddress {
    fn get_address(&self) -> usize {
        self.relative as usize
    }

    fn area_type(&self) -> Area {
        self.area
    }
}
