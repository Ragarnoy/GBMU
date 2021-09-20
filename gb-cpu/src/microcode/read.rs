use super::{Continuum, MicrocodeController, State};
use gb_bus::Bus;

pub fn read<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let byte = state.read();
    ctl.push(byte);
    Continuum::Ok
}
