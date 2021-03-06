use crate::error::{PPUError, PPUResult};

/// Represent the colors of pixels.
///
/// Can easily be converted into a RGB value (`[u8; 3]`) for the renderer.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
    Rgb555(u8, u8),
}

impl Color {
    const RED_MASK: u16 = 0b1_1111;
    const GREEN_MASK: u16 = 0b11_1110_0000;
    const BLUE_MASK: u16 = 0b111_1100_0000_0000;

    /// Separate each values of the color and scale them from rgb555 to rgb888
    fn rgb_scale(byte0: u8, byte1: u8) -> [u8; 3] {
        let color_bytes = ((byte1 as u16) << 8) | byte0 as u16;
        [
            ((color_bytes & Self::RED_MASK) as f32 * 255.0 / 31.0) as u8,
            (((color_bytes & Self::GREEN_MASK) >> 5) as f32 * 255.0 / 31.0) as u8,
            (((color_bytes & Self::BLUE_MASK) >> 10) as f32 * 255.0 / 31.0) as u8,
        ]
    }
}

impl TryFrom<u8> for Color {
    type Error = PPUError;
    fn try_from(value: u8) -> PPUResult<Color> {
        match value {
            3 => Ok(Color::Black),
            2 => Ok(Color::DarkGray),
            1 => Ok(Color::LightGray),
            0 => Ok(Color::White),
            _ => Err(PPUError::OutOfBound {
                value: value as usize,
                min_bound: 0,
                max_bound: 3,
            }),
        }
    }
}

impl From<[u8; 2]> for Color {
    fn from(value: [u8; 2]) -> Color {
        Color::Rgb555(value[0], value[1])
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        match color {
            Color::White => [255; 3],
            Color::LightGray => [170; 3],
            Color::DarkGray => [85; 3],
            Color::Black => [0; 3],
            Color::Rgb555(byte0, byte1) => Color::rgb_scale(byte0, byte1),
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::White
    }
}
