pub const VRAM_SIZE: usize = 0x2000;
const TILEDATA_ADRESS_MAX: usize = 0x17FF;
const TILEDATA_ADRESS_MIN: usize = 0x0000;

use crate::error::{Error, PPUResult};

pub struct Vram {
    data: [u8; VRAM_SIZE as usize],
}

impl Vram {
    pub fn new() -> Self {
        Vram {
            data: [0x00; VRAM_SIZE as usize],
        }
    }

    pub fn read_8_pixels(&self, adr: usize) -> PPUResult<[u8; 8], usize> {
        let mut pixels = [0; 8];
        if adr > TILEDATA_ADRESS_MAX - 1 {
            return Err(Error::OutOfBound {
                value: adr,
                min_bound: TILEDATA_ADRESS_MIN,
                max_bound: TILEDATA_ADRESS_MAX - 1,
            });
        }
        let byte_a = self.data[adr];
        let byte_b = self.data[adr + 1];
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

    pub fn read_8x8_tile(&self, adr: usize) -> PPUResult<[[u8; 8]; 8], usize> {
        let mut tile = [[0; 8]; 8];
        if adr * 8 * 2 > TILEDATA_ADRESS_MAX + 1 - 8 * 2 {
            return Err(Error::OutOfBound {
                value: adr,
                min_bound: TILEDATA_ADRESS_MIN,
                max_bound: TILEDATA_ADRESS_MAX / (8 * 2),
            });
        }
        for (i, row) in tile.iter_mut().enumerate() {
            *row = self.read_8_pixels((adr * 8 + i) * 2)?;
        }
        Ok(tile)
    }

    pub fn overwrite(&mut self, data: &[u8; VRAM_SIZE as usize]) {
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
