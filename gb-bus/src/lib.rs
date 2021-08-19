pub mod address;
pub mod address_bus;
pub mod file_operation;
pub mod generic;

pub use address::{Address, Area};
pub use address_bus::{AddressBus, Error};
pub use file_operation::FileOperation;

pub trait Bus<N> {
    /// read `N` into the bus
    fn read(address: u16) -> Result<N, Error>;
    /// write `N` into the bus
    fn write(address: u16, data: N) -> Result<(), Error>;
}
