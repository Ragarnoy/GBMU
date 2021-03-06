mod color;
mod drawing;
mod error;
mod memory;
mod ppu;
mod registers;
mod sprite;

#[cfg(test)]
mod test_tools;

use color::Color;
pub use drawing::Mode;
pub use memory::PPUMem;
pub use ppu::{ImageRGB, Ppu};
pub use registers::PPURegisters;
use sprite::Sprite;

pub const GB_SCREEN_WIDTH: usize = 160;
pub const GB_SCREEN_HEIGHT: usize = 144;

pub const TILESHEET_WIDTH: usize = 128 * 2;
pub const TILESHEET_HEIGHT: usize = 192;
pub const TILESHEET_TILE_COUNT: usize = 16 * 24;

pub const TILEMAP_TILE_DIM_COUNT: usize = 32;
pub const TILEMAP_DIM: usize = 256;
pub const TILEMAP_TILE_COUNT: usize = TILEMAP_TILE_DIM_COUNT * TILEMAP_TILE_DIM_COUNT;

pub const SPRITE_RENDER_WIDTH: usize = GB_SCREEN_WIDTH + 16;
pub const SPRITE_RENDER_HEIGHT: usize = GB_SCREEN_HEIGHT + TILEMAP_TILE_DIM_COUNT;

const SPRITE_LIST_PER_LINE: usize = 8;
pub const SPRITE_LIST_RENDER_WIDTH: usize = SPRITE_LIST_PER_LINE * 8;
pub const SPRITE_LIST_RENDER_HEIGHT: usize =
    (memory::Oam::SPRITE_COUNT / SPRITE_LIST_PER_LINE) * 16;

const UNDEFINED_VALUE: u8 = 0xFF;
