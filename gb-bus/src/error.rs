use crate::Address;
use std::convert::Into;

#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

impl Error {
    pub fn new_bus_error<A: Into<u16>>(addr: Box<dyn Address<A>>) -> Self {
        Self::BusError(addr.into())
    }

    pub fn new_segfault<A: Into<u16>>(addr: Box<dyn Address<A>>) -> Self {
        Self::SegmentationFault(addr.into())
    }
}
