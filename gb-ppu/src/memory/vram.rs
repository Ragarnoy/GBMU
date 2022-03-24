#[cfg(feature = "serialization")]
mod de_ser;

use super::{Lock, Lockable};
use crate::error::{PPUError, PPUResult};
use crate::registers::LcdReg;

pub const TILEDATA_ADRESS_MAX: usize = 0x17FF;
pub const TILEMAP_POSITION_MAX: usize = 0x3FF;
pub const TILEMAP_START_0: usize = 0x1800;
pub const TILEMAP_START_1: usize = 0x1C00;
pub const TILEDATA_START_1: usize = 0x1000 / 16;

const VBK_BANK_0: u8 = LcdReg::VBK_UNUSED_BITS;
const VBK_BANK_1: u8 = LcdReg::VBK_UNUSED_BITS | 1;

#[derive(Clone, Copy)]
pub enum BankSelector {
    Bank0,
    Bank1,
}

impl Default for BankSelector {
    fn default() -> BankSelector {
        BankSelector::Bank0
    }
}

impl From<BankSelector> for usize {
    fn from(bank: BankSelector) -> usize {
        match bank {
            BankSelector::Bank0 => 0,
            BankSelector::Bank1 => 1,
        }
    }
}

impl TryFrom<u8> for BankSelector {
    type Error = PPUError;
    fn try_from(byte: u8) -> PPUResult<BankSelector> {
        match byte {
            VBK_BANK_0 => Ok(BankSelector::Bank0),
            VBK_BANK_1 => Ok(BankSelector::Bank1),
            b => Err(PPUError::OutOfBound {
                max_bound: VBK_BANK_1 as usize,
                min_bound: VBK_BANK_0 as usize,
                value: b as usize,
            }),
        }
    }
}

/// Contains operations to read more easily the differents values of the vram.
#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct Vram {
    #[cfg_attr(feature = "serialization", serde(with = "de_ser::data"))]
    data: Vec<[u8; Vram::SIZE as usize]>,
    lock: Option<Lock>,
}

impl Vram {
    pub const SIZE: usize = 0x2000;

    pub fn new(cgb_enabled: bool) -> Self {
        Vram {
            data: vec![[0x00; Self::SIZE]; if cgb_enabled { 2 } else { 1 }],
            lock: None,
        }
    }

    pub fn read(&self, addr: usize, bank: Option<BankSelector>) -> PPUResult<u8> {
        let bank_index: usize = bank.unwrap_or_default().into();
        if addr < Self::SIZE {
            Ok(self.data[bank_index][addr])
        } else {
            Err(PPUError::OutOfBound {
                value: addr,
                min_bound: 0,
                max_bound: Self::SIZE,
            })
        }
    }

    pub fn write(&mut self, addr: usize, value: u8, bank: Option<BankSelector>) -> PPUResult<()> {
        let bank_index: usize = bank.unwrap_or_default().into();
        if addr < Self::SIZE {
            self.data[bank_index][addr] = value;
            Ok(())
        } else {
            Err(PPUError::OutOfBound {
                value: addr,
                min_bound: 0,
                max_bound: Self::SIZE,
            })
        }
    }

