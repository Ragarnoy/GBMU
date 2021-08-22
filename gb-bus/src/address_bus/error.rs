use crate::Address;

#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

impl Error {
    pub fn new_bus_error(addr: Box<dyn Address>) -> Self {
        Self::BusError(addr.get_address() as u16)
    }

    pub fn new_segfault(addr: Box<dyn Address>) -> Self {
        Self::SegmentationFault(addr.get_address() as u16)
    }
}
