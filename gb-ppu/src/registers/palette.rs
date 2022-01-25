use super::Register;
use crate::error::{PPUError, PPUResult};
use crate::Color;

const INDEX_0: u8 = 0b11;
const INDEX_1: u8 = 0b1100;
const INDEX_2: u8 = 0b11_0000;
const INDEX_3: u8 = 0b1100_0000;

#[derive(Clone, Copy, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Palette {
    map: u8,
    is_sprite: bool,
}

impl Palette {
    pub fn new(byte: u8, is_sprite: bool) -> Self {
        Palette {
            map: byte,
            is_sprite,
        }
    }

    pub fn new_background() -> Self {
        Palette {
            map: 0,
            is_sprite: false,
        }
    }

    pub fn new_sprite() -> Self {
        Palette {
            map: 0,
            is_sprite: true,
        }
    }

    pub fn is_sprite(&self) -> bool {
        self.is_sprite
    }

    pub fn get_map(&self) -> u8 {
        self.map
    }

    /// Get the color value associated to the given index.
    pub fn get_value(&self, index: u8) -> PPUResult<u8> {
        match index {
            3 => Ok((self.map & INDEX_3) >> 6),
            2 => Ok((self.map & INDEX_2) >> 4),
            1 => Ok((self.map & INDEX_1) >> 2),
            0 => Ok(self.map & INDEX_0),
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

impl From<u8> for Palette {
    fn from(byte: u8) -> Palette {
        Palette {
            map: byte,
            is_sprite: false,
        }
    }
}

impl From<Palette> for u8 {
    fn from(register: Palette) -> u8 {
        register.map
    }
}

impl Register for Palette {
    fn write(&mut self, v: u8) {
        self.map = v;
    }
}
