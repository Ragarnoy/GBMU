mod control;
mod palettes_mono;
mod scrolling;
mod stat;
mod window_pos;

pub use control::Control;
pub use palettes_mono::{MonoPaletteRef, PalettesMono};
pub use scrolling::Scrolling;
pub use stat::Stat;
pub use window_pos::WindowPos;

use super::{Register, RegisterArray};
use gb_bus::{Address, Error, IORegArea};
use std::cell::Cell;
use std::convert::TryInto;
use std::rc::Rc;

/// Regroup the registers of the Lcd IOregister area.
#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Debug)]
pub struct LcdReg {
    pub control: Control,
    pub stat: Stat,
    pub scrolling: Scrolling,
    pub pal_mono: PalettesMono,
    pub window_pos: WindowPos,
    pub vbk: Rc<Cell<u8>>,
}

impl Default for LcdReg {
    fn default() -> LcdReg {
        LcdReg {
            control: Control::default(),
            stat: Stat::default(),
            scrolling: Scrolling::default(),
            pal_mono: PalettesMono::default(),
            window_pos: WindowPos::default(),
            vbk: Rc::new(Cell::new(Self::VBK_UNUSED_BITS)),
        }
    }
}

impl LcdReg {
    pub const VBK_UNUSED_BITS: u8 = 0b1111_1110;
    const VBK_SIZE: usize = 1;

    pub const SIZE: usize = Control::SIZE
        + Stat::SIZE
        + Scrolling::SIZE
        + PalettesMono::SIZE
        + WindowPos::SIZE
        + Self::VBK_SIZE;

    pub fn new() -> Self {
        LcdReg::default()
    }

    pub fn read<A>(&self, addr: A) -> Result<u8, Error>
    where
        u16: From<A>,
        A: Address<IORegArea>,
    {
        #[cfg(feature = "cgb")]
        use gb_bus::io_reg_area::IORegArea::Vbk;
        use gb_bus::io_reg_area::IORegArea::{
            Bgp, LcdControl, LcdStat, Ly, Lyc, Obp0, Obp1, Scx, Scy, Wx, Wy,
        };

        match addr.area_type() {
            LcdControl => Ok(self.control.bits),
            LcdStat => Ok(self.stat.read()),

            Scy => Ok(self.scrolling.scy),
            Scx => Ok(self.scrolling.scx),
            Ly => Ok(self.scrolling.ly),
            Lyc => Ok(self.scrolling.lyc),

            Bgp => Ok(self.pal_mono.read(PalettesMono::BACKGROUND)),
            Obp0 => Ok(self.pal_mono.read(PalettesMono::OBJ_O)),
            Obp1 => Ok(self.pal_mono.read(PalettesMono::OBJ_1)),

            Wy => Ok(self.window_pos.wy),
            Wx => Ok(self.window_pos.wx),

            #[cfg(feature = "cgb")]
            Vbk => Ok(self.vbk.get()),

            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    pub fn write<A>(&mut self, addr: A, v: u8) -> Result<(), Error>
    where
        u16: From<A>,
        A: Address<IORegArea>,
    {
        use gb_bus::io_reg_area::IORegArea::{
            Bgp, LcdControl, LcdStat, Ly, Lyc, Obp0, Obp1, Scx, Scy, Wx, Wy,
        };

        match addr.area_type() {
            LcdControl => self.control.write(v),
            LcdStat => self.stat.write(v),

            Scy => self.scrolling.scy = v,
            Scx => self.scrolling.scx = v,
            Ly => (),
            Lyc => self.scrolling.lyc = v,

            Bgp => self.pal_mono.write(PalettesMono::BACKGROUND, v),
            Obp0 => self.pal_mono.write(PalettesMono::OBJ_O, v),
            Obp1 => self.pal_mono.write(PalettesMono::OBJ_1, v),

            Wy => self.window_pos.wy = v,
            Wx => self.window_pos.wx = v,

            #[cfg(feature = "cgb")]
            Vbk => self.vbk.set(v | Self::VBK_UNUSED_BITS),

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
        let vbk = Rc::new(Cell::new(bytes[11] | Self::VBK_UNUSED_BITS));
        LcdReg {
            control: bytes[0].into(),
            stat: bytes[1].into(),
            scrolling: scroll.into(),
            pal_mono: pal.into(),
            window_pos: window.into(),
            vbk,
        }
    }
}

impl From<LcdReg> for [u8; LcdReg::SIZE] {
    fn from(register: LcdReg) -> [u8; LcdReg::SIZE] {
        let scrolling: [u8; 4] = register.scrolling.into();
        let pal_mono: [u8; 3] = register.pal_mono.into();
        let window_pos: [u8; 2] = register.window_pos.into();
        let vbk = register.vbk.get();
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
            vbk,
        ]
    }
}
