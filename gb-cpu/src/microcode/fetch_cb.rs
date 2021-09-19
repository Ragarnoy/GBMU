use super::{
    controller::{MicrocodeController, State},
    opcode_cb::OpcodeCB,
    Continuum,
};
use gb_bus::Bus;
use std::convert::TryFrom;

pub fn fetch_cb<B: Bus<u8>>(_ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let byte = state.read();
    OpcodeCB::try_from(byte).map_or(Continuum::Err, |opcode| {
        unimplemented!("cb prefixed opcode not implemented: {:?}", opcode)
    })
}
