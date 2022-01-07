use crate::drawing::{FetchMode, Mode, PixelFIFO, PixelFetcher, State};
use crate::memory::{Lock, Lockable, Oam, PPUMem, Vram};
use crate::registers::{LcdReg, PPURegisters};
use crate::Color;
use crate::Sprite;
use crate::{
    SPRITE_LIST_PER_LINE, SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH,
    SPRITE_RENDER_HEIGHT, SPRITE_RENDER_WIDTH, TILEMAP_DIM, TILEMAP_TILE_COUNT, TILESHEET_HEIGHT,
    TILESHEET_TILE_COUNT, TILESHEET_WIDTH,
};
use gb_bus::{Area, Bus};
use gb_clock::{Tick, Ticker};
use gb_lcd::render::{RenderData, SCREEN_HEIGHT, SCREEN_WIDTH};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct PixelBorder {
    pub pos: usize,
    pub sc: usize,
    pub sc_bot: usize,
}

macro_rules! view_border {
    ($main:ident, $other:ident) => {
        (($main.pos == $main.sc || $main.pos == $main.sc_bot)
            && (($other.sc < $other.sc_bot
                && $other.pos >= $other.sc
                && $other.pos <= $other.sc_bot)
                || ($other.sc > $other.sc_bot
                    && ($other.pos >= $other.sc) ^ ($other.pos <= $other.sc_bot))))
    };
}

