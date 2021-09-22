use super::{ControlFlow, MicrocodeController, State};

pub fn read(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let byte = state.read();
    log::trace!("[microcode] byte read: {:#x}", byte);
    ctl.push(byte);
    ControlFlow::Ok
}
