use super::{MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::{ReadFlagReg, WriteFlagReg};

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

pub fn rl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let top_bit = value >> 7;
    value <<= 1;
    value += state.regs.carry() as u8;

    ctl.push(value);

    state.regs.set_zero(value == 0);
    state.regs.set_carry(top_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn rlc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let top_bit = value >> 7;
    value <<= 1;
    value += top_bit;

    ctl.push(value);

    state.regs.set_zero(value == 0);
    state.regs.set_carry(top_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn rr(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let lower_bit = value & 1;
    value >>= 1;
    if state.regs.carry() {
        value += 0x80
    }

    ctl.push(value);

    state.regs.set_zero(value == 0);
    state.regs.set_carry(lower_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn srl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let lower_bit = value & 1;
    value >>= 1;

    ctl.push(value);

    state.regs.set_zero(value == 0);
    state.regs.set_carry(lower_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn sla(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let top_bit = value >> 7;
    value <<= 1;

    ctl.push(value);

    state.regs.set_zero(value == 0);

    state.regs.set_carry(top_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn rrc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let lower_bit = value & 1;
    value >>= 1;
    if lower_bit == 1 {
        value += 0x80;
    }

    ctl.push(value);

    state.regs.set_zero(value == 0);

    state.regs.set_carry(lower_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}

pub fn sra(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut value = ctl.pop();
    let top_bit = value & 0x80;
    let lower_bit = value & 1;
    value >>= 1;
    value |= top_bit;

    ctl.push(value);

    state.regs.set_zero(value == 0);
    state.regs.set_carry(lower_bit == 1);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}
