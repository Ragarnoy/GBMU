use super::{
    controller::{MicrocodeController, State},
    opcode_cb::OpcodeCB,
    Continuum,
};
use gb_bus::Bus;
use std::convert::TryFrom;

pub fn fetch_cb<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let byte = state.read();
    OpcodeCB::try_from(byte).map_or_else(
        |e| {
            panic!(
                "how it's possible for an u8({}) to be outside the range of 0..ff: {}",
                byte, e
            );
        },
        |opcode| {
            ctl.opcode = Some(opcode.into());
            unimplemented!("cb prefixed opcode not implemented: {:?}", opcode)
        },
    )
}
