const VRAM_SIZE: usize = 0x2000;
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
            *pixel = if i < 7 {
                (byte_a & 0b1000_0000 >> i) >> (7 - i) | (byte_b & 0b1000_0000 >> i) >> (6 - i)
            } else {
                (byte_a & 0b0000_0001) | (byte_b & 0b0000_0001) << 1
            };
        }
        Ok(pixels)
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
        assert_eq!(pixels[0], 0, "pixel 0 wrong");
        assert_eq!(pixels[1], 2, "pixel 1 wrong");
        assert_eq!(pixels[2], 3, "pixel 2 wrong");
        assert_eq!(pixels[3], 1, "pixel 3 wrong");
        assert_eq!(pixels[4], 0, "pixel 4 wrong");
        assert_eq!(pixels[5], 2, "pixel 5 wrong");
        assert_eq!(pixels[6], 3, "pixel 6 wrong");
        assert_eq!(pixels[7], 1, "pixel 7 wrong");
    }

    #[test]
    fn read_pixel_out_of_bound() {
        let vram = Vram::new();
        vram.read_8_pixels(0x17FF).unwrap_err();
    }
}
