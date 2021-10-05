use super::{CycleDigest, MicrocodeController, MicrocodeFlow, State};

pub fn jump(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = ctl.pop_u16();
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

/// Jump to the address `PC + i8`
pub fn jump_relative(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let relative_addr = i8::from_be_bytes([ctl.pop()]);
    let addr = if relative_addr >= 0 {
        state.regs.pc + relative_addr as u16
    } else {
        state.regs.pc - (-relative_addr as u16)
    };
    log::trace!(
        "[microcode] jumping to relative {:#x}, final addr {:#x}",
        relative_addr,
        addr
    );
    state.regs.pc = addr;
    MicrocodeFlow::Continue(CycleDigest::Consume)
}
