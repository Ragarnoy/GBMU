use super::{CycleDigest, MicrocodeController, MicrocodeFlow, State};

/// Read a byte and push it to the cache stack
pub fn read(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = state.read();
    log::trace!("[microcode] byte read: {:02x}", byte);
    ctl.push(byte);
    ControlFlow::Ok
}

/// Read a byte at the address of `HL` and push it to the stack
pub fn read_hl(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let byte = state.read_hl();
    log::trace!("[microcode] byte read at (HL): {:02x}", byte);
    ctl.push(byte);
    MicrocodeFlow::Continue(CycleDigest::Consume)
}
