use gb_roms::Opcode;

pub trait Timing {
    fn timing(&self) -> usize;
}

impl Timing for Opcode {
    fn timing(&self) -> usize {
        unimplemented!("timing missing for {:?}", self);
    }
}
