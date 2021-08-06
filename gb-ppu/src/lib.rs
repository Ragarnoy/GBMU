use gb_lcd::render::{TextureData, SCREEN_HEIGHT, SCREEN_WIDTH, TEXTURE_SIZE};

pub struct PPU {
    pixels: TextureData,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            pixels: [[255; 3]; TEXTURE_SIZE],
        }
    }

    pub fn pixels(&self) -> &TextureData {
        &self.pixels
    }

    pub fn compute(&mut self) {
        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                self.pixels[(i + j * SCREEN_WIDTH) as usize] =
                    if j == 0 || j == SCREEN_HEIGHT - 1 || i == 0 || i == SCREEN_WIDTH - 1 {
                        [150, 50, 50]
                    } else if (i + j) % 2 == 0 {
                        [100; 3]
                    } else {
                        [200; 3]
                    };
            }
        }
    }
}

impl Default for PPU {
    fn default() -> PPU {
        PPU::new()
    }
}
