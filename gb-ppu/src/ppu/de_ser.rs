use super::Ppu;
use crate::drawing::{PixelFIFO, PixelFetcher, State};
use crate::memory::{Oam, Vram};
use crate::registers::LcdReg;
use crate::Sprite;
use gb_lcd::render::{SCREEN_HEIGHT, SCREEN_WIDTH};

use std::cell::RefCell;
use std::rc::Rc;

serde_big_array::big_array! { PixelBufferSize; SCREEN_HEIGHT * SCREEN_WIDTH * 3 }

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct PpuDeSer {
    enabled: bool,
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
    lcd_reg: Rc<RefCell<LcdReg>>,
    #[serde(with = "PixelBufferSize")]
    pixels: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    #[serde(with = "PixelBufferSize")]
    next_pixels: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    pixel_fifo: PixelFIFO,
    pixel_fetcher: PixelFetcher,
    state: State,
    scanline_sprites: Vec<Sprite>,
    pixel_discarded: u8,
    scx: u8,
}

impl PpuDeSer {
    pub fn flatten(
        pixels: [[[u8; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
    ) -> [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 3] {
        let mut pixels_flatten = [0; SCREEN_HEIGHT * SCREEN_WIDTH * 3];
        for (l, line) in pixels.into_iter().enumerate() {
            for (p, pixel) in line.into_iter().enumerate() {
                for (b, byte) in pixel.into_iter().enumerate() {
                    pixels_flatten[l * SCREEN_WIDTH + p * 3 + b] = byte;
                }
            }
        }
        pixels_flatten
    }

    pub fn unflatten(
        pixels_flat: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 3],
    ) -> [[[u8; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        let mut pixels = [[[0; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT];
        for (b, byte) in pixels_flat.into_iter().enumerate() {
            let p = b / 3;
            pixels[p / SCREEN_WIDTH][p % SCREEN_WIDTH][b % 3] = byte;
        }
        pixels
    }
}

impl From<Ppu> for PpuDeSer {
    fn from(ppu: Ppu) -> PpuDeSer {
        let pixel_flatten = PpuDeSer::flatten(ppu.pixels);
        let next_pixel_flatten = PpuDeSer::flatten(ppu.next_pixels);
        PpuDeSer {
            enabled: ppu.enabled,
            vram: ppu.vram,
            oam: ppu.oam,
            lcd_reg: ppu.lcd_reg,

            pixels: pixel_flatten,
            next_pixels: next_pixel_flatten,

            pixel_fifo: ppu.pixel_fifo,
            pixel_fetcher: ppu.pixel_fetcher,
            state: ppu.state,
            scanline_sprites: ppu.scanline_sprites,
            pixel_discarded: ppu.pixel_discarded,
            scx: ppu.scx,
        }
    }
}

impl From<PpuDeSer> for Ppu {
    fn from(ppu_flat: PpuDeSer) -> Ppu {
        let pixel = PpuDeSer::unflatten(ppu_flat.pixels);
        let next_pixel = PpuDeSer::unflatten(ppu_flat.next_pixels);
        Ppu {
            enabled: ppu_flat.enabled,
            vram: ppu_flat.vram,
            oam: ppu_flat.oam,
            lcd_reg: ppu_flat.lcd_reg,

            pixels: pixel,
            next_pixels: next_pixel,

            pixel_fifo: ppu_flat.pixel_fifo,
            pixel_fetcher: ppu_flat.pixel_fetcher,
            state: ppu_flat.state,
            scanline_sprites: ppu_flat.scanline_sprites,
            pixel_discarded: ppu_flat.pixel_discarded,
            scx: ppu_flat.scx,
        }
    }
}
