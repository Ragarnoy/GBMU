use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION};
use crate::interfaces::Write8BitsReg;

pub fn a(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_a(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn b(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_b(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn c(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_c(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn d(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_d(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn e(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_e(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn h(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_h(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn l(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_l(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

pub fn bc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.bc = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    OK_PLAY_NEXT_ACTION
}

pub fn de(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.de = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    OK_PLAY_NEXT_ACTION
}

pub fn hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    OK_PLAY_NEXT_ACTION
}

pub fn sp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    OK_PLAY_NEXT_ACTION
}

pub fn ind(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    let value = ctl.pop();
    state.write_bus(addr, value);
    OK_CONSUME_CYCLE
}
