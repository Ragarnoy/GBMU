#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct PalettesCGB {
    bcps: u8,
    bcpd: u8,
    bc_values: [u8; 64],
    ocps: u8,
    ocpd: u8,
    oc_values: [u8; 64],
}

impl Default for PalettesCGB {
    fn default() -> PalettesCGB {
        PalettesCGB {
            bcps: 0,
            bcpd: 0,
            bc_values: [0; 64],
            ocps: 0,
            ocpd: 0,
            oc_values: [0; 64],
        }
    }
}

impl PalettesCGB {
    pub const SIZE: usize = 4;
}

impl From<[u8; 4]> for PalettesCGB {
    fn from(bytes: [u8; 4]) -> PalettesCGB {
        PalettesCGB {
            bcps: bytes[0],
            bcpd: bytes[1],
            bc_values: [0; 64],
            ocps: bytes[2],
            ocpd: bytes[3],
            oc_values: [0; 64],
        }
    }
}

impl From<PalettesCGB> for [u8; 4] {
    fn from(register: PalettesCGB) -> [u8; 4] {
        [register.bcps, register.bcpd, register.ocps, register.ocpd]
    }
}
