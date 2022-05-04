mod builder;
pub mod bus;
pub mod constant;
pub mod cpu;
pub mod interfaces;
pub mod io_registers;
pub mod microcode;
pub mod registers;
#[cfg(test)]
mod registers_test;

pub use builder::new_cpu;

pub const NB_MAX_CYCLES: usize = 6;
pub const NB_MAX_ACTIONS: usize = 8;
pub const CACHE_LEN: usize = 6;
pub const MAX_CYCLES_IN_HALT_MODE: usize = 0x800; // 0x2000 t-cycles
