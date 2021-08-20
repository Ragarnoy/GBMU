/// Contains operations to collect objects from memory.
pub struct Oam {
    data: [u8; Oam::SIZE as usize],
}

impl Oam {
    pub const SIZE: usize = 0xA0;

    pub fn new() -> Self {
        Oam {
            data: [0x00; Self::SIZE as usize],
        }
    }
}

impl Default for Oam {
    fn default() -> Oam {
        Oam::new()
    }
}
