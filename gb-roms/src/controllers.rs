pub mod bios;
pub mod mbc1;
pub mod mbc3;
pub mod mbc5;
pub mod rom_only;

pub use bios::Bios;
pub use mbc1::MBC1;
pub use mbc3::MBC3;
pub use mbc5::MBC5;
pub use rom_only::RomOnlyController;

pub trait Controller {
    /// Save the current controller into a Serializer file
    fn save<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    /// Load data from a Deserializer file
    fn load<'de, D>(&mut self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>;
}
