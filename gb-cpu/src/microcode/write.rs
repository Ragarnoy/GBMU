use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE};

/// Write the byte at the top of the stack to the address `HL`
pub fn write_hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = ctl.pop();
    log::trace!("[microcode] write {:02x} at (HL)", byte);
    state.write_hl(byte);
    OK_CONSUME_CYCLE
}
