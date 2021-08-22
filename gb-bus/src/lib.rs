pub mod address;
pub mod address_bus;
pub mod error;
pub mod file_operation;
pub mod generic;

pub use address::Area;
pub use address_bus::AddressBus;
pub use error::Error;
pub use file_operation::{Address, FileOperation};

pub trait Bus<N> {
    /// read `N` into the bus
    fn read(&self, address: u16) -> Result<N, Error>;
    /// write `N` into the bus
    fn write(&mut self, address: u16, data: N) -> Result<(), Error>;
}
