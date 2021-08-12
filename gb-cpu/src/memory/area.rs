pub mod rom;
pub mod wram;

use super::consts;
pub use wram::Wram;

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Rom(u16),
    Vram(u16),
    ExtRam(u16),
    Wram(u16),
    EchoRam(u16),
    Oam(u16),
    IOReg(u16),
    HighRam(u16),
    IEReg,
}

impl Area {
    pub fn relative(&self) -> usize {
        let result = match self {
            Area::Rom(address) => *address,
            Area::Vram(address) => *address - consts::ROM_MIN,
            Area::ExtRam(address) => *address - consts::EXT_RAM_MIN,
            Area::Wram(address) => *address - consts::WRAM_MIN,
            Area::EchoRam(address) => *address - consts::ECHO_RAM_MIN,
            Area::Oam(address) => *address - consts::OAM_MIN,
            Area::IOReg(address) => *address - consts::IOREG_MIN,
            Area::HighRam(address) => *address - consts::HIGH_MIN,
            Area::IEReg => consts::INTERUPT_ENABLE,
        };
        result as usize
    }
}
