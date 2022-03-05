use crate::error::{PPUError, PPUResult};
use crate::Color;

#[cfg(feature = "serialization")]
serde_big_array::big_array! { PaletteDataSize; PalettesCGB::PALETTES_RAW_SIZE }

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct PalettesCGB {
    bcps: u8,
    bcpd: u8,
    #[cfg_attr(feature = "serialization", serde(with = "PaletteDataSize"))]
    bc_values: [u8; Self::PALETTES_RAW_SIZE],
    ocps: u8,
    ocpd: u8,
    #[cfg_attr(feature = "serialization", serde(with = "PaletteDataSize"))]
    oc_values: [u8; Self::PALETTES_RAW_SIZE],
}

impl Default for PalettesCGB {
    fn default() -> PalettesCGB {
        PalettesCGB {
            bcps: 0,
            bcpd: 0,
            bc_values: [0; Self::PALETTES_RAW_SIZE],
            ocps: 0,
            ocpd: 0,
            oc_values: [0; Self::PALETTES_RAW_SIZE],
        }
    }
}

impl PalettesCGB {
    pub const SIZE: usize = 4;
    const PALETTE_COLOR_NB: usize = 4;
    const PALETTE_COLOR_SIZE: usize = 2;
    const PALETTE_NB: usize = 8;
    const PALETTES_RAW_SIZE: usize =
        Self::PALETTE_NB * Self::PALETTE_COLOR_NB * Self::PALETTE_COLOR_SIZE;

    pub fn get_color(
        &self,
        color_index: usize,
        palette_nb: usize,
        is_background: bool,
    ) -> PPUResult<Color> {
        if palette_nb < Self::PALETTE_NB && color_index < Self::PALETTE_COLOR_NB {
            let index = palette_nb * Self::PALETTE_COLOR_NB * Self::PALETTE_COLOR_SIZE
                + color_index * Self::PALETTE_COLOR_SIZE;
            let values = if is_background {
                self.bc_values
            } else {
                self.oc_values
            };
            Ok(Color::Rgb555(values[index], values[index + 1]))
        } else {
            Err(PPUError::OutOfBound {
                value: palette_nb,
                min_bound: 0,
                max_bound: Self::PALETTE_NB - 1,
            })
        }
    }
}

impl From<[u8; 4]> for PalettesCGB {
    fn from(bytes: [u8; 4]) -> PalettesCGB {
        PalettesCGB {
            bcps: bytes[0],
            bcpd: bytes[1],
            bc_values: [0; Self::PALETTES_RAW_SIZE],
            ocps: bytes[2],
            ocpd: bytes[3],
            oc_values: [0; Self::PALETTES_RAW_SIZE],
        }
    }
}

impl From<PalettesCGB> for [u8; 4] {
    fn from(register: PalettesCGB) -> [u8; 4] {
        [register.bcps, register.bcpd, register.ocps, register.ocpd]
    }
}
