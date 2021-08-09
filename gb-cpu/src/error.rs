use crate::processor::cpu::registers::area;

#[derive(Debug)]
pub enum Error {
    InvalidAbsoluteAddress(u16),
    InvalidRegister(area::Area),
    InvalidPC(u16),
}
