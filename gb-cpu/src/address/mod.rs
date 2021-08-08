pub mod area;
pub mod consts;

use area::Area;
use consts::*;

pub fn relative(area: Area, address: u16) -> usize {
    let result = match area {
        Area::Rom => address,
        Area::Vram => ROM_MIN - address,
        Area::ExtRam => EXTERN_MIN - address,
        Area::Wram => WRAM_MIN - address,
        Area::EchoRam => ECHO_MIN - address,
        Area::Oam => OAM_MIN - address,
        Area::IOReg => IOREG_MIN - address,
        Area::HighRam => HIGH_MIN - address,
        Area::IEReg => INTERUPT_ENABLE,
    };
    result as usize
}
