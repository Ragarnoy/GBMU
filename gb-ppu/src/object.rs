use crate::error::{Error, PPUResult};
use crate::memory::Vram;
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
    pub fn new() -> Self {
        Object {
            y_pos: 0,
            x_pos: 0,
            tile_index: 0,
            attributes: Attributes::new(),
        }
    }

    pub const SIZE: usize = 4;

    pub fn y_pos(&self) -> u8 {
        self.y_pos
    }

    pub fn x_pos(&self) -> u8 {
        self.x_pos
    }

    fn y_flip(&self) -> bool {
        self.attributes.y_flip() != 0
    }

    fn x_flip(&self) -> bool {
        self.attributes.x_flip() != 0
    }

    /// Read the row of 8 pixels values for this object.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8 or 16 depending of the size_16 flag.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    ///  - **size_16**: the bit 2 flag from Control indicating if the object is 8(*false*) or 16(*true*) pixels high.
    pub fn get_pixels_row(
        &self,
        line: usize,
        vram: &Vram,
        size_16: bool,
    ) -> PPUResult<[u8; 8], usize> {
        if !size_16 {
            self.get_pixels_row_8x8(line, vram)
        } else {
            self.get_pixels_row_8x16(line, vram)
        }
    }

    /// Read the row of 8 pixels values for this object in 8x8 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    fn get_pixels_row_8x8(&self, line: usize, vram: &Vram) -> PPUResult<[u8; 8], usize> {
        let mut row = [0; 8];
        if line > 8 {
            return Err(Error::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 8,
            });
        }
        let y = if self.y_flip() { 7 - line } else { line };
        let tile_row = vram.read_tile_line(self.tile_index as usize, y).unwrap();
        for (i, pixel) in row.iter_mut().enumerate() {
            let x = if self.x_flip() { 7 - i } else { i };
            let value = tile_row[x];
            *pixel = value;
        }
        Ok(row)
    }

    /// Read the row of 8 pixels values for this object in 8x16 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 16.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    fn get_pixels_row_8x16(&self, mut line: usize, vram: &Vram) -> PPUResult<[u8; 8], usize> {
        let mut row = [0; 8];
        if line > 15 {
            return Err(Error::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 15,
            });
        }
        let index = if line > 7 && !self.y_flip() || line < 8 && self.y_flip() {
            self.tile_index as usize + 1
        } else {
            self.tile_index as usize
        };
        if line > 7 {
            line -= 8
        }
        let y = if self.y_flip() { 7 - line } else { line };
        let tile_line = vram.read_tile_line(index, y).unwrap();
        for (i, pixel) in row.iter_mut().enumerate() {
            let x = if self.x_flip() { 7 - i } else { i };
            let value = tile_line[x];
            *pixel = value;
        }
        Ok(row)
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
