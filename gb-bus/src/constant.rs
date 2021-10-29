pub const ROM_START: u16 = 0x0000;
pub const ROM_STOP: u16 = 0x7fff;
pub const VRAM_START: u16 = 0x8000;
pub const VRAM_STOP: u16 = 0x9fff;
pub const EXT_RAM_START: u16 = 0xa000;
pub const EXT_RAM_STOP: u16 = 0xbfff;
pub const RAM_START: u16 = 0xc000;
pub const RAM_STOP: u16 = 0xdfff;
pub const ERAM_START: u16 = 0xe000;
pub const ERAM_STOP: u16 = 0xfdff;
pub const OAM_START: u16 = 0xfe00;
pub const OAM_STOP: u16 = 0xfe9f;
pub const IO_REG_START: u16 = 0xff00;
pub const IO_REG_STOP: u16 = 0xff7f;
pub const HRAM_START: u16 = 0xff80;
pub const HRAM_STOP: u16 = 0xfffe;
pub const IE_REG_START: u16 = 0xffff;
pub const FORBIDDEN_START: u16 = 0xfea0;

pub const UNDEFINED_VALUE: u8 = 0xff;
