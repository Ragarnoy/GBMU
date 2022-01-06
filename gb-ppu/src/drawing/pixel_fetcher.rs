use super::{Pixel, PixelFIFO};
use crate::memory::Vram;
use crate::registers::{LcdReg, Palette};
use crate::Sprite;
use crate::TILEMAP_TILE_DIM_COUNT;
use std::cell::Cell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

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
    pixels_sprite: VecDeque<Pixel>,
    mode: FetchMode,
    default_mode: FetchMode,
    internal_tick: u8,
    internal_tick_sprite: u8,
    win_line_counter: u8,
    tile: usize,
}

impl PixelFetcher {
    pub const WINDOW_BASE_OFFSET: usize = 7;

    pub fn new() -> Self {
        PixelFetcher {
            pixels: VecDeque::with_capacity(8),
            pixels_sprite: VecDeque::with_capacity(8),
            mode: FetchMode::default(),
            default_mode: FetchMode::default(),
            internal_tick: 0,
            internal_tick_sprite: 0,
            win_line_counter: 0,
            tile: 0,
        }
    }

    pub fn reset(&mut self) {
        self.internal_tick = 0;
        self.pixels.clear();
        self.mode = FetchMode::Background;
        self.default_mode = FetchMode::Background;
    }
    pub fn reset_win_line_counter(&mut self) {
        self.win_line_counter = 0;
    }

    pub fn clear(&mut self) {
        self.internal_tick = 0;
        self.pixels.clear();
    }

    pub fn clear_sprite(&mut self) {
        self.internal_tick_sprite = 0;
        self.pixels_sprite.clear();
    }

    pub fn set_mode_to_sprite(&mut self, sprite: Sprite) {
        self.mode = FetchMode::Sprite(sprite);
    }

    pub fn set_mode_to_default(&mut self) {
        self.mode = self.default_mode;
    }

    pub fn set_default_mode(&mut self, mode: FetchMode) {
        if self.default_mode == FetchMode::Background && mode == FetchMode::Window {
            self.win_line_counter += 1;
        }
        self.default_mode = mode;
    }

    pub fn default_mode(&self) -> FetchMode {
        self.default_mode
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
        pixels_not_drawn: usize,
        scx: usize,
    ) {
        let tick = if let FetchMode::Sprite(_) = self.mode {
            self.internal_tick_sprite
        } else {
            self.internal_tick
        };

        if tick % 2 == 1 {
            // the fetcher take 2 tick to process one step
            match tick / 2 {
                0 => self.get_tile_index(vram, lcd_reg, y, x, pixels_not_drawn, scx), // get the tile index.
                1 => {}                                     // get the high data of the tile
                2 => self.fetch_full_row(vram, lcd_reg, y), // get the low data of the tile, the pixels are ready after this step
                _ => {}                                     // idle on the last step
            }
        }
        if let FetchMode::Sprite(_) = self.mode {
            self.internal_tick_sprite = (self.internal_tick_sprite + 1) % 8
        } else {
            self.internal_tick = (self.internal_tick + 1) % 8
        };
    }

    fn get_tile_index(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        y: usize,
        x: usize,
        pixels_not_drawn: usize,
        scx: usize,
    ) {
        match self.mode {
            FetchMode::Background => {
                let scy = lcd_reg.scrolling.scy as usize;
                let tile_x = ((x + pixels_not_drawn + scx + 247) % 255) / 8;
                let tile_y = ((y + scy) % 255) / 8;
                let tile_pos_in_vram = tile_x + tile_y * TILEMAP_TILE_DIM_COUNT;
                self.tile = vram
                    .get_map_tile_index(
                        tile_pos_in_vram,
                        lcd_reg.control.bg_tilemap_area(),
                        lcd_reg.control.bg_win_tiledata_area(),
                    )
                    .unwrap_or_else(|err| {
                        log::error!("Failed to get background tile index: {}", err);
                        0xFF
                    });
            }
            FetchMode::Window => {
                let wx = lcd_reg.window_pos.wx as usize;
                let tile_x = ((x + pixels_not_drawn + Self::WINDOW_BASE_OFFSET - wx) % 255) / 8;
                let tile_y = ((self.win_line_counter - 1) as usize) / 8;
                let tile_pos_in_vram = tile_x + tile_y * TILEMAP_TILE_DIM_COUNT;
                self.tile = vram
                    .get_map_tile_index(
                        tile_pos_in_vram,
                        lcd_reg.control.win_tilemap_area(),
                        lcd_reg.control.bg_win_tiledata_area(),
                    )
                    .unwrap_or_else(|err| {
                        log::error!("Failed to get window tile index: {}", err);
                        0xFF
                    });
            }
            FetchMode::Sprite(_) => {}
        };
    }

    fn fetch_full_row(
        &mut self,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        line: usize,
    ) {
        let scy = lcd_reg.scrolling.scy as usize;
        match self.mode {
            FetchMode::Background => {
                let tile_line = (line + scy) % 8;
                self.fetch_bg_win_row(vram, lcd_reg, tile_line)
            }
            FetchMode::Window => {
                let tile_line = ((self.win_line_counter - 1) as usize) % 8;
                self.fetch_bg_win_row(vram, lcd_reg, tile_line)
            }
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
                if lcd_reg.control.bg_win_enable() {
                    for color_id in row {
                        self.pixels.push_front(Pixel::new(
                            color_id,
                            lcd_reg.pal_mono.bg().clone(),
                            false,
                        ));
                    }
                } else {
                    for _ in row {
                        self.pixels.push_front(Pixel::new(
                            0,
                            Rc::new(Cell::new(Palette::new_background())),
                            false,
                        ));
                    }
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
                    self.pixels_sprite.push_front(Pixel::new(
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
        match self.mode {
            FetchMode::Sprite(_) => {
                if self.pixels_sprite.len() >= 8 && self.internal_tick_sprite % 2 == 1 {
                    self.mix_to_fifo(fifo)
                } else {
                    false
                }
            }
            _ => {
                if self.pixels.len() >= 8 && self.internal_tick % 2 == 1 {
                    self.append_to_fifo(fifo)
                } else {
                    false
                }
            }
        }
    }

    fn append_to_fifo(&mut self, fifo: &mut PixelFIFO) -> bool {
        if fifo.append(&mut self.pixels) {
            self.clear();
            true
        } else {
            false
        }
    }

    fn mix_to_fifo(&mut self, fifo: &mut PixelFIFO) -> bool {
        if fifo.mix(&self.pixels_sprite) {
            self.clear_sprite();
            self.set_mode_to_default();
            true
        } else {
            false
        }
    }
}
