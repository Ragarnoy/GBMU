use super::{math, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

pub fn sub(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let (value, flag) = math::sub_components(ctl.pop(), value);
    state.regs.set_subtraction(true);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn add(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let (value, flag) = math::add_components(ctl.pop(), value);
    state.regs.set_subtraction(false);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}
