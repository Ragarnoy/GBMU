use super::{ident::Ident, MicrocodeController, MicrocodeFlow, State};

pub fn load(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.get_src();
    match value {
        Ident::Raw16 => load_from_16(ctl, state),
        _ => unimplemented!("cannot retrieve value from {:?}", value),
    }
}

fn load_from_16(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
}
