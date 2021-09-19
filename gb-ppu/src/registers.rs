mod control;
mod palette;
mod palettes_mono;
mod ppu_reg;
mod scrolling;
mod stat;
mod window_pos;

pub use control::Control;
pub use palette::Palette;
pub use palettes_mono::PalettesMono;
pub use ppu_reg::PPURegisters;
pub use scrolling::Scrolling;
pub use stat::Stat;
pub use window_pos::WindowPos;

trait Register: Copy + Into<u8> + From<u8> {
    const WRITE_BITS: u8 = 0b1111_1111;

    fn read(&self) -> u8 {
        (*self).into()
    }

    fn write(&mut self, value: u8) {
        *self = (value & Self::WRITE_BITS | (*self).into() & !Self::WRITE_BITS).into();
    }
}

impl Register for u8 {
    fn read(&self) -> u8 {
        *self
    }

    fn write(&mut self, value: u8) {
        *self = value;
    }
}

trait RegisterArray<R: Register, const SIZE: usize>: std::ops::IndexMut<usize, Output = R> {
    const WRITE_POS: [bool; SIZE] = [true; SIZE];

    fn read(&self, pos: usize) -> u8 {
        self[pos].read()
    }

    fn write(&mut self, pos: usize, value: u8) {
        if pos < SIZE && Self::WRITE_POS[pos] {
            self[pos].write(value);
        }
    }
}
