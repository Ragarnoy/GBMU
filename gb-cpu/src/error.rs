#[derive(Debug)]
pub enum Error {
    InvalidPC(u16),
    IllegalSet(usize, u8),
    SegmentationFault(u16),
}
