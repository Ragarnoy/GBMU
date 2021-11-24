use super::error::Error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(usize)]
pub enum RomSize {
    KByte32,
    KByte64,
    KByte128,
    KByte256,
    KByte512,
    MByte1,
    MByte2,
    MByte4,
    MByte8,
}

#[cfg(test)]
impl Default for RomSize {
    fn default() -> Self {
        Self::KByte32
    }
}

impl RomSize {
    const REF_SIZE: usize = 32_768;
    const REF_BANK: usize = 2;

    pub fn get_rom_size(&self) -> usize {
        RomSize::REF_SIZE << *self as usize
    }

    pub fn get_bank_amounts(&self) -> usize {
        RomSize::REF_BANK << *self as usize
    }
}

#[test]
fn test_rom_size() {
    assert_eq!(RomSize::KByte32.get_rom_size(), 32_768);
    assert_eq!(RomSize::KByte64.get_rom_size(), 65_536);
    assert_eq!(RomSize::KByte128.get_rom_size(), 131_072);
    assert_eq!(RomSize::KByte256.get_rom_size(), 262_144);
    assert_eq!(RomSize::KByte512.get_rom_size(), 524_288);
    assert_eq!(RomSize::MByte1.get_rom_size(), 1_048_576);
    assert_eq!(RomSize::MByte2.get_rom_size(), 2_097_152);
    assert_eq!(RomSize::MByte4.get_rom_size(), 4_194_304);
    assert_eq!(RomSize::MByte8.get_rom_size(), 8_388_608);
}

#[test]
fn test_rom_bank_amounts() {
    assert_eq!(RomSize::KByte32.get_bank_amounts(), 2);
    assert_eq!(RomSize::KByte64.get_bank_amounts(), 4);
    assert_eq!(RomSize::KByte128.get_bank_amounts(), 8);
    assert_eq!(RomSize::KByte256.get_bank_amounts(), 16);
    assert_eq!(RomSize::KByte512.get_bank_amounts(), 32);
    assert_eq!(RomSize::MByte1.get_bank_amounts(), 64);
    assert_eq!(RomSize::MByte2.get_bank_amounts(), 128);
    assert_eq!(RomSize::MByte4.get_bank_amounts(), 256);
    assert_eq!(RomSize::MByte8.get_bank_amounts(), 512);
}

impl TryFrom<u8> for RomSize {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(RomSize::KByte32),
            0x01 => Ok(RomSize::KByte64),
            0x02 => Ok(RomSize::KByte128),
            0x03 => Ok(RomSize::KByte256),
            0x04 => Ok(RomSize::KByte512),
            0x05 => Ok(RomSize::MByte1),
            0x06 => Ok(RomSize::MByte2),
            0x07 => Ok(RomSize::MByte4),
            0x08 => Ok(RomSize::MByte8),
            _ => Err(Error::InvalidRomSize(v)),
        }
    }
}

#[test]
fn test_convert_rom_size() {
    assert_eq!(RomSize::try_from(0x00), Ok(RomSize::KByte32));
    assert_eq!(RomSize::try_from(0x01), Ok(RomSize::KByte64));
    assert_eq!(RomSize::try_from(0x02), Ok(RomSize::KByte128));
    assert_eq!(RomSize::try_from(0x03), Ok(RomSize::KByte256));
    assert_eq!(RomSize::try_from(0x04), Ok(RomSize::KByte512));
    assert_eq!(RomSize::try_from(0x05), Ok(RomSize::MByte1));
    assert_eq!(RomSize::try_from(0x06), Ok(RomSize::MByte2));
    assert_eq!(RomSize::try_from(0x07), Ok(RomSize::MByte4));
    assert_eq!(RomSize::try_from(0x08), Ok(RomSize::MByte8));
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum RamSize {
    NoRamOrUnused = 0,
    KByte8 = 1,
    KByte32 = 3,
    KByte64,
    KByte128,
}

#[cfg(test)]
impl Default for RamSize {
    fn default() -> Self {
        Self::NoRamOrUnused
    }
}

impl RamSize {
    const REF_RAM: usize = 8_192;
    const REF_BANK: usize = 1;

    pub fn get_ram_size(&self) -> usize {
        if self == &RamSize::NoRamOrUnused {
            0
        } else {
            RamSize::REF_RAM << (*self as usize - 1)
        }
    }
    pub fn get_bank_amounts(&self) -> usize {
        if self == &RamSize::NoRamOrUnused {
            0
        } else {
            RamSize::REF_BANK << (*self as usize - 1)
        }
    }
}

#[test]
fn test_ram_size() {
    assert_eq!(RamSize::NoRamOrUnused.get_ram_size(), 0);
    assert_eq!(RamSize::KByte8.get_ram_size(), 8_192);
    assert_eq!(RamSize::KByte32.get_ram_size(), 32_768);
    assert_eq!(RamSize::KByte64.get_ram_size(), 65_536);
    assert_eq!(RamSize::KByte128.get_ram_size(), 131_072);
}

#[test]
fn test_ram_bank_amounts() {
    assert_eq!(RamSize::NoRamOrUnused.get_bank_amounts(), 0);
    assert_eq!(RamSize::KByte8.get_bank_amounts(), 1);
    assert_eq!(RamSize::KByte32.get_bank_amounts(), 4);
    assert_eq!(RamSize::KByte64.get_bank_amounts(), 8);
    assert_eq!(RamSize::KByte128.get_bank_amounts(), 16);
}

impl TryFrom<u8> for RamSize {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 | 0x01 => Ok(RamSize::NoRamOrUnused),
            0x02 => Ok(RamSize::KByte8),
            0x03 => Ok(RamSize::KByte32),
            0x04 => Ok(RamSize::KByte128),
            0x05 => Ok(RamSize::KByte64),
            _ => Err(Error::InvalidRamSize(v)),
        }
    }
}

#[test]
fn test_convert_ram_szie() {
    assert_eq!(RamSize::try_from(0x00), Ok(RamSize::NoRamOrUnused));
    assert_eq!(RamSize::try_from(0x01), Ok(RamSize::NoRamOrUnused));
    assert_eq!(RamSize::try_from(0x02), Ok(RamSize::KByte8));
    assert_eq!(RamSize::try_from(0x03), Ok(RamSize::KByte32));
    assert_eq!(RamSize::try_from(0x04), Ok(RamSize::KByte128));
    assert_eq!(RamSize::try_from(0x05), Ok(RamSize::KByte64));
}
