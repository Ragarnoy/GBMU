use crate::Address;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BusError(Address),
    SegmentationFault(Address),
}
