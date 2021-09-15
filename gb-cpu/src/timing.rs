use gb_roms::Opcode;

pub trait Timing {
    fn timing(&self) -> usize;
}

impl Timing for Opcode {
    fn timing(&self) -> usize {
        match self {
            _ => unimplemented!("timing missing for {:?}", self),
        }
    }
}
