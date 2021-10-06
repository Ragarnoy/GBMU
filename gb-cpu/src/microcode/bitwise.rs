use super::{MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

fn read_bit(ctl: &mut MicrocodeController, state: &mut State, bit: u8) {
    let value = ctl.pop();
    let operation = (value >> bit) & 1;

    state.regs.set_zero(operation == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(true);
}

pub fn bit_0(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 0);
    OK_PLAY_NEXT_ACTION
}
