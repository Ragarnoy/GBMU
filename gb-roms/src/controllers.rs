pub mod mbc1;
pub mod mbc5;
pub mod rom_only;

pub use mbc1::MBC1;
pub use mbc5::MBC5;
pub use rom_only::RomOnlyController;

pub trait Controller {
    /// Save the current controller into a Serializer file
    fn save<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(0xDead)
    }

    /// Load data from a Deserializer file
    fn load<'de, D>(&self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(())
    }
}
