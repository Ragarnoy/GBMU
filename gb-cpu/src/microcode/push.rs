use super::{MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};

fn push_addr(ctl: &mut MicrocodeController, addr: u8) -> MicrocodeFlow {
    ctl.push_u16(addr as u16);
    OK_PLAY_NEXT_ACTION
}

pub fn addr_0000(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x00);
    OK_PLAY_NEXT_ACTION
}

pub fn addr_0008(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x08);
    OK_PLAY_NEXT_ACTION
}

pub fn addr_0010(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x10);
    OK_PLAY_NEXT_ACTION
}
pub fn addr_0018(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x18);
    OK_PLAY_NEXT_ACTION
}
pub fn addr_0020(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x20);
    OK_PLAY_NEXT_ACTION
}
pub fn addr_0028(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x28);
    OK_PLAY_NEXT_ACTION
}
pub fn addr_0030(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x30);
    OK_PLAY_NEXT_ACTION
}
pub fn addr_0038(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    push_addr(ctl, 0x38);
    OK_PLAY_NEXT_ACTION
}
