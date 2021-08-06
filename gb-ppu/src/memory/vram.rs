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
            data: [0; VRAM_SIZE as usize],
        }
    }

    pub fn read_8_pixels(&self, adr: usize) -> PPUResult<[u8; 8], usize> {
        let pixels = [0; 8];
        if adr > TILEDATA_ADRESS_MAX {
            return Err(Error::OutOfBound {
                value: adr,
                min_bound: TILEDATA_ADRESS_MIN,
                max_bound: TILEDATA_ADRESS_MAX,
            });
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
        let vram = Vram::new();
        let pixels = vram.read_8_pixels(0).unwrap();
        assert_eq!(pixels[0], 0);
        assert_eq!(pixels[1], 0);
        assert_eq!(pixels[2], 0);
        assert_eq!(pixels[3], 0);
        assert_eq!(pixels[4], 0);
        assert_eq!(pixels[5], 0);
        assert_eq!(pixels[6], 0);
        assert_eq!(pixels[7], 0);
    }

    #[test]
    fn read_pixel_out_of_bound() {
        let vram = Vram::new();
        vram.read_8_pixels(0x1900).unwrap_err();
    }
}
