use crate::error::{Error, PPUResult};

pub const TILEDATA_ADRESS_MAX: usize = 0x17FF;
pub const TILEMAP_POSITION_MAX: usize = 0x3FF;
pub const TILEMAP_START_0: usize = 0x1800;
pub const TILEMAP_START_1: usize = 0x1C00;
pub const TILEDATA_START_1: usize = 0x1000 / 16;

/// Contains operations to read more easily the differents values of the vram.
pub struct Vram {
    data: [u8; Vram::SIZE as usize],
}

impl Vram {
    pub const SIZE: usize = 0x2000;

    pub fn new() -> Self {
        Vram {
            data: [0x00; Self::SIZE as usize],
        }
    }

    pub fn read(&self, addr: usize) -> Option<u8> {
        if addr < Self::SIZE {
            Some(self.data[addr])
        } else {
            None
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) -> Option<()> {
        if addr < Self::SIZE {
            self.data[addr] = value;
            Some(())
        } else {
            None
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
    ) -> PPUResult<usize> {
        if pos > TILEMAP_POSITION_MAX {
            return Err(Error::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEMAP_POSITION_MAX,
            });
        }
        let index = if map_area_bit {
            self.data[TILEMAP_START_1 + pos]
        } else {
            self.data[TILEMAP_START_0 + pos]
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
    pub fn read_8_pixels(&self, pos: usize) -> PPUResult<[u8; 8]> {
        let mut pixels = [0; 8];
        if pos > TILEDATA_ADRESS_MAX - 1 {
            return Err(Error::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEDATA_ADRESS_MAX - 1,
            });
        }
        let byte_a = self.data[pos];
        let byte_b = self.data[pos + 1];
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
    pub fn read_tile_line(&self, tile_pos: usize, line: usize) -> PPUResult<[u8; 8]> {
        if line > 7 {
            return Err(Error::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 7,
            });
        }
        self.read_8_pixels((tile_pos * 8 + line) * 2)
    }

    /// Return all the pixel values of a tile.
    ///
    /// This function is used for debugging purpose, the ppu does not select pixels tile by tile.
    ///
    /// ### Parameters
    ///  - **pos**: position of the first byte of the tile.
    pub fn read_8x8_tile(&self, pos: usize) -> PPUResult<[[u8; 8]; 8]> {
        let mut tile = [[0; 8]; 8];
        if pos * 8 * 2 > TILEDATA_ADRESS_MAX + 1 - 8 * 2 {
            return Err(Error::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: TILEDATA_ADRESS_MAX / (8 * 2),
            });
        }
        for (i, row) in tile.iter_mut().enumerate() {
            *row = self.read_tile_line(pos, i)?;
        }
        Ok(tile)
    }

    pub fn overwrite(&mut self, data: &[u8; Self::SIZE as usize]) {
        self.data = *data;
    }
}

impl Default for Vram {
    fn default() -> Vram {
        Vram::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_8_pixels() {
        let mut vram = Vram::new();
        vram.data[42] = 0x33;
        vram.data[43] = 0x66;
        let pixels = vram.read_8_pixels(42).unwrap();
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
        let vram = Vram::new();
        vram.read_8_pixels(0x17FF).unwrap_err();
    }
}
