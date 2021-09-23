use super::{CycleDigest, MicrocodeController, MicrocodeFlow, State};

pub fn read(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = state.read();
    log::trace!("[microcode] byte read: {:#x}", byte);
    ctl.push(byte);
    MicrocodeFlow::Continue(CycleDigest::Consume)
}
