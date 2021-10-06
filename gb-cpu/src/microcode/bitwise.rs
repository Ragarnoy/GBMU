use super::{MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::WriteFlagReg;

fn read_bit(ctl: &mut MicrocodeController, state: &mut State, bit: u8) -> MicrocodeFlow {
    let value = ctl.pop();
    let operation = (value >> bit) & 1;

    state.regs.set_zero(operation == 0);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(true);
    OK_PLAY_NEXT_ACTION
}

pub fn bit_0(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 0)
}

pub fn bit_1(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 1)
}

pub fn bit_2(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 2)
}

pub fn bit_3(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 3)
}

pub fn bit_4(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 4)
}

pub fn bit_5(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 5)
}

pub fn bit_6(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 6)
}

pub fn bit_7(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    read_bit(ctl, state, 7)
}