use crate::interfaces::{ReadFlagReg, WriteFlagReg};

use super::{MicrocodeController, MicrocodeFlow, State, CONTINUE};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Flag {
    pub half_carry: bool,
    pub carry: bool,
    pub negative: bool,
    pub zero: bool,
}

pub fn scf(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_carry(true);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    CONTINUE
}

pub fn ccf(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_carry(!state.regs.carry());
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    CONTINUE
}

pub fn reset_flag_zero(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_zero(false);
    CONTINUE
}
