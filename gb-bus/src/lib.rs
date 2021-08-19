pub mod address_bus;
pub mod file_operation;
pub mod generic;

pub use address_bus::{Address, AddressBus, Area, Error};
pub use file_operation::FileOperation;

trait Bus {
    /// read `N` into the bus where `N: u8 | u16`
    fn read<N>(address: u16) -> Result<N, Error>;
    /// write `N` into the bus where `N: u8 | u16`
    fn write<N>(address: u16, data: N) -> Result<(), Error>;
}
