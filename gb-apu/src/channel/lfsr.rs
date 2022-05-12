const LFSR_ALL_BIT_SET: u16 = 0x7FFF;

#[derive(Debug, PartialEq)]
pub enum WidthMode {
    Width7Bits,
    Width14Bits,
}

// Linear Feedback Shift Register
#[derive(Debug)]
pub struct Lfsr {
    value: u16,
    pub width_mode: WidthMode,
}

impl Default for Lfsr {
    fn default() -> Self {
        Self {
            value: 0,
            width_mode: WidthMode::Width14Bits,
        }
    }
}

impl Lfsr {
    pub fn step(&mut self) {
        let xor_res = (self.value & 0b01) ^ ((self.value & 0b10) >> 1);
        self.value = (self.value >> 1) | (xor_res << 14);

        if self.width_mode == WidthMode::Width7Bits {
            self.value &= !(1 << 6);
            self.value |= xor_res << 6;
        }
    }

    pub fn get_amplitude(&self) -> u8 {
        (!self.value & 1) as u8
    }

    pub fn reload(&mut self) {
        self.value = LFSR_ALL_BIT_SET;
    }
}
