mod control;
mod palettes_mono;
mod scrolling;
mod stat;
mod window_pos;

pub use control::Control;
pub use palettes_mono::PalettesMono;
pub use scrolling::Scrolling;
pub use stat::Stat;
pub use window_pos::WindowPos;

use super::{Register, RegisterArray};
use gb_bus::{Addr, Address, Error, IORegArea};
use std::convert::TryInto;

/// Regroup the registers of the Lcd IOregister area.
#[derive(Default, Debug)]
pub struct LcdReg {
    pub control: Control,
    pub stat: Stat,
    pub scrolling: Scrolling,
    pub pal_mono: PalettesMono,
    pub window_pos: WindowPos,
}

impl LcdReg {
    pub const SIZE: usize =
        Control::SIZE + Stat::SIZE + Scrolling::SIZE + PalettesMono::SIZE + WindowPos::SIZE;

    pub fn new() -> Self {
        LcdReg::default()
    }

    pub fn read(&self, addr: Addr<IORegArea>) -> Result<u8, Error> {
        match addr.get_address() {
            0x00 => Ok(self.control.read()),
            0x01 => Ok(self.stat.read()),
            pos @ 0x02..=0x05 => Ok(self.scrolling.read(pos - 0x02)),
            pos @ 0x07..=0x09 => Ok(self.pal_mono.read(pos - 0x07)),
            pos @ 0x0A..=0x0B => Ok(self.window_pos.read(pos - 0x0A)),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    pub fn write(&mut self, addr: Addr<IORegArea>, v: u8) -> Result<(), Error> {
        match addr.get_address() {
            0x00 => self.control.write(v),
            0x01 => self.stat.write(v),
            pos @ 0x02..=0x05 => self.scrolling.write(pos - 0x02, v),
            pos @ 0x07..=0x09 => self.pal_mono.write(pos - 0x07, v),
            pos @ 0x0A..=0x0B => self.window_pos.write(pos - 0x0A, v),
            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}

impl From<[u8; LcdReg::SIZE]> for LcdReg {
    fn from(bytes: [u8; LcdReg::SIZE]) -> LcdReg {
        let scroll: [u8; 4] = bytes[2..=5].try_into().expect("bad bytes for LcdReg");
        let pal: [u8; 3] = bytes[6..=8].try_into().expect("bad bytes for LcdReg");
        let window: [u8; 2] = bytes[9..=10].try_into().expect("bad bytes for LcdReg");
        LcdReg {
            control: bytes[0].into(),
            stat: bytes[1].into(),
            scrolling: scroll.into(),
            pal_mono: pal.into(),
            window_pos: window.into(),
        }
    }
}

impl From<LcdReg> for [u8; LcdReg::SIZE] {
    fn from(register: LcdReg) -> [u8; LcdReg::SIZE] {
        let scrolling: [u8; 4] = register.scrolling.into();
        let pal_mono: [u8; 3] = register.pal_mono.into();
        let window_pos: [u8; 2] = register.window_pos.into();
        [
            register.control.into(),
            register.stat.into(),
            scrolling[0],
            scrolling[1],
            scrolling[2],
            scrolling[3],
            pal_mono[0],
            pal_mono[1],
            pal_mono[2],
            window_pos[0],
            window_pos[1],
        ]
    }
}
