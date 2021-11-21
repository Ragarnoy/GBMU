use super::Register;

const BG_WIN_ENABLE: u8 = 0b1;
const OBJ_ENABLE: u8 = 0b10;
const OBJ_SIZE: u8 = 0b100;
const BG_TILEMAP_AREA: u8 = 0b1000;
const BG_WIN_TILEDATA_AREA: u8 = 0b1_0000;
const WIN_ENABLE: u8 = 0b10_0000;
const WIN_TILEMAP_AREA: u8 = 0b100_0000;
const PPU_ENABLE: u8 = 0b1000_0000;

#[derive(Default, Clone, Copy, Debug)]
pub struct Control {
    bits: u8,
}

impl Control {
    pub const SIZE: usize = 1;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn bg_win_enable(&self) -> bool {
        self.bits & BG_WIN_ENABLE == BG_WIN_ENABLE
    }

    pub fn set_bg_win_enable(&mut self, flag: bool) {
        if flag {
            self.bits |= BG_WIN_ENABLE
        } else {
            self.bits &= !BG_WIN_ENABLE
        }
    }

    pub fn obj_enable(&self) -> bool {
        self.bits & OBJ_ENABLE == OBJ_ENABLE
    }

    pub fn set_obj_enable(&mut self, flag: bool) {
        if flag {
            self.bits |= OBJ_ENABLE
        } else {
            self.bits &= OBJ_ENABLE
        }
    }

    pub fn obj_size(&self) -> bool {
        self.bits & OBJ_SIZE == OBJ_SIZE
    }

    pub fn set_obj_size(&mut self, flag: bool) {
        if flag {
            self.bits |= OBJ_SIZE
        } else {
            self.bits &= OBJ_SIZE
        }
    }

    pub fn bg_tilemap_area(&self) -> bool {
        self.bits & BG_TILEMAP_AREA == BG_TILEMAP_AREA
    }

    pub fn set_bg_tilemap_area(&mut self, flag: bool) {
        if flag {
            self.bits |= BG_TILEMAP_AREA
        } else {
            self.bits &= BG_TILEMAP_AREA
        }
    }

    pub fn bg_win_tiledata_area(&self) -> bool {
        self.bits & BG_WIN_TILEDATA_AREA == BG_WIN_TILEDATA_AREA
    }

    pub fn set_bg_win_tiledata_area(&mut self, flag: bool) {
        if flag {
            self.bits |= BG_WIN_TILEDATA_AREA
        } else {
            self.bits &= BG_WIN_TILEDATA_AREA
        }
    }

    pub fn win_enable(&self) -> bool {
        self.bits & WIN_ENABLE == WIN_ENABLE
    }

    pub fn set_win_enable(&mut self, flag: bool) {
        if flag {
            self.bits |= WIN_ENABLE
        } else {
            self.bits &= WIN_ENABLE
        }
    }

    pub fn win_tilemap_area(&self) -> bool {
        self.bits & WIN_TILEMAP_AREA == WIN_TILEMAP_AREA
    }

    pub fn set_win_tilemap_area(&mut self, flag: bool) {
        if flag {
            self.bits |= WIN_TILEMAP_AREA
        } else {
            self.bits &= WIN_TILEMAP_AREA
        }
    }

    pub fn ppu_enable(&self) -> bool {
        self.bits & PPU_ENABLE == PPU_ENABLE
    }

    pub fn set_ppu_enable(&mut self, flag: bool) {
        if flag {
            self.bits |= PPU_ENABLE
        } else {
            self.bits &= PPU_ENABLE
        }
    }
}

impl From<u8> for Control {
    fn from(byte: u8) -> Control {
        Control { bits: byte }
    }
}

impl From<Control> for u8 {
    fn from(register: Control) -> u8 {
        register.bits
    }
}

impl Register for Control {}
