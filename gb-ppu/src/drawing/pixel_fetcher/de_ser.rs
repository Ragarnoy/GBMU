use super::super::pixel::de_ser::PixelDeSer;
use super::{FetchMode, PixelFetcher};
use crate::registers::LcdReg;
use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct PixelFetcherDeSer {
    pixels: VecDeque<PixelDeSer>,
    pixels_sprite: VecDeque<PixelDeSer>,
    mode: FetchMode,
    default_mode: FetchMode,
    internal_tick: u8,
    internal_tick_sprite: u8,
    win_line_counter: u8,
    tile: usize,
}

impl PixelFetcherDeSer {
    pub fn from_fetcher(fetcher: PixelFetcher, lcd_reg: impl Deref<Target = LcdReg>) -> Self {
        let pixels = fetcher
            .pixels
            .into_iter()
            .map(|p| PixelDeSer::from_pixel(p, &lcd_reg.pal_mono))
            .collect();
        let pixels_sprite = fetcher
            .pixels_sprite
            .into_iter()
            .map(|p| PixelDeSer::from_pixel(p, &lcd_reg.pal_mono))
            .collect();
        PixelFetcherDeSer {
            pixels,
            pixels_sprite,
            mode: fetcher.mode,
            default_mode: fetcher.default_mode,
            internal_tick: fetcher.internal_tick,
            internal_tick_sprite: fetcher.internal_tick_sprite,
            win_line_counter: fetcher.win_line_counter,
            tile: fetcher.tile,
        }
    }

    pub fn into_fetcher(self, lcd_reg: impl Deref<Target = LcdReg>) -> PixelFetcher {
        let pixels = self
            .pixels
            .into_iter()
            .map(|p| p.into_pixel(&lcd_reg.pal_mono))
            .collect();
        let pixels_sprite = self
            .pixels_sprite
            .into_iter()
            .map(|p| p.into_pixel(&lcd_reg.pal_mono))
            .collect();
        PixelFetcher {
            pixels,
            pixels_sprite,
            mode: self.mode,
            default_mode: self.default_mode,
            internal_tick: self.internal_tick,
            internal_tick_sprite: self.internal_tick_sprite,
            win_line_counter: self.win_line_counter,
            tile: self.tile,
        }
    }
}
