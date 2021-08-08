pub mod address_bus;

pub mod address;
pub mod cpu;
pub mod error;
pub mod memory;
pub mod processor;
pub mod registers;

pub use address_bus::{AddressBus, Area, FileOperation};
