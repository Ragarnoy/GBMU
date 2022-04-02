use crate::error::{PPUError, PPUResult};
use crate::memory::{BankSelector, Vram};
use crate::registers::{LcdReg, PaletteRef};
use std::ops::Deref;

const PALETTE_CGB_NB: u8 = 0b111;
const TILE_BANK: u8 = 0b1000;
const TILE_BANK_OFFSET: u8 = 3;

const PALETTE_NB: u8 = 0b1_0000;
const X_FLIP: u8 = 0b10_0000;
const Y_FLIP: u8 = 0b100_0000;
const BG_WIN_PRIORITY: u8 = 0b1000_0000;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Sprite {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attributes: u8,
}

impl<'r> Sprite {
    pub const VERTICAL_OFFSET: u8 = 16;
    pub const HORIZONTAL_OFFSET: u8 = 8;

    pub const TILE_INDEX_MASK: u8 = 0b1111_1110;

    pub fn new() -> Self {
        Sprite {
            y_pos: 0,
            x_pos: 0,
            tile_index: 0,
            attributes: 0,
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
        self.attributes & Y_FLIP != 0
    }

    fn x_flip(&self) -> bool {
        self.attributes & X_FLIP != 0
    }

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }

    fn bank_selector(&self, cgb_enabled: bool) -> Option<BankSelector> {
        if cgb_enabled {
            Some(
                BankSelector::try_from(
                    ((self.attributes & TILE_BANK) >> TILE_BANK_OFFSET) | LcdReg::VBK_UNUSED_BITS,
                )
                .expect("Corrupted sprite data for bank selector"),
            )
        } else {
            None
        }
    }

    fn get_palette_ref(&self, cgb_enabled: bool) -> PaletteRef {
        if cgb_enabled {
            if self.attributes & PALETTE_NB == 0 {
                PaletteRef::MonoSprite0
            } else {
                PaletteRef::MonoSprite1
            }
        } else {
            PaletteRef::CgbSprite(self.attributes & PALETTE_CGB_NB)
        }
    }

    pub fn bg_win_priority(&self) -> bool {
        self.attributes & BG_WIN_PRIORITY != 0
    }

    /// Read the row of 8 pixels values for this sprite.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8 or 16 depending of the size_16 flag.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    ///  - **lcd_reg**: A reference to the lcd registers.
    ///  - **cgb_enabled**: indicate if we are in cgb display mode.
    pub fn get_pixels_row(
        &self,
        line: usize,
        vram: &dyn Deref<Target = Vram>,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        cgb_enabled: bool,
    ) -> PPUResult<([u8; 8], PaletteRef)> {
        let size_16 = lcd_reg.control.obj_size();
        let palette = self.get_palette_ref(cgb_enabled);
        let row = if !size_16 {
            self.get_pixels_row_8x8(line, vram, cgb_enabled)?
        } else {
            self.get_pixels_row_8x16(line, vram, cgb_enabled)?
        };
        Ok((row, palette))
    }

    /// Read the row of 8 pixels values for this sprite in 8x8 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 8.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    ///  - **cgb_enabled**: indicate if we are in cgb display mode.
    fn get_pixels_row_8x8(
        &self,
        line: usize,
        vram: &Vram,
        cgb_enabled: bool,
    ) -> PPUResult<[u8; 8]> {
        let mut row = [0; 8];
        if line > 8 {
            return Err(PPUError::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 8,
            });
        }
        let y = if self.y_flip() { 7 - line } else { line };
        let vram_bank = self.bank_selector(cgb_enabled);
        let tile_row = vram.read_tile_line(self.tile_index as usize, y, vram_bank)?;
        for (i, pixel) in row.iter_mut().enumerate() {
            let x = if self.x_flip() { 7 - i } else { i };
            *pixel = tile_row[x];
        }
        Ok(row)
    }

    /// Read the row of 8 pixels values for this sprite in 8x16 pixels mode.
    ///
    /// ### Parameters
    ///  - **line**: The index of the row of pixel to return. Should be below 16.
    ///  - **vram**: A reference to the vram to read the pixel values from.
    ///  - **cgb_enabled**: indicate if we are in cgb display mode.
    fn get_pixels_row_8x16(
        &self,
        mut line: usize,
        vram: &Vram,
        cgb_enabled: bool,
    ) -> PPUResult<[u8; 8]> {
        let mut row = [0; 8];
        if line > 15 {
            return Err(PPUError::OutOfBound {
                value: line,
                min_bound: 0,
                max_bound: 15,
            });
        }
        let index = if line > 7 && !self.y_flip() || line < 8 && self.y_flip() {
            (self.tile_index & Self::TILE_INDEX_MASK) as usize + 1
        } else {
            (self.tile_index & Self::TILE_INDEX_MASK) as usize
        };
        if line > 7 {
            line -= 8
        }
        let y = if self.y_flip() { 7 - line } else { line };
        let vram_bank = self.bank_selector(cgb_enabled);
        let tile_line = vram.read_tile_line(index, y, vram_bank)?;
        for (i, pixel) in row.iter_mut().enumerate() {
            let x = if self.x_flip() { 7 - i } else { i };
            *pixel = tile_line[x];
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
            attributes: bytes[3],
        }
    }
}

impl From<Sprite> for [u8; Sprite::SIZE] {
    fn from(obj: Sprite) -> [u8; Sprite::SIZE] {
        [obj.y_pos, obj.x_pos, obj.tile_index, obj.attributes]
    }
}
