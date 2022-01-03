#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

impl Error {
    pub fn bus_error(addr: u16) -> Self {
        Self::BusError(addr)
    }

    pub fn new_segfault(addr: u16) -> Self {
        Self::SegmentationFault(addr)
    }
}
