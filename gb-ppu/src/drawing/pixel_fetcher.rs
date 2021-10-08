use super::{Pixel, PixelFIFO};
use crate::Sprite;
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

    pub fn set_mode(&mut self, mode: FetchMode) {
        if self.mode != mode {
            self.internal_tick = 0;
            self.pixels.clear();
        }
        self.mode = mode;
    }

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
