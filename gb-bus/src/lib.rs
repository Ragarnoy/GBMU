pub mod address;
pub mod address_bus;
pub mod area;
pub mod constant;
pub mod error;
pub mod file_operation;
pub mod generic;
pub mod io_reg_area;
mod io_reg_bus;
pub mod io_reg_constant;
mod lock;
mod working_ram;

pub use address_bus::AddressBus;
pub use area::Area;
pub use error::Error;
pub use file_operation::{Address, FileOperation};
pub use io_reg_area::IORegArea;
pub use io_reg_bus::IORegBus;
pub use lock::{Lock, MemoryLock};
pub use working_ram::WorkingRam;

pub trait Bus<N>: MemoryLock {
    /// read `N` into the bus
    fn read(&self, address: u16, lock_key: Option<Lock>) -> Result<N, Error>;
    /// write `N` into the bus
    fn write(&mut self, address: u16, data: N, lock_key: Option<Lock>) -> Result<(), Error>;
}
