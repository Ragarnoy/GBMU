use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3},
};

#[bitfield]
#[derive(Clone, Copy, Debug)]
struct Attributes {
    pub palette_cgb_nb: B3,
    pub tile_bank: B1,
    pub palette_nb: B1,
    pub x_flip: B1,
    pub y_flip: B1,
    pub bg_win_priority: B1,
}

#[derive(Clone, Copy, Debug)]
pub struct Object {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attributes: Attributes,
}

impl Object {
    pub const SIZE: usize = 4;

    pub fn y_pos(&self) -> u8 {
        self.y_pos
    }

    pub fn x_pos(&self) -> u8 {
        self.x_pos
    }

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }

    pub fn y_flip(&self) -> u8 {
        self.attributes.y_flip()
    }

    pub fn x_flip(&self) -> u8 {
        self.attributes.x_flip()
    }
}

impl From<[u8; Object::SIZE]> for Object {
    fn from(bytes: [u8; Object::SIZE]) -> Object {
        Object {
            y_pos: bytes[0],
            x_pos: bytes[1],
            tile_index: bytes[2],
            attributes: Attributes::from_bytes([bytes[3]]),
        }
    }
}
