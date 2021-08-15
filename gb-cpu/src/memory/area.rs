pub mod rom;
pub mod wram;

use super::consts;
pub use wram::Wram;

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Rom,
    _Vram,
    _ExtRam,
    Wram,
    _EchoRam,
    _Oam,
    _IOReg,
    _HighRam,
}

impl Area {
    pub fn relative(self, address: u16) -> usize {
        let result = match self {
            Area::Rom => address,
            Area::_Vram => address - consts::ROM_MIN,
            Area::_ExtRam => address - consts::EXT_RAM_MIN,
            Area::Wram => address - consts::WRAM_MIN,
            Area::_EchoRam => address - consts::ECHO_RAM_MIN,
            Area::_Oam => address - consts::OAM_MIN,
            Area::_IOReg => address - consts::IOREG_MIN,
            Area::_HighRam => address - consts::HIGH_MIN,
        };
        result as usize
    }
}
