use super::{
    bitwise, opcode_cb::OpcodeCB, read, CycleDigest, MicrocodeController, MicrocodeFlow, State,
};
use std::convert::TryFrom;

pub fn fetch_cb(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
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
            match opcode {
                OpcodeCB::Bit0B => ctl.push_actions(&[read::b, bitwise::bit_0]),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
