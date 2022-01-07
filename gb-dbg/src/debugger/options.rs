use gb_bus::constant::{
    EXT_RAM_START, EXT_RAM_STOP, HRAM_START, HRAM_STOP, RAM_START, RAM_STOP, ROM_START, ROM_STOP,
    VRAM_START, VRAM_STOP,
};
use std::ops::Range;

#[derive(Clone)]
pub struct DebuggerOptions {
    pub address_ranges: Vec<(&'static str, Range<u16>)>,
    pub breakpoints: Vec<String>,
}

impl Default for DebuggerOptions {
    fn default() -> Self {
        Self {
            address_ranges: vec![
                ("VRAM", VRAM_START..VRAM_STOP),
                ("EXT_RAM", EXT_RAM_START..EXT_RAM_STOP),
                ("ROM", ROM_START..ROM_STOP),
                ("RAM", RAM_START..RAM_STOP),
                ("HRAM", HRAM_START..HRAM_STOP),
            ],
            breakpoints: Default::default(),
        }
    }
}
