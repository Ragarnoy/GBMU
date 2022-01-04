use super::{Pixel, PixelFIFO};
use crate::memory::Vram;
use crate::registers::LcdReg;
use crate::Sprite;
use crate::TILEMAP_TILE_DIM_COUNT;
use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FetchMode {
    Background,
    Window,
    Sprite(Sprite),
}

impl Default for FetchMode {
    fn default() -> Self {
        FetchMode::Background
    }
}

pub struct PixelFetcher {
    pixels: VecDeque<Pixel>,
    mode: FetchMode,
    internal_tick: u8,
    tile: usize,
}

impl PixelFetcher {
    pub const WINDOW_BASE_OFFSET: usize = 7;

    pub fn new() -> Self {
        PixelFetcher {
            pixels: VecDeque::with_capacity(8),
            mode: FetchMode::default(),
            internal_tick: 0,
            tile: 0,
        }
    }

    pub fn clear(&mut self) {
        self.internal_tick = 0;
        self.pixels.clear();
        self.mode = FetchMode::Background;
    }

    pub fn set_mode(&mut self, mode: FetchMode) {
        if self.mode != mode {
            self.internal_tick = 0;
            self.pixels.clear();
        }
        self.mode = mode;
    }

    pub fn mode(&self) -> FetchMode {
        self.mode
    }

    pub fn fetch(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        y: usize,
        x: usize,
        x_queued: usize,
    ) {
        if self.internal_tick % 2 == 1 {
            // the fetcher take 2 tick to process one step
            match self.internal_tick / 2 {
                0 => self.get_tile_index(vram, lcd_reg, y, x, x_queued), // get the tile index.
                1 => {}                                     // get the high data of the tile
                2 => self.fetch_full_row(vram, lcd_reg, y), // get the low data of the tile, the pixels are ready after this step
                _ => {}                                     // idle on the last step
            }
        }
        self.internal_tick = (self.internal_tick + 1) % 8
    }

    fn get_tile_index(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        y: usize,
        x: usize,
        x_queued: usize,
    ) {
        self.tile = match self.mode {
            FetchMode::Background => {
                let scx = lcd_reg.scrolling.scx as usize;
                let scy = lcd_reg.scrolling.scy as usize;
                vram.get_map_tile_index(
                    ((x + x_queued + scx) % 255) / 8
                        + ((y + scy) % 255) / 8 * TILEMAP_TILE_DIM_COUNT,
                    lcd_reg.control.bg_tilemap_area(),
                    lcd_reg.control.bg_win_tiledata_area(),
                )
                .unwrap_or_else(|err| {
                    log::error!("Failed to get background tile index: {}", err);
                    0xFF
                })
            }
            FetchMode::Window => {
                let wx = lcd_reg.window_pos.wx as usize;
                let wy = lcd_reg.window_pos.wy as usize;
                vram.get_map_tile_index(
                    ((x + x_queued + Self::WINDOW_BASE_OFFSET - wx) % 255) / 8
                        + ((y - wy) % 255) / 8 * TILEMAP_TILE_DIM_COUNT,
                    lcd_reg.control.win_tilemap_area(),
                    lcd_reg.control.bg_win_tiledata_area(),
                )
                .unwrap_or_else(|err| {
                    log::error!("Failed to get window tile index: {}", err);
                    0xFF
                })
            }
            FetchMode::Sprite(sprite) => sprite.tile_index() as usize,
        };
    }

    fn fetch_full_row(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        line: usize,
    ) {
        let scy = lcd_reg.scrolling.scy as usize;
        let wy = lcd_reg.window_pos.wy as usize;
        match self.mode {
            FetchMode::Background => self.fetch_bg_win_row(vram, lcd_reg, (line + scy) % 8),
            FetchMode::Window => self.fetch_bg_win_row(vram, lcd_reg, (line + wy) % 8),
            FetchMode::Sprite(sprite) => self.fetch_spr_row(vram, lcd_reg, line, &sprite),
        }
    }

    fn fetch_bg_win_row(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        line: usize,
    ) {
        match vram.read_tile_line(self.tile, line) {
            Ok(row) => {
                for color_id in row {
                    self.pixels.push_front(Pixel::new(
                        color_id,
                        lcd_reg.pal_mono.bg().clone(),
                        false,
                    ));
                }
            }
            Err(err) => log::error!("Failed to fetch background/window row of pixel: {}", err),
        }
    }

    fn fetch_spr_row(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        line: usize,
        sprite: &Sprite,
    ) {
        match sprite.get_pixels_row(
            line + Sprite::VERTICAL_OFFSET as usize - sprite.y_pos() as usize,
            vram,
            lcd_reg.control.obj_size(),
            lcd_reg.pal_mono.obj(),
        ) {
            Ok(row) => {
                for (color_id, _) in row {
                    self.pixels.push_front(Pixel::new(
                        color_id,
                        sprite.get_palette(lcd_reg.pal_mono.obj()).clone(),
                        sprite.bg_win_priority(),
                    ));
                }
            }
            Err(err) => log::error!("Failed to fetch sprite row of pixel: {}", err),
        }
    }

    pub fn push_to_fifo(&mut self, fifo: &mut PixelFIFO) -> bool {
        if self.pixels.len() >= 8 && self.internal_tick % 2 == 1 {
            match self.mode {
                FetchMode::Background => self.append_to_fifo(fifo),
                FetchMode::Window => self.append_to_fifo(fifo),
                FetchMode::Sprite(_) => self.mix_to_fifo(fifo),
            }
        } else {
            false
        }
    }

    fn append_to_fifo(&mut self, fifo: &mut PixelFIFO) -> bool {
        if fifo.append(&mut self.pixels) {
            self.pixels.clear();
        }
        false
    }

    fn mix_to_fifo(&mut self, fifo: &mut PixelFIFO) -> bool {
        if fifo.mix(&self.pixels) {
            self.pixels.clear();
            true
        } else {
            false
        }
    }
}
