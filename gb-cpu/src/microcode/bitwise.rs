use super::{MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

pub fn bit_0(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let operation = value & 1;

    state.regs.set_zero(operation == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(true);
    OK_PLAY_NEXT_ACTION
}
