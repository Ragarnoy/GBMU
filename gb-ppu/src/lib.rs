mod error;
mod memory;

use gb_lcd::render::{RenderData, SCREEN_HEIGHT, SCREEN_WIDTH};

const TEXTURE_SIZE: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

use memory::{Vram, VRAM_SIZE};

pub struct PPU {
    vram: Vram,
    pixels: RenderData<TEXTURE_SIZE>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: Vram::new(),
            pixels: [[255; 3]; TEXTURE_SIZE],
        }
    }

    pub fn pixels(&self) -> &RenderData<TEXTURE_SIZE> {
        &self.pixels
    }

    pub fn compute(&mut self) {
        let mut x = 0;
        let mut y = 0;
        for k in 0..383 {
            let tile = self.vram.read_8x8_tile(k).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    self.pixels[((x + i) + (y + j) * SCREEN_WIDTH) as usize] =
                        match tile[j as usize][i as usize] {
                            3 => [0; 3],
                            2 => [85; 3],
                            1 => [170; 3],
                            0 => [255; 3],
                            _ => [255; 3],
                        }
                }
            }
            x += 8;
            if x >= 160 {
                x = 0;
                y += 8;
            }
            if y >= 144 {
                return;
            }
        }
    }

    pub fn overwrite_vram(&mut self, data: [u8; VRAM_SIZE as usize]) {
        self.vram.overwrite(data);
    }

    pub fn tilesheet_image(&self) -> RenderData<{ 128 * 192 }> {
        let mut image = [[255; 3]; 128 * 192];
        let mut x = 0;
        let mut y = 0;
        for k in 0..384 {
            let tile = self.vram.read_8x8_tile(k).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    image[((x + i) + (y + j) * 128) as usize] = match tile[j as usize][i as usize] {
                        3 => [0; 3],
                        2 => [85; 3],
                        1 => [170; 3],
                        0 => [255; 3],
                        _ => [255; 3],
                    }
                }
            }
            x += 8;
            if x >= 128 {
                x = 0;
                y += 8;
            }
            if y >= 192 {
                return image;
            }
        }
        image
    }
}

impl Default for PPU {
    fn default() -> PPU {
        PPU::new()
    }
}
