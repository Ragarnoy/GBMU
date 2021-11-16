use super::{math::sub_components, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

pub fn cp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let left = ctl.pop();
    let right = ctl.pop();
    let (_, flag) = sub_components(left, right);
    state.regs.set_substraction(true);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    state.regs.set_zero(flag.zero);
    OK_PLAY_NEXT_ACTION
}

pub fn xor(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() ^ value;

    state.regs.set_zero(value == 0);
    state.regs.set_substraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn cpl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();

    ctl.push(!value);
    state.regs.set_half_carry(true);
    state.regs.set_substraction(true);
    OK_PLAY_NEXT_ACTION
}

pub fn and(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() & value;

    state.regs.set_zero(value == 0);
    state.regs.set_substraction(false);
    state.regs.set_half_carry(true);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn or(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() | value;

    state.regs.set_zero(value == 0);
    state.regs.set_substraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}
