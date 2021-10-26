use std::convert::TryFrom;

use super::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam1,
    RomRamBattery1,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery2,
    Mbc3,
    Mbc3Ram2,
    Mbc3RamBattery2,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl TryFrom<u8> for CartridgeType {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(CartridgeType::RomOnly),
            0x01 => Ok(CartridgeType::Mbc1),
            0x02 => Ok(CartridgeType::Mbc1Ram),
            0x03 => Ok(CartridgeType::Mbc1RamBattery),
            0x05 => Ok(CartridgeType::Mbc2),
            0x06 => Ok(CartridgeType::Mbc2Battery),
            0x08 => Ok(CartridgeType::RomRam1),
            0x09 => Ok(CartridgeType::RomRamBattery1),
            0x0B => Ok(CartridgeType::Mmm01),
            0x0C => Ok(CartridgeType::Mmm01Ram),
            0x0D => Ok(CartridgeType::Mmm01RamBattery),
            0x0F => Ok(CartridgeType::Mbc3TimerBattery),
            0x10 => Ok(CartridgeType::Mbc3TimerRamBattery2),
            0x11 => Ok(CartridgeType::Mbc3),
            0x12 => Ok(CartridgeType::Mbc3Ram2),
            0x13 => Ok(CartridgeType::Mbc3RamBattery2),
            0x19 => Ok(CartridgeType::Mbc5),
            0x1A => Ok(CartridgeType::Mbc5Ram),
            0x1B => Ok(CartridgeType::Mbc5RamBattery),
            0x1C => Ok(CartridgeType::Mbc5Rumble),
            0x1D => Ok(CartridgeType::Mbc5RumbleRam),
            0x1E => Ok(CartridgeType::Mbc5RumbleRamBattery),
            0x20 => Ok(CartridgeType::Mbc6),
            0x22 => Ok(CartridgeType::Mbc7SensorRumbleRamBattery),
            0xFC => Ok(CartridgeType::PocketCamera),
            0xFD => Ok(CartridgeType::BandaiTama5),
            0xFE => Ok(CartridgeType::HuC3),
            0xFF => Ok(CartridgeType::HuC1RamBattery),
            _ => Err(Error::InvalidCartridgeType(v)),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum AutoSave {
    Ram,
    RamTimer,
}

impl CartridgeType {
    /// return the type of auto save the cartridge type require
    /// ```
    /// # use gb_roms::header::{CartridgeType, AutoSave};
    ///
    /// assert_eq!(CartridgeType::Mbc1RamBattery.auto_save_type(), Some(AutoSave::Ram));
    /// assert_eq!(CartridgeType::Mbc1.auto_save_type(), None);
    /// assert_eq!(CartridgeType::Mbc3TimerBattery.auto_save_type(), Some(AutoSave::RamTimer));
    /// ```
    pub fn auto_save_type(&self) -> Option<AutoSave> {
        use CartridgeType::*;

        match self {
            Mbc1RamBattery
            | RomRamBattery1
            | Mmm01RamBattery
            | Mbc3RamBattery2
            | Mbc5RamBattery
            | Mbc5RumbleRamBattery
            | Mbc7SensorRumbleRamBattery
            | HuC1RamBattery => Some(AutoSave::Ram),
            Mbc3TimerBattery | Mbc3TimerRamBattery2 => Some(AutoSave::RamTimer),
            _ => None,
        }
    }
}

#[test]
fn test_convert_cartridge_type() {
    assert_eq!(CartridgeType::try_from(0x00), Ok(CartridgeType::RomOnly));
    assert_eq!(CartridgeType::try_from(0x01), Ok(CartridgeType::Mbc1));
    assert_eq!(CartridgeType::try_from(0x02), Ok(CartridgeType::Mbc1Ram));
    assert_eq!(
        CartridgeType::try_from(0x03),
        Ok(CartridgeType::Mbc1RamBattery)
    );
    assert_eq!(CartridgeType::try_from(0x05), Ok(CartridgeType::Mbc2));
    assert_eq!(
        CartridgeType::try_from(0x06),
        Ok(CartridgeType::Mbc2Battery)
    );
    assert_eq!(CartridgeType::try_from(0x08), Ok(CartridgeType::RomRam1));
    assert_eq!(
        CartridgeType::try_from(0x09),
        Ok(CartridgeType::RomRamBattery1)
    );
    assert_eq!(CartridgeType::try_from(0x0B), Ok(CartridgeType::Mmm01));
    assert_eq!(CartridgeType::try_from(0x0C), Ok(CartridgeType::Mmm01Ram));
    assert_eq!(
        CartridgeType::try_from(0x0D),
        Ok(CartridgeType::Mmm01RamBattery)
    );
    assert_eq!(
        CartridgeType::try_from(0x0F),
        Ok(CartridgeType::Mbc3TimerBattery)
    );
    assert_eq!(
        CartridgeType::try_from(0x10),
        Ok(CartridgeType::Mbc3TimerRamBattery2)
    );
    assert_eq!(CartridgeType::try_from(0x11), Ok(CartridgeType::Mbc3));
    assert_eq!(CartridgeType::try_from(0x12), Ok(CartridgeType::Mbc3Ram2));
    assert_eq!(
        CartridgeType::try_from(0x13),
        Ok(CartridgeType::Mbc3RamBattery2)
    );
    assert_eq!(CartridgeType::try_from(0x19), Ok(CartridgeType::Mbc5));
    assert_eq!(CartridgeType::try_from(0x1A), Ok(CartridgeType::Mbc5Ram));
    assert_eq!(
        CartridgeType::try_from(0x1B),
        Ok(CartridgeType::Mbc5RamBattery)
    );
    assert_eq!(CartridgeType::try_from(0x1C), Ok(CartridgeType::Mbc5Rumble));
    assert_eq!(
        CartridgeType::try_from(0x1D),
        Ok(CartridgeType::Mbc5RumbleRam)
    );
    assert_eq!(
        CartridgeType::try_from(0x1E),
        Ok(CartridgeType::Mbc5RumbleRamBattery)
    );
    assert_eq!(CartridgeType::try_from(0x20), Ok(CartridgeType::Mbc6));
    assert_eq!(
        CartridgeType::try_from(0x22),
        Ok(CartridgeType::Mbc7SensorRumbleRamBattery)
    );
    assert_eq!(
        CartridgeType::try_from(0xFC),
        Ok(CartridgeType::PocketCamera)
    );
    assert_eq!(
        CartridgeType::try_from(0xFD),
        Ok(CartridgeType::BandaiTama5)
    );
    assert_eq!(CartridgeType::try_from(0xFE), Ok(CartridgeType::HuC3));
    assert_eq!(
        CartridgeType::try_from(0xFF),
        Ok(CartridgeType::HuC1RamBattery)
    );
}
