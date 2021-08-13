mod error;
mod memory;
mod ppu;
mod registers;

pub use ppu::PPU;

pub const TILESHEET_WIDTH: usize = 128;
pub const TILESHEET_HEIGHT: usize = 192;
pub const TILESHEET_TILE_COUNT: usize = 16 * 24;

pub const TILEMAP_DIM: usize = 256;
pub const TILEMAP_TILE_COUNT: usize = 32 * 32;
