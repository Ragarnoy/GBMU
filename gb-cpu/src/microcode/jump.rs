use super::{ControlFlow, MicrocodeController, State};

pub fn jump(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    ControlFlow::Ok
}

/// Jump to the address which value is `HL`
pub fn jump_hl(_ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let addr = state.regs.hl;
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    ControlFlow::Chain
}
