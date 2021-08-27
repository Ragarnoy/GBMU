use crate::color::{Color, BLACK, DARK_GRAY, LIGHT_GRAY, WHITE};
use modular_bitfield::{bitfield, specifiers::B2};

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
struct MapField {
    index_0: B2,
    index_1: B2,
    index_2: B2,
    index_3: B2,
}

#[derive(Default)]
pub struct Palette {
    map: MapField,
}

const COLOR_MAP: [Color; 4] = [WHITE, LIGHT_GRAY, DARK_GRAY, BLACK];

impl Palette {
    pub fn new() -> Self {
        Palette {
            map: MapField::new(),
        }
    }

    pub fn map_color(&self, index: u8) -> Color {
        match index {
            3 => COLOR_MAP[self.map.index_3() as usize],
            2 => COLOR_MAP[self.map.index_2() as usize],
            1 => COLOR_MAP[self.map.index_1() as usize],
            _ => COLOR_MAP[self.map.index_0() as usize],
        }
    }
}

impl From<u8> for MapField {
    fn from(byte: u8) -> MapField {
        MapField::from_bytes([byte])
    }
}

impl From<u8> for Palette {
    fn from(byte: u8) -> Palette {
        Palette { map: byte.into() }
    }
}

impl From<MapField> for u8 {
    fn from(map: MapField) -> u8 {
        map.into_bytes()[0]
    }
}

impl From<Palette> for u8 {
    fn from(register: Palette) -> u8 {
        register.map.into()
    }
}
