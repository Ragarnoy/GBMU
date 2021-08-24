use crate::Address;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

impl Error {
    pub fn new_bus_error(addr: Box<dyn Address>) -> Self {
        Self::BusError(u16::from(addr))
    }

    pub fn new_segfault(addr: Box<dyn Address>) -> Self {
        Self::SegmentationFault(u16::from(addr))
    }
}
