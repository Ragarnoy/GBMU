use super::consts;

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Rom,
    Vram,
    ExtRam,
    Wram,
    EchoRam,
    Oam,
    IOReg,
    HighRam,
    IEReg,
}

impl Area {
    pub fn relative(&self, address: u16) -> usize {
        let result = match self {
            Area::Rom => address,
            Area::Vram => address - consts::ROM_MIN,
            Area::ExtRam => address - consts::EXT_RAM_MIN,
            Area::Wram => address - consts::WRAM_MIN,
            Area::EchoRam => address - consts::ECHO_RAM_MIN,
            Area::Oam => address - consts::OAM_MIN,
            Area::IOReg => address - consts::IOREG_MIN,
            Area::HighRam => address - consts::HIGH_MIN,
        };
        result as usize
    }
}
