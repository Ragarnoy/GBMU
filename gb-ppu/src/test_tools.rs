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
    pub fn new(area: IORegArea, abs: u16, rel: u16) -> Self {
        Self {
            relative: rel,
            absolute: abs,
            area,
        }
    }

    pub fn reg(area: IORegArea, abs: u16) -> Self {
        Self::new(area, abs, 0)
    }

    pub fn from_area(area: IORegArea) -> Self {
        Self::reg(area, u16::from(area))
    }

    pub fn control() -> Self {
        Self::from_area(IORegArea::LcdControl)
    }

    pub fn stat() -> Self {
        Self::from_area(IORegArea::LcdStat)
    }

    pub fn scrolling(pos: u16) -> Self {
        match pos {
            0 => Self::from_area(IORegArea::Scy),
            1 => Self::from_area(IORegArea::Scx),
            2 => Self::from_area(IORegArea::Ly),
            3 => Self::from_area(IORegArea::Lyc),
            _ => panic!("unexpected scroll position {}", pos),
        }
    }

    pub fn palette(pos: u16) -> Self {
        match pos {
            0 => Self::from_area(IORegArea::Bgp),
            1 => Self::from_area(IORegArea::Obp0),
            2 => Self::from_area(IORegArea::Obp1),
            _ => panic!("unexpected palette position {}", pos),
        }
    }

    pub fn window_pos(pos: u16) -> Self {
        match pos {
            0 => Self::from_area(IORegArea::Wy),
            1 => Self::from_area(IORegArea::Wx),
            _ => panic!("unexpected windows position {}", pos),
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
