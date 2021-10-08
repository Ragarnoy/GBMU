use super::{Pixel, PixelFIFO};
use crate::memory::Vram;
use crate::Sprite;
use std::cell::RefMut;
use std::collections::VecDeque;

#[derive(Eq, PartialEq)]
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
    pub mode: FetchMode,
    internal_tick: u8,
}

impl PixelFetcher {
    pub fn new() -> Self {
        PixelFetcher {
            pixels: VecDeque::with_capacity(8),
            mode: FetchMode::default(),
            internal_tick: 0,
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

    pub fn fetch(&mut self, vram: RefMut<Vram>) {
        if self.internal_tick % 2 == 1 {
            // the fetcher take 2 tick to process one step
            match self.internal_tick / 2 {
                0 => {}                         // get the tile index.
                1 => {}                         // get the high data of the tile
                2 => self.fetch_full_row(vram), // get the low data of the tile, the pixels are ready after this step
                _ => {}                         // idle on the last step
            }
        }
        self.internal_tick = (self.internal_tick + 1) % 8
    }

    fn fetch_full_row(&mut self, vram: RefMut<Vram>) {}

    pub fn push_to_fifo(&mut self, fifo: &mut PixelFIFO) {
        if self.pixels.len() >= 8 && self.internal_tick % 2 == 1 {
            match self.mode {
                FetchMode::Background => self.append_to_fifo(fifo),
                FetchMode::Window => self.append_to_fifo(fifo),
                FetchMode::Sprite(_) => self.mix_to_fifo(fifo),
            }
        }
    }

    fn append_to_fifo(&mut self, fifo: &mut PixelFIFO) {
        if let Some(unused_pixels) = fifo.append(self.pixels.drain(0..).collect()) {
            self.pixels = unused_pixels;
        }
    }

    fn mix_to_fifo(&mut self, fifo: &mut PixelFIFO) {
        if let Some(unused_pixels) = fifo.mix(self.pixels.drain(0..).collect()) {
            self.pixels = unused_pixels;
        }
    }
}
