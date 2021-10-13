use super::{math::sub_components, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

pub fn cp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let (_, flag) = sub_components(ctl.pop(), value);
    state.regs.set_subtraction(true);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    state.regs.set_zero(flag.zero);
    OK_PLAY_NEXT_ACTION
}

pub fn xor(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() ^ value;

    state.regs.set_raw(0);
    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn cpl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();

    ctl.push(!value);
    state.regs.set_half_carry(true);
    state.regs.set_subtraction(true);
    OK_PLAY_NEXT_ACTION
}

pub fn and(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() & value;

    state.regs.set_raw(0);
    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(true);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn or(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() | value;

    state.regs.set_raw(0);
    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}
