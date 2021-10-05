use super::Register;
use crate::color::Color;
use crate::error::{PPUError, PPUResult};
use modular_bitfield::{bitfield, specifiers::B2};

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
struct MapField {
    index_0: B2,
    index_1: B2,
    index_2: B2,
    index_3: B2,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Palette {
    map: MapField,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            map: MapField::new(),
        }
    }

    /// Get the color value associated to the given index.
    pub fn get_value(&self, index: u8) -> PPUResult<u8> {
        match index {
            3 => Ok(self.map.index_3()),
            2 => Ok(self.map.index_2()),
            1 => Ok(self.map.index_1()),
            0 => Ok(self.map.index_0()),
            _ => Err(PPUError::OutOfBound {
                value: index as usize,
                min_bound: 0,
                max_bound: 3,
            }),
        }
    }

    /// Get the color for the value associated to the given index.
    pub fn get_color(&self, index: u8) -> PPUResult<Color> {
        Color::from_value(self.get_value(index)?)
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

impl Register for Palette {}
