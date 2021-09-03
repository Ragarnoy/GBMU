use modular_bitfield::{bitfield, specifiers::B1};

#[bitfield]
pub struct Control {
    pub bg_win_enable: B1,
    pub obj_enable: B1,
    pub obj_size: B1,
    pub bg_tilemap_area: B1,
    pub bg_win_tiledata_area: B1,
    pub win_enable: B1,
    pub win_tilemap_area: B1,
    pub ppu_enable: B1,
}
