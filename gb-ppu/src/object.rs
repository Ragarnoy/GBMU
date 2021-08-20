use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3},
};

#[bitfield]
struct Attributes {
    pub bg_win_priority: B1,
    pub y_flip: B1,
    pub x_flip: B1,
    pub palette_nb: B1,
    pub tile_bank: B1,
    pub palette_cgb_nb: B3,
}

pub struct Object {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attributes: Attributes,
}

impl From<[u8; 4]> for Object {
    fn from(bytes: [u8; 4]) -> Object {
        Object {
            y_pos: bytes[0],
            x_pos: bytes[1],
            tile_index: bytes[2],
            attributes: Attributes::from_bytes([bytes[3]]),
        }
    }
}
