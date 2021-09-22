use std::ops::Range;

pub struct DebuggerOptions {
    pub address_ranges: Vec<(&'static str, Range<usize>)>,
}

impl Default for DebuggerOptions {
    fn default() -> Self {
        Self {
            address_ranges: vec![
                ("ROM", 0x0000..0x7FFF),
                ("VRAM", 0x8000..0x9FFF),
                ("ERAM", 0xA000..0xBFFF),
                ("RAM", 0xC000..0xDFFF),
                ("HRAM", 0xFF80..0xFFFE),
            ],
        }
    }
}
