use super::{
    flag::Flag, math::sub_components, MicrocodeController, MicrocodeFlow, State, CONTINUE,
};
use crate::interfaces::WriteFlagReg;

pub fn dec16(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    let (value, _) = ctl.pop_u16().overflowing_sub(1);
    ctl.push_u16(value);
    CONTINUE
}

pub fn dec8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = sub_components(ctl.pop(), 1, false);
    update_dec_flag(state.regs, flag);
    ctl.push(value);
    CONTINUE
}

fn update_dec_flag(state: &mut impl WriteFlagReg, flag: Flag) {
    state.set_half_carry(flag.half_carry);
    state.set_zero(flag.zero);
    state.set_subtraction(true);
}

/// decrease sp by one
pub fn sp(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp -= 1;
    CONTINUE
}

/// decrease hl by one
pub fn hl(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl -= 1;
    CONTINUE
}
