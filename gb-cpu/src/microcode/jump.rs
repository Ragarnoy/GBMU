use super::{CycleDigest, MicrocodeController, MicrocodeFlow, State};

pub fn jump(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    MicrocodeFlow::Continue(CycleDigest::Consume)
}

/// Jump to the address which value is `HL`
pub fn jump_hl(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = state.regs.hl;
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    MicrocodeFlow::Continue(CycleDigest::Again)
}
