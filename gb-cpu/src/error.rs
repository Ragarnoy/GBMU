use crate::registers::area;

#[derive(Debug)]
pub enum Error {
    InvalidRelativeAddress(usize),
    InvalidAbsoluteAddress(u16),
    InvalidRegister(area::Area),
    InvalidPC(u16),
}
