use super::{Continuum, MicrocodeController, State};
use gb_bus::Bus;

pub fn read<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let byte = state.read();
    log::trace!("[microcode] byte read: {:#x}", byte);
    ctl.push(byte);
    Continuum::Ok
}
