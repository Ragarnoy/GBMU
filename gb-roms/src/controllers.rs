pub mod bios;
pub mod bios_wrapper;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;
pub mod rom_only;

use crate::Header;
pub use bios::Bios;
pub use bios_wrapper::BiosWrapper;
use gb_bus::{Address, Area, Error, FileOperation};
pub use mbc1::MBC1;
pub use mbc2::MBC2;
pub use mbc3::MBC3;
pub use mbc5::MBC5;
pub use rom_only::RomOnlyController;
use serde::{Deserialize, Serialize};
use std::convert::From;

/// Size of the ROM Area
pub const ROM_AREA_SIZE: usize = 0x8000;

/// Maximum size of a rom bank
pub const ROM_BANK_SIZE: usize = 0x4000;

/// Maximum size of a ram bank
pub const RAM_BANK_SIZE: usize = 0x2000;

pub trait Controller {
    /// Save the current controller into a Serializer file
    fn save(&self) -> MbcStates;
}

pub enum MbcController {
    RomOnly(RomOnlyController),
    Mbc1(MBC1),
    Mbc2(MBC2),
    Mbc3(MBC3),
    Mbc5(MBC5),
}

impl MbcController {
    pub fn name(&self) -> &'static str {
        match self {
            MbcController::RomOnly(_) => "romonly",
            MbcController::Mbc1(_) => "Mbc1",
            MbcController::Mbc2(_) => "Mbc2",
            MbcController::Mbc3(_) => "Mbc3",
            MbcController::Mbc5(_) => "Mbc5",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MbcStates {
    Mbc1(mbc1::MbcState),
    Mbc2(mbc2::MbcState),
    Mbc3(mbc3::MbcState),
    Mbc5(mbc5::MbcState),
}

impl MbcStates {
    pub fn name(&self) -> &'static str {
        match self {
            MbcStates::Mbc1(_) => "Mbc1",
            MbcStates::Mbc2(_) => "Mbc2",
            MbcStates::Mbc3(_) => "Mbc3",
            MbcStates::Mbc5(_) => "Mbc5",
        }
    }
}

impl From<RomOnlyController> for MbcController {
    fn from(rom_only: RomOnlyController) -> Self {
        Self::RomOnly(rom_only)
    }
}

impl From<MBC1> for MbcController {
    fn from(mbc1: MBC1) -> Self {
        Self::Mbc1(mbc1)
    }
}

impl From<MBC2> for MbcController {
    fn from(mbc2: MBC2) -> Self {
        Self::Mbc2(mbc2)
    }
}

impl From<MBC3> for MbcController {
    fn from(mbc3: MBC3) -> Self {
        Self::Mbc3(mbc3)
    }
}

impl From<MBC5> for MbcController {
    fn from(mbc5: MBC5) -> Self {
        Self::Mbc5(mbc5)
    }
}

impl MbcController {
    pub fn save<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mbc_state = match self {
            Self::RomOnly(_rom) => panic!("ROM has no data to save"),
            Self::Mbc1(mbc1) => mbc1.save(),
            Self::Mbc2(mbc2) => mbc2.save(),
            Self::Mbc3(mbc3) => mbc3.save(),
            Self::Mbc5(mbc5) => mbc5.save(),
        };
        mbc_state.serialize(serializer)
    }

    pub fn load<'de, D>(&mut self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let mbc_state = MbcStates::deserialize(deserializer)?;
        match (self, mbc_state) {
            (Self::Mbc1(mbc1), MbcStates::Mbc1(state)) => {
                mbc1.with_state(state).map_err(Error::custom)?;
            }
            (Self::RomOnly(_rom), _) => log::warn!("trying to load saved state for romonly"),
            (ctl @ _, state @ _) => {
                log::warn!(
                    "miss match rom type with game save type ({} is incompatible with {})",
                    ctl.name(),
                    state.name(),
                )
            }
        }
        Ok(())
    }
}

impl FileOperation<Area> for MbcController {
    fn read(&self, address: Box<dyn Address<Area>>) -> Result<u8, Error> {
        match self {
            Self::RomOnly(rom) => rom.read(address),
            Self::Mbc1(mbc1) => mbc1.read(address),
            Self::Mbc2(mbc2) => mbc2.read(address),
            Self::Mbc3(mbc3) => mbc3.read(address),
            Self::Mbc5(mbc5) => mbc5.read(address),
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn Address<Area>>) -> Result<(), Error> {
        match self {
            Self::RomOnly(rom) => rom.write(v, address),
            Self::Mbc1(mbc1) => mbc1.write(v, address),
            Self::Mbc2(mbc2) => mbc2.write(v, address),
            Self::Mbc3(mbc3) => mbc3.write(v, address),
            Self::Mbc5(mbc5) => mbc5.write(v, address),
        }
    }
}

pub fn generate_rom_controller(
    reader: impl std::io::Read,
    header: Header,
) -> Result<MbcController, std::io::Error> {
    use crate::header::cartridge_type::CartridgeType::{
        Mbc1, Mbc1Ram, Mbc1RamBattery, Mbc2, Mbc2Battery, Mbc3, Mbc3Ram2, Mbc3RamBattery2,
        Mbc3TimerBattery, Mbc3TimerRamBattery2, Mbc5, Mbc5Ram, Mbc5RamBattery, Mbc5Rumble,
        Mbc5RumbleRam, Mbc5RumbleRamBattery, RomOnly,
    };

    Ok(match header.cartridge_type {
        RomOnly => RomOnlyController::from_file(reader)?.into(),
        Mbc1 | Mbc1Ram | Mbc1RamBattery => MBC1::from_file(reader, header)?.into(),
        Mbc2 | Mbc2Battery => MBC2::from_file(reader, header)?.into(),
        Mbc3 | Mbc3Ram2 | Mbc3RamBattery2 | Mbc3TimerBattery | Mbc3TimerRamBattery2 => {
            MBC3::from_reader(reader, header)?.into()
        }
        Mbc5 | Mbc5Ram | Mbc5RamBattery | Mbc5Rumble | Mbc5RumbleRam | Mbc5RumbleRamBattery => {
            MBC5::from_file(reader, header)?.into()
        }
        _ => panic!("unhandle rom type"),
    })
}
