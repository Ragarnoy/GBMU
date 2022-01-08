pub mod bios;
pub mod bios_wrapper;
pub mod generic;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;
pub mod rom_only;

use crate::Header;
pub use bios::Bios;
pub use bios_wrapper::BiosWrapper;
pub use generic::{Generic, GenericState};

/// Size of the ROM Area
pub const ROM_AREA_SIZE: usize = 0x8000;

/// Maximum size of a rom bank
pub const ROM_BANK_SIZE: usize = 0x4000;

/// Maximum size of a ram bank
pub const RAM_BANK_SIZE: usize = 0x2000;

pub trait SaveState {
    fn serialize(&self) -> Full;
    fn serialize_partial(&self) -> Partial {
        Partial::None
    }
    fn load(&self, state: Full) -> Result<(), String>;
    fn load_partial(&self, _state: Partial) -> Result<(), String> {
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Full {
    None,
    Mbc1(mbc1::State),
    Mbc2(mbc2::State),
    Mbc3(mbc3::State),
    Mbc5(mbc5::State),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Partial {
    None,
    Mbc2(mbc2::PartialState),
    Mbc3(mbc3::PartialState),
}

pub trait Controller: SaveState {
    /// Return the size of the rom and optionnaly the size of the external ram
    fn sizes(&self) -> (usize, Option<usize>);

    /// When data is beeing written to ROM
    /// MBC catch these writes to modify their internal register
    fn write_rom(&mut self, v: u8, addr: u16);

    /// Indicate that the RAM is readable/writable
    fn ram_enabled(&self) -> bool;

    /// Indicate if the read to the RAM area was overrided
    fn override_read_ram(&self, addr: u16) -> Option<u8>;

    /// Indicate if the write to the RAM area was overrided
    /// If the action is catch, it will no modify the ram
    fn override_write_ram(&mut self, v: u8, addr: u16) -> Option<()>;

    /// Offset the address when performing an operation on the RAM
    /// This is usefull when you have to manage BANKS of RAM
    fn offset_ram_addr(&self, addr: u16) -> usize;

    /// Offset the address when performing an operation on the ROM
    /// This is usefull when you have to manage BANKS of ROM
    fn offset_rom_addr(&self, addr: u16) -> usize;

    /// Create a new RAM area
    fn create_ram(&self) -> Option<Vec<u8>> {
        let (_, ram_size) = self.sizes();

        ram_size.map(|size| vec![0; size])
    }
    /// Create a new ROM area
    fn create_rom(&self) -> Vec<u8> {
        let (rom_size, _) = self.sizes();

        vec![0; rom_size]
    }
}

fn new_controller_from_header(header: Header) -> Box<dyn Controller> {
    use crate::header::cartridge_type::CartridgeType::{
        Mbc1, Mbc1Ram, Mbc1RamBattery, Mbc2, Mbc2Battery, Mbc3, Mbc3Ram2, Mbc3RamBattery2,
        Mbc3TimerBattery, Mbc3TimerRamBattery2, Mbc5, Mbc5Ram, Mbc5RamBattery, Mbc5Rumble,
        Mbc5RumbleRam, Mbc5RumbleRamBattery, RomOnly,
    };
    match header.cartridge_type {
        RomOnly => rom_only::new_controller(header),
        Mbc1 | Mbc1Ram | Mbc1RamBattery => mbc1::new_controller(header),
        Mbc2 | Mbc2Battery => mbc2::new_controller(header),
        Mbc3 | Mbc3Ram2 | Mbc3RamBattery2 | Mbc3TimerBattery | Mbc3TimerRamBattery2 => {
            mbc3::new_controller(header)
        }
        Mbc5 | Mbc5Ram | Mbc5RamBattery | Mbc5Rumble | Mbc5RumbleRam | Mbc5RumbleRamBattery => {
            mbc5::new_controller(header)
        }
        _ => panic!("unsupported cartridge type: {:?}", header.cartridge_type),
    }
}

pub fn generate_rom_controller(
    reader: impl std::io::Read,
    header: Header,
) -> Result<Generic, std::io::Error> {
    Generic::from_reader(header, reader)
}
