use super::{
    controller::{MicrocodeController, State},
    fetch_cb::fetch_cb,
    opcode::Opcode,
    Continuum,
};
use gb_bus::Bus;
use std::convert::TryFrom;

pub fn fetch<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    let byte = state.read();
    Opcode::try_from(byte).map_or(Continuum::Err, |opcode| {
        match opcode {
            Opcode::Nop => {}
            Opcode::PrefixCb => ctl.push_action(fetch_cb),
            _ => todo!("unimplemented opcode {:?}", opcode),
        };
        Continuum::Ok
    })
}
