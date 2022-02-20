use crate::memory::BankSelector;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone)]
pub struct BGTileAttributes {
    bits: u8,
}

impl BGTileAttributes {
    const BG_TO_OAM_PRIORITY: u8 = 0b1000_0000;
    const V_FLIP: u8 = 0b100_0000;
    const H_FLIP: u8 = 0b10_0000;

    const VRAM_BANK_NB: u8 = 0b1000;
    // const PALETTE_NB: u8 = 0b111;

    pub fn bg_priority(&self) -> bool {
        self.bits & Self::BG_TO_OAM_PRIORITY != 0
    }

    pub fn v_flip(&self) -> bool {
        self.bits & Self::V_FLIP != 0
    }

    pub fn h_flip(&self) -> bool {
        self.bits & Self::H_FLIP != 0
    }

    pub fn bank_nb(&self) -> BankSelector {
        if self.bits & Self::VRAM_BANK_NB != 0 {
            BankSelector::Bank1
        } else {
            BankSelector::Bank0
        }
    }
}

impl From<usize> for BGTileAttributes {
    fn from(byte: usize) -> BGTileAttributes {
        BGTileAttributes { bits: byte as u8 }
    }
}
