use super::{math::sub_components, MicrocodeController, MicrocodeFlow, State, CONTINUE};
use crate::interfaces::WriteFlagReg;

pub fn cp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let left = ctl.pop();
    let right = ctl.pop();
    let (_, flag) = sub_components(left, right, false);
    state.regs.set_subtraction(true);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    state.regs.set_zero(flag.zero);
    CONTINUE
}

pub fn xor(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() ^ value;

    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    CONTINUE
}

pub fn cpl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();

    ctl.push(!value);
    state.regs.set_half_carry(true);
    state.regs.set_subtraction(true);
    CONTINUE
}

pub fn and(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() & value;

    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(true);
    state.regs.set_carry(false);

    ctl.push(value);
    CONTINUE
}

pub fn or(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let value = ctl.pop() | value;

    state.regs.set_zero(value == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(false);
    state.regs.set_carry(false);

    ctl.push(value);
    CONTINUE
}
