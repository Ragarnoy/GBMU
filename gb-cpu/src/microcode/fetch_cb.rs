use super::{opcode_cb::OpcodeCB, ControlFlow, MicrocodeController, State};
use std::convert::TryFrom;

pub fn fetch_cb(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
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
