use super::{arithmetic, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
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

fn set_bit(ctl: &mut MicrocodeController, bit: u8) -> MicrocodeFlow {
    let bit_to_set = 1_u8 << bit;
    let value = ctl.pop() | bit_to_set;

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn set_0(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 0)
}

pub fn set_1(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 1)
}

pub fn set_2(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 2)
}

pub fn set_3(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 3)
}

pub fn set_4(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 4)
}

pub fn set_5(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 5)
}

pub fn set_6(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 6)
}

pub fn set_7(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    set_bit(ctl, 7)
}

fn res_bit(ctl: &mut MicrocodeController, bit: u8) -> MicrocodeFlow {
    let bit_to_res = 1_u8 << bit;
    let value = ctl.pop() & !bit_to_res;

    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

pub fn res_0(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 0)
}

pub fn res_1(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 1)
}

pub fn res_2(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 2)
}

pub fn res_3(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 3)
}

pub fn res_4(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 4)
}

pub fn res_5(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 5)
}

pub fn res_6(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 6)
}

pub fn res_7(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    res_bit(ctl, 7)
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

pub fn swap(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let (upper, lower) = arithmetic::slice_byte(value);
    let swapped_value = (lower << 4) + upper;

    ctl.push(swapped_value);
    state.regs.set_zero(value == 0);
    state.regs.set_carry(false);
    state.regs.set_half_carry(false);
    state.regs.set_subtraction(false);
    OK_PLAY_NEXT_ACTION
}
