use super::{
    controller::{MicrocodeController, State},
    Continuum,
};
use gb_bus::Bus;

pub fn read<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    state.bus.
}
