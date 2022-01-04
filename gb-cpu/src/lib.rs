mod builder;
pub mod bus;
pub mod constant;
pub mod cpu;
pub mod interfaces;
mod io_registers;
pub mod microcode;
pub mod registers;
#[cfg(test)]
mod registers_test;

pub use builder::new_cpu;
pub use cpu::Cpu;
pub use io_registers::IORegisters;
pub use io_registers::Speed;

pub const NB_MAX_CYCLES: usize = 6;
pub const NB_MAX_ACTIONS: usize = 8;
pub const CACHE_LEN: usize = 6;
