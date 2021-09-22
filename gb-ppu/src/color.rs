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
}

impl Color {
    /// Translate a color value into a Color enum.
    ///
    /// ### Parameters
    /// - **value**: the color value of a pixel from a palette.
    pub fn from_value(value: u8) -> PPUResult<Color> {
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

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> [u8; 3] {
        match color {
            Color::White => [255; 3],
            Color::LightGray => [170; 3],
            Color::DarkGray => [85; 3],
            Color::Black => [0; 3],
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::White
    }
}
