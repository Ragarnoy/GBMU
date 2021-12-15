mod builder;
pub mod bus;
pub mod cpu;
pub mod interfaces;
mod interrupt_flags;
pub mod microcode;
pub mod registers;
#[cfg(test)]
mod registers_test;

pub use builder::new_cpu;

pub const NB_MAX_CYCLES: usize = 6;
pub const NB_MAX_ACTIONS: usize = 8;
pub const CACHE_LEN: usize = 6;
