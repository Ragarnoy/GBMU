use crate::file_operation::Address;

#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(Box<dyn Address>),
    SegmentationFault(Box<dyn Address>),
}
