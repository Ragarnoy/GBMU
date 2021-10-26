#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Area {
    Rom,
    Vram,
    ExtRam,
    Ram,
    ERam,
    Oam,
    IoReg,
    HighRam,
    IEReg,
}

impl std::convert::From<Area> for u16 {
    fn from(area: Area) -> Self {
        use crate::constant::{
            ERAM_START, EXT_RAM_START, HRAM_START, IE_REG_START, IO_REG_START, OAM_START,
            RAM_START, ROM_START, VRAM_START,
        };

        match area {
            Area::Rom => ROM_START,
            Area::Vram => VRAM_START,
            Area::ExtRam => EXT_RAM_START,
            Area::Ram => RAM_START,
            Area::ERam => ERAM_START,
            Area::Oam => OAM_START,
            Area::IoReg => IO_REG_START,
            Area::HighRam => HRAM_START,
            Area::IEReg => IE_REG_START,
        }
    }
}
