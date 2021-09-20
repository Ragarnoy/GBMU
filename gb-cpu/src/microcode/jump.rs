use super::{Continuum, MicrocodeController, State};
use gb_bus::Bus;

pub fn jump<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    log::trace!("jumping to {}", addr);
    state.regs.pc = addr;
    Continuum::Ok
}
