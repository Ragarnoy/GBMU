use super::Register;
use modular_bitfield::{bitfield, specifiers::B1};

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
struct ControlBits {
    pub bg_win_enable: B1,
    pub obj_enable: B1,
    pub obj_size: B1,
    pub bg_tilemap_area: B1,
    pub bg_win_tiledata_area: B1,
    pub win_enable: B1,
    pub win_tilemap_area: B1,
    pub ppu_enable: B1,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Control {
    bits: ControlBits,
}

impl Control {
    pub const SIZE: usize = 1;

    pub fn new() -> Self {
        Control {
            bits: ControlBits::new(),
        }
    }

    pub fn bg_win_enable(&self) -> bool {
        self.bits.bg_win_enable() != 0
    }
    pub fn set_bg_win_enable(&mut self, flag: bool) {
        self.bits.set_bg_win_enable(if flag { 1 } else { 0 })
    }

    pub fn obj_enable(&self) -> bool {
        self.bits.obj_enable() != 0
    }
    pub fn set_obj_enable(&mut self, flag: bool) {
        self.bits.set_obj_enable(if flag { 1 } else { 0 })
    }

    pub fn obj_size(&self) -> bool {
        self.bits.obj_size() != 0
    }
    pub fn set_obj_size(&mut self, flag: bool) {
        self.bits.set_obj_size(if flag { 1 } else { 0 })
    }

    pub fn bg_tilemap_area(&self) -> bool {
        self.bits.bg_tilemap_area() != 0
    }
    pub fn set_bg_tilemap_area(&mut self, flag: bool) {
        self.bits.set_bg_tilemap_area(if flag { 1 } else { 0 })
    }

    pub fn bg_win_tiledata_area(&self) -> bool {
        self.bits.bg_win_tiledata_area() != 0
    }
    pub fn set_bg_win_tiledata_area(&mut self, flag: bool) {
        self.bits.set_bg_win_tiledata_area(if flag { 1 } else { 0 })
    }

    pub fn win_enable(&self) -> bool {
        self.bits.win_enable() != 0
    }
    pub fn set_win_enable(&mut self, flag: bool) {
        self.bits.set_win_enable(if flag { 1 } else { 0 })
    }

    pub fn win_tilemap_area(&self) -> bool {
        self.bits.win_tilemap_area() != 0
    }
    pub fn set_win_tilemap_area(&mut self, flag: bool) {
        self.bits.set_win_tilemap_area(if flag { 1 } else { 0 })
    }

    pub fn ppu_enable(&self) -> bool {
        self.bits.ppu_enable() != 0
    }
    pub fn set_ppu_enable(&mut self, flag: bool) {
        self.bits.set_ppu_enable(if flag { 1 } else { 0 })
    }
}

impl From<u8> for ControlBits {
    fn from(byte: u8) -> ControlBits {
        ControlBits::from_bytes([byte])
    }
}

impl From<u8> for Control {
    fn from(byte: u8) -> Control {
        Control { bits: byte.into() }
    }
}

impl From<ControlBits> for u8 {
    fn from(bits: ControlBits) -> u8 {
        bits.into_bytes()[0]
    }
}

impl From<Control> for u8 {
    fn from(register: Control) -> u8 {
        register.bits.into()
    }
}

impl Register for Control {}
