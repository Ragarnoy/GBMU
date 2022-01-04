use crate::error::{PPUError, PPUResult};
use crate::memory::Vram;
use crate::registers::Palette;
use crate::Color;
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3},
};
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

#[bitfield]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Attributes {
    #[skip]
    pub palette_cgb_nb: B3,
    #[skip]
    pub tile_bank: B1,
    pub palette_nb: B1,
    pub x_flip: B1,
    pub y_flip: B1,
    pub bg_win_priority: B1,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sprite {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attributes: Attributes,
}

impl<'r> Sprite {
    pub const VERTICAL_OFFSET: u8 = 16;
    pub const HORIZONTAL_OFFSET: u8 = 8;

    pub fn new() -> Self {
        Sprite {
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

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }

    pub fn get_palette(
        &self,
        palettes: (&'r Rc<Cell<Palette>>, &'r Rc<Cell<Palette>>),
    ) -> &'r Rc<Cell<Palette>> {
        if self.attributes.palette_nb() == 0 {
            palettes.0
        } else {
            palettes.1
        }
    }

    pub fn bg_win_priority(&self) -> bool {
        self.attributes.bg_win_priority() != 0
    }

    /// Read the row of 8 pixels values for this sprite.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8 or 16 depending of the size_16 flag.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    ///  - **size_16**: the bit 2 flag from Control indicating if the sprite is 8(*false*) or 16(*true*) pixels high.
    pub fn get_pixels_row(
        &self,
        line: usize,
        vram: &dyn Deref<Target = Vram>,
        size_16: bool,
        palettes: (&Rc<Cell<Palette>>, &Rc<Cell<Palette>>),
    ) -> PPUResult<[(u8, Color); 8]> {
        let palette = self.get_palette(palettes);
        if !size_16 {
            self.get_pixels_row_8x8(line, vram, &palette.get())
        } else {
            self.get_pixels_row_8x16(line, vram, &palette.get())
        }
    }

    /// Read the row of 8 pixels values for this sprite in 8x8 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    fn get_pixels_row_8x8(
        &self,
        line: usize,
        vram: &Vram,
        palette: &Palette,
    ) -> PPUResult<[(u8, Color); 8]> {
        let mut row = [(0, Color::default()); 8];
        if line > 8 {
            return Err(PPUError::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 8,
            });
        }
        let y = if self.y_flip() { 7 - line } else { line };
        let tile_row = vram.read_tile_line(self.tile_index as usize, y)?;
        for (i, pixel) in row.iter_mut().enumerate() {
            let x = if self.x_flip() { 7 - i } else { i };
            let value = tile_row[x];
            let color = palette.get_color(tile_row[x])?;
            *pixel = (value, color);
        }
        Ok(row)
    }

    /// Read the row of 8 pixels values for this sprite in 8x16 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 16.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    fn get_pixels_row_8x16(
        &self,
        mut line: usize,
        vram: &Vram,
        palette: &Palette,
    ) -> PPUResult<[(u8, Color); 8]> {
        let mut row = [(0, Color::default()); 8];
        if line > 15 {
            return Err(PPUError::OutOfBound {
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
            let color = palette.get_color(tile_line[x])?;
            *pixel = (value, color);
        }
        Ok(row)
    }
}

impl From<[u8; Sprite::SIZE]> for Sprite {
    fn from(bytes: [u8; Sprite::SIZE]) -> Sprite {
        Sprite {
            y_pos: bytes[0],
            x_pos: bytes[1],
            tile_index: bytes[2],
            attributes: Attributes::from_bytes([bytes[3]]),
        }
    }
}

impl From<Sprite> for [u8; Sprite::SIZE] {
    fn from(obj: Sprite) -> [u8; Sprite::SIZE] {
        [
            obj.y_pos,
            obj.x_pos,
            obj.tile_index,
            obj.attributes.into_bytes()[0],
        ]
    }
}
