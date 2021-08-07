mod error;
mod memory;

use gb_lcd::render::{RenderData, SCREEN_HEIGHT, SCREEN_WIDTH};

pub const TILESHEET_WIDTH: usize = 128;
pub const TILESHEET_HEIGHT: usize = 192;

use memory::{Vram, VRAM_SIZE};

pub struct PPU {
    vram: Vram,
    pixels: RenderData<SCREEN_WIDTH, SCREEN_HEIGHT>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: Vram::new(),
            pixels: [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn pixels(&self) -> &RenderData<SCREEN_WIDTH, SCREEN_HEIGHT> {
        &self.pixels
    }

    pub fn compute(&mut self) {
        let mut x = 0;
        let mut y = 0;
        for k in 0..383 {
            let tile = self.vram.read_8x8_tile(k).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    self.pixels[y + j][x + i] = match tile[j as usize][i as usize] {
                        3 => [0; 3],
                        2 => [85; 3],
                        1 => [170; 3],
                        0 => [255; 3],
                        _ => [255; 3],
                    }
                }
            }
            x += 8;
            if x >= SCREEN_WIDTH {
                x = 0;
                y += 8;
            }
            if y >= SCREEN_HEIGHT {
                return;
            }
        }
    }

    pub fn overwrite_vram(&mut self, data: &[u8; VRAM_SIZE as usize]) {
        self.vram.overwrite(data);
    }

    pub fn tilesheet_image(&self) -> RenderData<TILESHEET_WIDTH, TILESHEET_HEIGHT> {
        let mut image = [[[255; 3]; TILESHEET_WIDTH]; TILESHEET_HEIGHT];
        let mut x = 0;
        let mut y = 0;
        for k in 0..384 {
            let tile = self.vram.read_8x8_tile(k).unwrap();
            for j in 0..8 {
                for i in 0..8 {
                    image[y * 8 + j][TILESHEET_WIDTH - (x + 1) * 8 + i] =
                        match tile[j as usize][i as usize] {
                            3 => [0; 3],
                            2 => [85; 3],
                            1 => [170; 3],
                            0 => [255; 3],
                            _ => [255; 3],
                        }
                }
            }
            x += 1;
            if x * 8 >= TILESHEET_WIDTH {
                x = 0;
                y += 1;
            }
            if y * 8 >= TILESHEET_HEIGHT {
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
