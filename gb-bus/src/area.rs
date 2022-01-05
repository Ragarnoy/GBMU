#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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
    Forbidden,
}

impl std::convert::From<Area> for u16 {
    fn from(area: Area) -> Self {
        use crate::constant::{
            ERAM_START, EXT_RAM_START, FORBIDDEN_START, HRAM_START, IE_REG, IO_REG_START,
            OAM_START, RAM_START, ROM_START, VRAM_START,
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
            Area::IEReg => IE_REG,
            Area::Forbidden => FORBIDDEN_START,
        }
    }
}

impl std::convert::From<u16> for Area {
    fn from(bytes: u16) -> Self {
        use crate::constant::{
            ERAM_START, ERAM_STOP, EXT_RAM_START, EXT_RAM_STOP, HRAM_START, HRAM_STOP, IE_REG,
            IO_REG_START, IO_REG_STOP, OAM_START, OAM_STOP, RAM_START, RAM_STOP, ROM_START,
            ROM_STOP, VRAM_START, VRAM_STOP,
        };

        match bytes {
            ROM_START..=ROM_STOP => Area::Rom,
            VRAM_START..=VRAM_STOP => Area::Vram,
            EXT_RAM_START..=EXT_RAM_STOP => Area::ExtRam,
            RAM_START..=RAM_STOP => Area::Ram,
            ERAM_START..=ERAM_STOP => Area::ERam,
            OAM_START..=OAM_STOP => Area::Oam,
            IO_REG_START..=IO_REG_STOP => Area::IoReg,
            HRAM_START..=HRAM_STOP => Area::HighRam,
            IE_REG => Area::IEReg,
            _ => Area::Forbidden,
        }
    }
}
