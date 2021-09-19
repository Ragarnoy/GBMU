mod color;
mod error;
mod memory;
mod object;
mod ppu;
mod registers;
#[cfg(test)]
mod test_tools;

pub use memory::PPUMem;
pub use ppu::PPU;
pub use registers::PPURegisters;

pub const TILESHEET_WIDTH: usize = 128;
pub const TILESHEET_HEIGHT: usize = 192;
pub const TILESHEET_TILE_COUNT: usize = 16 * 24;

pub const TILEMAP_DIM: usize = 256;
pub const TILEMAP_TILE_COUNT: usize = 32 * 32;

pub const OBJECT_RENDER_WIDTH: usize = gb_lcd::render::SCREEN_WIDTH + 16;
pub const OBJECT_RENDER_HEIGHT: usize = gb_lcd::render::SCREEN_HEIGHT + 32;

const OBJECT_LIST_PER_LINE: usize = 8;
pub const OBJECT_LIST_RENDER_WIDTH: usize = OBJECT_LIST_PER_LINE * 8;
pub const OBJECT_LIST_RENDER_HEIGHT: usize =
    (memory::Oam::OBJECT_COUNT / OBJECT_LIST_PER_LINE) * 16;

const UNDEFINED_VALUE: u8 = 0xFF;
