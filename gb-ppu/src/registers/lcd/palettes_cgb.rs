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

    const SPEC_AUTO_INCR: u8 = 0b1000_0000;
    const SPEC_DATA_INDEX: u8 = !Self::SPEC_AUTO_INCR;

    pub fn get_bcps(&self) -> u8 {
        self.bcps
    }

    pub fn set_bcps(&mut self, value: u8) {
        self.bcps = value;
    }

    pub fn get_bcpd(&self) -> u8 {
        self.bcpd
    }

    pub fn set_bcpd(&mut self, value: u8) {
        self.bcpd = value;
        let index = (self.bcps & Self::SPEC_DATA_INDEX) as usize;
        self.bc_values[index] = value;

        if self.bcps & Self::SPEC_AUTO_INCR != 0 {
            if self.bcps == Self::SPEC_AUTO_INCR | Self::SPEC_DATA_INDEX {
                self.bcps = Self::SPEC_AUTO_INCR;
            } else {
                self.bcps += 1;
            }
        }
    }

    pub fn get_ocps(&self) -> u8 {
        self.ocps
    }

    pub fn set_ocps(&mut self, value: u8) {
        self.ocps = value;
    }

    pub fn get_ocpd(&self) -> u8 {
        self.ocpd
    }

    pub fn set_ocpd(&mut self, value: u8) {
        self.ocpd = value;
        let index = (self.ocps & Self::SPEC_DATA_INDEX) as usize;
        self.oc_values[index] = value;

        if self.ocps & Self::SPEC_AUTO_INCR != 0 {
            if self.ocps == Self::SPEC_AUTO_INCR | Self::SPEC_DATA_INDEX {
                self.ocps = Self::SPEC_AUTO_INCR;
            } else {
                self.ocps += 1;
            }
        }
    }

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
