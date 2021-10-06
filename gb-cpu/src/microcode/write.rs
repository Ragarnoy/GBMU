use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION};
use crate::interfaces::Write8BitsReg;

/// Write the value stored in cache to `A`, do not consume the cycle
pub fn a(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_a(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `B`, do not consume the cycle
pub fn b(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_b(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `C`, do not consume the cycle
pub fn c(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_c(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `D`, do not consume the cycle
pub fn d(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_d(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `E`, do not consume the cycle
pub fn e(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_e(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `H`, do not consume the cycle
pub fn h(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_h(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `L`, do not consume the cycle
pub fn l(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_l(ctl.pop());
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `BC`, do not consume the cycle
pub fn bc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.bc = ctl.pop_u16();
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `DE`, do not consume the cycle
pub fn de(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.de = ctl.pop_u16();
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `HL`, do not consume the cycle
pub fn hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl = ctl.pop_u16();
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `HL`, do not consume the cycle
pub fn af(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl = ctl.pop_u16();
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to `SP`, do not consume the cycle
pub fn sp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp = ctl.pop_u16();
    OK_PLAY_NEXT_ACTION
}

/// Write the value stored in cache to the u16 address stored in cache, do consume the cycle
pub fn ind(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    let value = ctl.pop();
    state.write_bus(addr, value);
    OK_CONSUME_CYCLE
}
