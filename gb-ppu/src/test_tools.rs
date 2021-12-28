use gb_bus::{Address, Area, IORegArea};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TestAddress {
    pub relative: u16,
    pub absolute: u16,
    pub area: Area,
}

impl From<TestAddress> for u16 {
    fn from(a: TestAddress) -> Self {
        a.absolute
    }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TestIORegAddress {
    pub relative: u16,
    pub absolute: u16,
    pub area: IORegArea,
}

impl TestIORegAddress {
    pub fn control() -> Self {
        TestIORegAddress {
            relative: 0x0000,
            absolute: 0xFF40,
            area: IORegArea::Lcd,
        }
    }

    pub fn stat() -> Self {
        TestIORegAddress {
            relative: 0x0001,
            absolute: 0xFF41,
            area: IORegArea::Lcd,
        }
    }

    pub fn scrolling(pos: u16) -> Self {
        TestIORegAddress {
            relative: 0x0002 + pos.min(3),
            absolute: 0xFF42 + pos.min(3),
            area: IORegArea::Lcd,
        }
    }

    pub fn palette(pos: u16) -> Self {
        TestIORegAddress {
            relative: 0x0007 + pos.min(2),
            absolute: 0xFF47 + pos.min(2),
            area: IORegArea::Lcd,
        }
    }

    pub fn window_pos(pos: u16) -> Self {
        TestIORegAddress {
            relative: 0x000A + pos.min(1),
            absolute: 0xFF4A + pos.min(1),
            area: IORegArea::Lcd,
        }
    }
}

impl Address<IORegArea> for TestIORegAddress {
    fn get_address(&self) -> usize {
        self.relative as usize
    }

    fn area_type(&self) -> IORegArea {
        self.area
    }
}

impl From<TestIORegAddress> for u16 {
    fn from(a: TestIORegAddress) -> Self {
        a.absolute
    }
}
