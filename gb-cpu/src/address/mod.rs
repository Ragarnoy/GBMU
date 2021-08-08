pub mod area;
pub mod consts;

use area::Area;
use consts::*;

pub fn relative(area: Area, address: u16) -> usize {
    let result = match area {
        Area::Rom => address,
        Area::Vram => address - ROM_MIN,
        Area::ExtRam => address - EXTERN_MIN,
        Area::Wram => address - WRAM_MIN,
        Area::EchoRam => address - ECHO_MIN,
        Area::Oam => address - OAM_MIN,
        Area::IOReg => address - IOREG_MIN,
        Area::HighRam => address - HIGH_MIN,
        Area::IEReg => INTERUPT_ENABLE,
    };
    result as usize
}
