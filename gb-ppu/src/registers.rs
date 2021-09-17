mod control;
mod palette;
mod ppu_reg;
mod stat;

pub use control::Control;
pub use palette::Palette;
pub use ppu_reg::PPURegisters;
pub use stat::Stat;

trait Register: Copy + Into<u8> + From<u8> {
    const WRITE_BITS: u8 = 0b1111_1111;

    fn read(&self) -> u8 {
        (*self).into()
    }

    fn write(&mut self, value: u8) {
        *self = (value & Self::WRITE_BITS | (*self).into() & !Self::WRITE_BITS).into()
    }
}