    /// Return the index of a tile from the correct map area depending on the area_bits.
    ///
    /// ### Parameters
    ///  - **pos**: the position of the index to retrieve in the tilemap.
    ///  - **map_area_bit**: the control bit (bg_tilemap_area or win_tilemap_area) indicating in which block of the vram is stored the tilemap.
    ///  - **data_area_bit**: the control bit (bg_win_tiledata_area) indicating in which block of the vram is stored the tilesheet for the background/window.
    pub fn get_map_tile_index(
        &self,
        pos: usize,
        map_area_bit: bool,
        data_area_bit: bool,
        bank: Option<BankSelector>,
    ) -> PPUResult<usize> {
        if pos > TILEMAP_POSITION_MAX {
            return Err(PPUError::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEMAP_POSITION_MAX,
            });
        }
        let bank_index: usize = bank.unwrap_or_default().into();
        let index = if map_area_bit {
            self.data[bank_index][TILEMAP_START_1 + pos]
        } else {
            self.data[bank_index][TILEMAP_START_0 + pos]
        };
        if data_area_bit {
            Ok(index as usize)
        } else {
            let index = index as i8;
            Ok((TILEDATA_START_1 as i32 + index as i32) as usize)
        }
    }

    /// Read a row of 8 pixels values contained in a couple of byte in the vram.
    ///
    /// ### Parameters
    ///  - **pos**: position of the couple of bytes to be interpreted as pixels values.
    pub fn read_8_pixels(&self, pos: usize, bank: Option<BankSelector>) -> PPUResult<[u8; 8]> {
        let mut pixels = [0; 8];
        let bank_index: usize = bank.unwrap_or_default().into();
        if pos > TILEDATA_ADRESS_MAX - 1 {
            return Err(PPUError::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEDATA_ADRESS_MAX - 1,
            });
        }
        let byte_a = self.data[bank_index][pos];
        let byte_b = self.data[bank_index][pos + 1];
        for (i, pixel) in pixels.iter_mut().enumerate() {
            let bit = 0b0000_0001 << i;
            *pixel = if i > 0 {
                (byte_a & bit) >> i | (byte_b & bit) >> (i - 1)
            } else {
                (byte_a & bit) | (byte_b & bit) << 1
            };
        }
        Ok(pixels)
    }

    /// Read a line of 8 pixels values from a tile in the vram.
    ///
    /// ### Parameters
    ///  - **tile_pos**: The position of the tile to get the line from.
    ///  - **line**: The number of the line to return.
    pub fn read_tile_line(
        &self,
        tile_pos: usize,
        line: usize,
        bank: Option<BankSelector>,
    ) -> PPUResult<[u8; 8]> {
        if line > 7 {
            return Err(PPUError::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 7,
            });
        }
        self.read_8_pixels((tile_pos * 8 + line) * 2, bank)
    }

    /// Return all the pixel values of a tile.
    ///
    /// This function is used for debugging purpose, the ppu does not select pixels tile by tile.
    ///
    /// ### Parameters
    ///  - **pos**: position of the first byte of the tile.
    pub fn read_8x8_tile(&self, pos: usize, bank: Option<BankSelector>) -> PPUResult<[[u8; 8]; 8]> {
        let mut tile = [[0; 8]; 8];
        if pos * 8 * 2 > TILEDATA_ADRESS_MAX + 1 - 8 * 2 {
            return Err(PPUError::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEDATA_ADRESS_MAX / (8 * 2),
            });
        }
        for (i, row) in tile.iter_mut().enumerate() {
            *row = self.read_tile_line(pos, i, bank)?;
        }
        Ok(tile)
    }

    pub fn overwrite(&mut self, data: &[u8; Self::SIZE as usize], bank: Option<BankSelector>) {
        let bank_index: usize = bank.unwrap_or_default().into();
        self.data[bank_index] = *data;
    }
}

impl From<[u8; Vram::SIZE]> for Vram {
    fn from(data: [u8; Vram::SIZE]) -> Vram {
        Vram {
            data: vec![data],
            lock: None,
        }
    }
}

impl Default for Vram {
    fn default() -> Vram {
        Vram::new(false)
    }
}

impl Lockable for Vram {
    fn lock(&mut self, owner: Lock) {
        self.lock = Some(owner);
    }

    fn unlock(&mut self) {
        self.lock = None;
    }

    fn get_lock(&self) -> Option<Lock> {
        self.lock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_8_pixels() {
        let mut vram = Vram::default();
        vram.data[0][42] = 0x33;
        vram.data[0][43] = 0x66;
        let pixels = vram.read_8_pixels(42, None).unwrap();
        assert_eq!(pixels[0], 1, "pixel 0 wrong");
        assert_eq!(pixels[1], 3, "pixel 1 wrong");
        assert_eq!(pixels[2], 2, "pixel 2 wrong");
        assert_eq!(pixels[3], 0, "pixel 3 wrong");
        assert_eq!(pixels[4], 1, "pixel 4 wrong");
        assert_eq!(pixels[5], 3, "pixel 5 wrong");
        assert_eq!(pixels[6], 2, "pixel 6 wrong");
        assert_eq!(pixels[7], 0, "pixel 7 wrong");
    }

    #[test]
    fn read_pixel_out_of_bound() {
        let vram = Vram::default();
        vram.read_8_pixels(0x17FF, None).unwrap_err();
    }
}
