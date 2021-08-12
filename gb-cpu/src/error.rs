#[derive(Debug)]
pub enum Error {
    InvalidPC(u16),
    SetError(usize, u8),
    SegmentationFault(u16),
}
