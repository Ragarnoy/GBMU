use super::{MicrocodeController, MicrocodeFlow, State, CONTINUE};
use crate::interfaces::Write8BitsReg;

const FLAG_MASK: u16 = 0xfff0;

/// Write the value stored in cache to `A`, do not consume the cycle
pub fn a(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_a(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `B`, do not consume the cycle
pub fn b(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_b(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `C`, do not consume the cycle
pub fn c(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_c(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `D`, do not consume the cycle
pub fn d(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_d(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `E`, do not consume the cycle
pub fn e(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_e(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `H`, do not consume the cycle
pub fn h(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_h(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `L`, do not consume the cycle
pub fn l(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.set_l(ctl.pop());
    CONTINUE
}

/// Write the value stored in cache to `BC`, do not consume the cycle
pub fn bc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.bc = ctl.pop_u16();
    CONTINUE
}

/// Write the value stored in cache to `DE`, do not consume the cycle
pub fn de(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.de = ctl.pop_u16();
    CONTINUE
}

/// Write the value stored in cache to `HL`, do not consume the cycle
pub fn hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl = ctl.pop_u16();
    CONTINUE
}

/// Write the value stored in cache to `AF`, do not consume the cycle
pub fn af(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.af = ctl.pop_u16() & FLAG_MASK;
    CONTINUE
}

/// Write the value stored in cache to `SP`, do not consume the cycle
pub fn sp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp = ctl.pop_u16();
    CONTINUE
}

/// Write the value stored in cache to the u16 address stored in cache, do consume the cycle
pub fn ind(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = ctl.pop_u16();
    let value = ctl.pop();
    state.write_bus(addr, value);
    CONTINUE
}

/// Write the value stored in cache to result of `u8 + 0xFF00` where `u8` is stored in the cache
pub fn hram(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = 0xff00 + ctl.pop() as u16;
    let value = ctl.pop();
    state.write_bus(addr, value);
    CONTINUE
}

/// Write the u16 value stored in cache to the u16 address also stored in cache, do consume the
/// cycle
pub fn ind16(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = ctl.pop_u16();
    state.write_bus(addr, ctl.pop());
    state.write_bus(addr + 1, ctl.pop());
    CONTINUE
}