/// The Pixel Process Unit is in charge of selecting the pixel to be displayed on the lcd screen.
///
/// It owns the VRAM and the OAM, as well as a few registers.
pub struct Ppu {
    enabled: bool,
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
    lcd_reg: Rc<RefCell<LcdReg>>,
    pixels: RenderData<SCREEN_WIDTH, SCREEN_HEIGHT>,
    next_pixels: RenderData<SCREEN_WIDTH, SCREEN_HEIGHT>,
    pixel_fifo: PixelFIFO,
    pixel_fetcher: PixelFetcher,
    state: State,
    scanline_sprites: Vec<Sprite>,
    pixel_discarded: u8,
    scx: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            enabled: false,
            vram: Rc::new(RefCell::new(Vram::new())),
            oam: Rc::new(RefCell::new(Oam::new())),
            lcd_reg: Rc::new(RefCell::new(LcdReg::new())),
            pixels: [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
            next_pixels: [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT],
            pixel_fifo: PixelFIFO::new(),
            pixel_fetcher: PixelFetcher::new(),
            state: State::new(),
            scanline_sprites: Vec::with_capacity(10),
            pixel_discarded: 0,
            scx: 0,
        }
    }

    /// Build and return a [PPUMem] struct to access/modify the memory of this ppu instance.
    pub fn memory(&self) -> PPUMem {
        PPUMem::new(Rc::clone(&self.vram), Rc::clone(&self.oam))
    }

    /// Build and return a [PPURegisters] struct to access/modify the registers of this ppu instance.
    pub fn registers(&self) -> PPURegisters {
        PPURegisters::new(Rc::clone(&self.lcd_reg))
    }

    pub fn pixels(&self) -> &RenderData<SCREEN_WIDTH, SCREEN_HEIGHT> {
        &self.pixels
    }

    pub fn compute(&mut self) {
        for j in 0..SCREEN_HEIGHT {
            for i in 0..SCREEN_WIDTH {
                if i == 0 || i == SCREEN_WIDTH - 1 || j == 0 || j == SCREEN_HEIGHT - 1 {
                    self.pixels[j][i] = [255, 0, 0];
                } else if (i + j) % 2 == 0 {
                    self.pixels[j][i] = [0; 3];
                } else {
                    self.pixels[j][i] = [255; 3];
                }
            }
        }
    }

    /// Create an image of the current tilesheet.
    ///
    /// This function is used for debugging purpose.
    pub fn tilesheet_image(&self) -> RenderData<TILESHEET_WIDTH, TILESHEET_HEIGHT> {
        let mut image = [[[255; 3]; TILESHEET_WIDTH]; TILESHEET_HEIGHT];
        let mut x = 0;
        let mut y = 0;
        let vram = self.vram.borrow();
        let lcd_reg = self.lcd_reg.borrow();
        for k in 0..TILESHEET_TILE_COUNT {
            let tile = vram.read_8x8_tile(k).unwrap();
            for (j, row) in tile.iter().enumerate() {
                for (i, pixel) in row.iter().rev().enumerate() {
                    image[y * 8 + j][x * 8 + i] = lcd_reg
                        .pal_mono
                        .bg()
                        .get()
                        .get_color(*pixel)
                        .unwrap_or_default()
                        .into();
                }
            }
            x += 1;
            if x * 8 >= TILESHEET_WIDTH {
                x = 0;
                y += 1;
            }
        }
        image
    }

    /// Create an image of the current tilemap.
    ///
    /// This function is used for debugging purpose.
    pub fn tilemap_image(&self, window: bool) -> RenderData<TILEMAP_DIM, TILEMAP_DIM> {
        let mut image = [[[255; 3]; TILEMAP_DIM]; TILEMAP_DIM];
        let mut x = 0;
        let mut y = 0;
        let vram = self.vram.borrow();
        let lcd_reg = self.lcd_reg.borrow();
        let scx = lcd_reg.scrolling.scx as usize;
        let scx_bot = (scx + 160) & 0xff;
        let scy = lcd_reg.scrolling.scy as usize;
        let scy_bot = (scy + 144) & 0xff;
        for k in 0..TILEMAP_TILE_COUNT {
            let index = vram
                .get_map_tile_index(
                    k,
                    if !window {
                        lcd_reg.control.bg_tilemap_area()
                    } else {
                        lcd_reg.control.win_tilemap_area()
                    },
                    lcd_reg.control.bg_win_tiledata_area(),
                )
                .unwrap();
            let tile = vram.read_8x8_tile(index).unwrap();
            for (j, row) in tile.iter().enumerate() {
                for (i, pixel) in row.iter().rev().enumerate() {
                    let pix_y = y * 8 + j;
                    let pix_x = x * 8 + i;
                    let pixel_y = PixelBorder {
                        pos: pix_y,
                        sc: scy,
                        sc_bot: scy_bot,
                    };
                    let pixel_x = PixelBorder {
                        pos: pix_x,
                        sc: scx,
                        sc_bot: scx_bot,
                    };
                    image[pix_y][pix_x] = lcd_reg
                        .pal_mono
                        .bg()
                        .get()
                        .get_color(*pixel)
                        .unwrap_or_default()
                        .into();
                    if !window && (view_border!(pixel_y, pixel_x) || view_border!(pixel_x, pixel_y))
                    {
                        let neg_pixel = image[pix_y][pix_x];
                        image[pix_y][pix_x] = [!neg_pixel[0], !neg_pixel[1], !neg_pixel[2]];
                    }
                }
            }
            x += 1;
            if x * 8 >= TILEMAP_DIM {
                x = 0;
                y += 1;
            }
        }
        image
    }

    /// Create an image of the currents sprites placed relatively to the viewport.
    ///
    /// This function is used for debugging purpose.
    pub fn sprites_image(
        &self,
        invert_pixel: bool,
    ) -> RenderData<SPRITE_RENDER_WIDTH, SPRITE_RENDER_HEIGHT> {
        let mut image = [[[255; 3]; SPRITE_RENDER_WIDTH]; SPRITE_RENDER_HEIGHT];
        let sprites = self
            .oam
            .borrow()
            .collect_all_sprites()
            .expect("failed to collect sprites for image");
        let vram = self.vram.borrow();
        let lcd_reg = self.lcd_reg.borrow();
        let height = if lcd_reg.control.obj_size() { 16 } else { 8 };
        for sprite in sprites {
            let x = sprite.x_pos().min(SPRITE_RENDER_WIDTH as u8 - 8) as usize;
            let y = sprite.y_pos().min(SPRITE_RENDER_HEIGHT as u8 - 16) as usize;
            for j in 0..height {
                let pixels_values = sprite
                    .get_pixels_row(j, &vram, lcd_reg.control.obj_size(), lcd_reg.pal_mono.obj())
                    .expect("invalid line passed");
                let y_img = y + j;
                for (i, (pixel_value, pixel_color)) in pixels_values.iter().rev().enumerate() {
                    if *pixel_value != 0 {
                        let x_img = x + i;
                        let mut rgb: [u8; 3] = (*pixel_color).into();
                        if invert_pixel {
                            rgb[0] = 255 - rgb[0];
                            rgb[1] = 255 - rgb[1];
                            rgb[2] = 255 - rgb[2];
                        }
                        image[y_img][x_img] = rgb;
                    }
                }
            }
        }
        // draw screen outline
        for (y, column) in image.iter_mut().enumerate() {
            for (x, pixel) in column.iter_mut().enumerate() {
                if ((x == 7 || x == SPRITE_RENDER_WIDTH - 8)
                    && y >= 15
                    && y <= SPRITE_RENDER_HEIGHT - 16)
                    || ((y == 15 || y == SPRITE_RENDER_HEIGHT - 16)
                        && x >= 7
                        && x <= SPRITE_RENDER_WIDTH - 8)
                {
                    *pixel = [!pixel[0], !pixel[1], !pixel[2]];
                }
            }
        }
        image
    }

    /// Create an image of the currents sprites.
    ///
    /// This function is used for debugging purpose.
    pub fn sprites_list_image(
        &self,
        invert_pixel: bool,
    ) -> RenderData<SPRITE_LIST_RENDER_WIDTH, SPRITE_LIST_RENDER_HEIGHT> {
        let mut image = [[[255; 3]; SPRITE_LIST_RENDER_WIDTH]; SPRITE_LIST_RENDER_HEIGHT];
        let sprites = self
            .oam
            .borrow()
            .collect_all_sprites()
            .expect("failed to collect sprites for image");
        let vram = self.vram.borrow();
        let lcd_reg = self.lcd_reg.borrow();
        let height = if lcd_reg.control.obj_size() { 16 } else { 8 };
        for (r, sprite) in sprites.iter().enumerate() {
            let x = (r % SPRITE_LIST_PER_LINE) * 8;
            let y = (r / SPRITE_LIST_PER_LINE) * 16;
            for j in 0..height {
                let pixels_values = sprite
                    .get_pixels_row(j, &vram, lcd_reg.control.obj_size(), lcd_reg.pal_mono.obj())
                    .expect("invalid line passed");
                let y_img = y + j;
                for (i, (pixel_value, pixel_color)) in pixels_values.iter().rev().enumerate() {
                    if *pixel_value != 0 {
                        let x_img = x + i;
                        let mut rgb: [u8; 3] = (*pixel_color).into();
                        if invert_pixel {
                            rgb[0] = 255 - rgb[0];
                            rgb[1] = 255 - rgb[1];
                            rgb[2] = 255 - rgb[2];
                        }
                        image[y_img][x_img] = rgb;
                    }
                }
            }
        }
        image
    }

    fn vblank(&mut self, _adr_bus: &mut dyn Bus<u8>) {
        if self.state.line() == State::LAST_LINE && self.state.step() == State::LAST_STEP {
            std::mem::swap(&mut self.pixels, &mut self.next_pixels);
            self.next_pixels = [[[255; 3]; SCREEN_WIDTH]; SCREEN_HEIGHT];
            self.pixel_fetcher.reset_win_line_counter();
        }
    }

    fn hblank(&mut self, adr_bus: &mut dyn Bus<u8>) {
        self.unlock_mem(adr_bus);
    }

    fn unlock_mem(&mut self, adr_bus: &mut dyn Bus<u8>) {
        let unlock_oam: bool;
        if let Ok(oam) = self.oam.try_borrow() {
            if let Some(Lock::Ppu) = oam.get_lock() {
                unlock_oam = true;
            } else {
                unlock_oam = false;
            }
        } else {
            log::error!("Oam borrow failed for ppu to unlock");
            unlock_oam = false;
        }
        if unlock_oam {
            adr_bus.unlock(Area::Oam);
        }

        let unlock_vram: bool;
        if let Ok(vram) = self.vram.try_borrow() {
            if let Some(Lock::Ppu) = vram.get_lock() {
                unlock_vram = true;
            } else {
                unlock_vram = false;
            }
        } else {
            log::error!("Vram borrow failed for ppu to unlock");
            unlock_vram = false;
        }
        if unlock_vram {
            adr_bus.unlock(Area::Vram);
        }
    }

    fn oam_fetch(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if let Ok(lcd_reg) = self.lcd_reg.try_borrow() {
            let mut lock: Option<Lock>;
            let step: u16;
            if let Ok(oam) = self.oam.try_borrow() {
                lock = oam.get_lock();
                step = self.state.step();
            } else {
                log::error!("Oam borrow failed for ppu in mode 2");
                return;
            }

            if lock.is_none() {
                // init mode 2
                adr_bus.lock(Area::Oam, Lock::Ppu);
                lock = Some(Lock::Ppu);
                self.scanline_sprites.clear();
            }
            if let Some(Lock::Ppu) = lock {
                if lcd_reg.control.obj_enable() {
                    let oam = self.oam.borrow();
                    if step % 2 == 1 {
                        let sprite_pos = step as usize / 2;

                        match oam.read_sprite(sprite_pos) {
                            Err(err) => log::error!("Error while reading sprite: {}", err),
                            Ok(sprite) => {
                                let scanline = self.state.line() + 16;
                                let top = sprite.y_pos().min(160);
                                let bot = top + if lcd_reg.control.obj_size() { 16 } else { 8 };

                                if scanline >= top && scanline < bot {
                                    for i in 0..self.scanline_sprites.len() {
                                        let scan_sprite = self.scanline_sprites[i];

                                        if sprite.x_pos() < scan_sprite.x_pos() {
                                            self.scanline_sprites.insert(i, sprite);
                                            if self.scanline_sprites.len() > 10 {
                                                self.scanline_sprites.pop();
                                            }
                                            return;
                                        }
                                    }
                                    if self.scanline_sprites.len() < 10 {
                                        self.scanline_sprites.push(sprite);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            log::error!("Lcd reg borrow failed for ppu in mode 2");
        }
    }

    fn pixel_drawing(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if let Ok(lcd_reg) = self.lcd_reg.try_borrow() {
            let mut lock: Option<Lock>;
            let mut x: u8;
            let y: u8;
            if let Ok(vram) = self.vram.try_borrow() {
                lock = vram.get_lock();
                x = self.state.pixel_drawn();
                y = self.state.line();
            } else {
                log::error!("Vram borrow failed for ppu in mode 3");
                return;
            }

            if lock.is_none() {
                // init mode 3
                adr_bus.lock(Area::Vram, Lock::Ppu);
                lock = Some(Lock::Ppu);
                self.pixel_fetcher.reset();
                self.pixel_fifo.clear();
                self.state.clear_pixel_count();
                // reverse the sprites order so the ones on the left of the viewport are the first to pop
                self.scanline_sprites = self.scanline_sprites.drain(0..).rev().collect();
                self.scx = lcd_reg.scrolling.scx;
                self.pixel_discarded = 0;
                Self::check_next_pixel_mode(
                    &lcd_reg,
                    &mut self.pixel_fetcher,
                    &mut self.pixel_fifo,
                    &mut self.scanline_sprites,
                    (x, y),
                    self.pixel_discarded,
                    (self.scx & 7) + 8,
                );
            }
            let pixel_offset = (self.scx & 7) + 8;
            if let Some(Lock::Ppu) = lock {
                let vram = self.vram.borrow();
                if self.pixel_fifo.enabled && x < SCREEN_WIDTH as u8 {
                    if let Some(pixel) = self.pixel_fifo.pop() {
                        if self.pixel_fetcher.mode() == FetchMode::Window {
                            self.pixel_discarded = pixel_offset;
                        }
                        if self.state.pixel_drawn() > 0 || self.pixel_discarded >= pixel_offset {
                            self.next_pixels[y as usize][x as usize] = Color::from(pixel).into();
                            self.state.draw_pixel();
                            x += 1;
                        } else {
                            self.pixel_discarded += 1;
                        }
                        Self::check_next_pixel_mode(
                            &lcd_reg,
                            &mut self.pixel_fetcher,
                            &mut self.pixel_fifo,
                            &mut self.scanline_sprites,
                            (x, y),
                            self.pixel_discarded,
                            pixel_offset,
                        );
                    };
                }
                let pixels_not_drawn = if self.pixel_fetcher.mode() != FetchMode::Window {
                    self.pixel_fifo.count() + self.pixel_discarded.min(8) as usize
                } else {
                    self.pixel_fifo.count()
                };
                self.pixel_fetcher.fetch(
                    &vram,
                    &lcd_reg,
                    y as usize,
                    x as usize,
                    pixels_not_drawn,
                    self.scx as usize & 0xff,
                );
                if self.pixel_fetcher.push_to_fifo(&mut self.pixel_fifo) {
                    Self::check_next_pixel_mode(
                        &lcd_reg,
                        &mut self.pixel_fetcher,
                        &mut self.pixel_fifo,
                        &mut self.scanline_sprites,
                        (x, y),
                        self.pixel_discarded,
                        pixel_offset,
                    );
                }
            }
        } else {
            log::error!("Lcd_reg borrow failed for ppu in mode 3");
        }
    }

    fn check_next_pixel_mode(
        lcd_reg: &dyn Deref<Target = LcdReg>,
        pixel_fetcher: &mut PixelFetcher,
        pixel_fifo: &mut PixelFIFO,
        sprites: &mut Vec<Sprite>,
        cursor: (u8, u8),
        pixels_discarded: u8,
        pixel_offset: u8,
    ) {
        let (x, y) = cursor;
        pixel_fifo.enabled = true;

        // check if w switch to window mode
        if let FetchMode::Background = pixel_fetcher.default_mode() {
            if lcd_reg.control.win_enable()
                && lcd_reg.window_pos.wy <= y
                && lcd_reg.window_pos.wx <= x + PixelFetcher::WINDOW_BASE_OFFSET as u8
            {
                pixel_fetcher.clear();
                pixel_fetcher.set_default_mode(FetchMode::Window);
                pixel_fetcher.set_mode_to_default();
                pixel_fifo.clear();
                return;
            }
        }

        // check for sprite eventually
        if pixel_fifo.count() >= 8 {
            if let Some(sprite) = sprites.pop() {
                let viewport_x_at_sprite_scale = x + Sprite::HORIZONTAL_OFFSET;
                let pixels_to_skip_before_viewport = pixel_offset - pixels_discarded;
                if sprite.x_pos() == viewport_x_at_sprite_scale - pixels_to_skip_before_viewport {
                    pixel_fetcher.set_mode_to_sprite(sprite);
                    pixel_fifo.enabled = false;
                } else {
                    sprites.push(sprite);
                }
            }
        }
    }

    fn pixel_drawing_disabled(&mut self) {
        if self.state.pixel_drawn() < 160 {
            self.state.draw_pixel();
        }
    }
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu::new()
    }
}

impl Ticker for Ppu {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        let enable = self.lcd_reg.borrow().control.ppu_enable();
        if self.enabled && !enable {
            log::info!("disabling lcd");
            self.unlock_mem(adr_bus);
            self.enabled = false;
        }
        if !self.enabled && enable && self.state.mode() == Mode::VBlank {
            log::info!("enabling lcd");
            self.enabled = true;
        }

        match self.state.mode() {
            Mode::OAMFetch => {
                if self.enabled {
                    self.oam_fetch(adr_bus)
                }
            }
            Mode::PixelDrawing => {
                if self.enabled {
                    self.pixel_drawing(adr_bus)
                } else {
                    self.pixel_drawing_disabled()
                }
            }
            Mode::HBlank => {
                if self.enabled {
                    self.hblank(adr_bus)
                }
            }
            Mode::VBlank => self.vblank(adr_bus),
        }
        // update state after executing tick
        let lcd_reg = self.lcd_reg.try_borrow_mut().ok();
        self.state.update(lcd_reg, adr_bus);
    }
}
