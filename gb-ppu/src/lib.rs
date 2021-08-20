mod error;
mod memory;
mod object;
mod ppu;
mod registers;

pub use ppu::PPU;

pub const TILESHEET_WIDTH: usize = 128;
pub const TILESHEET_HEIGHT: usize = 192;
pub const TILESHEET_TILE_COUNT: usize = 16 * 24;

pub const TILEMAP_DIM: usize = 256;
pub const TILEMAP_TILE_COUNT: usize = 32 * 32;

pub const OBJECT_RENDER_WIDTH: usize = gb_lcd::render::SCREEN_WIDTH + 16;
pub const OBJECT_RENDER_HEIGHT: usize = gb_lcd::render::SCREEN_HEIGHT + 32;
