use super::{Continuum, MicrocodeController, State};
use gb_bus::Bus;

pub fn jump<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    Continuum::Ok
}

/// Jump to the address which value is `HL`
pub fn jump_hl<B: Bus<u8>>(_ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let addr = state.regs.hl;
    log::trace!("[microcode] jumping to {:#x}", addr);
    state.regs.pc = addr;
    Continuum::Chain
}
