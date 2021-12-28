use super::{MicrocodeController, MicrocodeFlow, State, CONTINUE};
use crate::interfaces::Read8BitsReg;

/// Read a byte and push it to the cache stack
pub fn byte(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = state.read();
    ctl.push(byte);
    CONTINUE
}

/// Read `A` register, do not consume the current cycle
pub fn a(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.a());
    CONTINUE
}

/// Read `B` register, do not consume the current cycle
pub fn b(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.b());
    CONTINUE
}

/// Read `C` register, do not consume the current cycle
pub fn c(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.c());
    CONTINUE
}

/// Read `D` register, do not consume the current cycle
pub fn d(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.d());
    CONTINUE
}

/// Read `E` register, do not consume the current cycle
pub fn e(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.e());
    CONTINUE
}

/// Read `H` register, do not consume the current cycle
pub fn h(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.h());
    CONTINUE
}

/// Read `L` register, do not consume the current cycle
pub fn l(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.l());
    CONTINUE
}

/// Read `BC` register, do not consume the current cycle
pub fn bc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.bc);
    CONTINUE
}

/// Read `DE` register, do not consume the current cycle
pub fn de(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.de);
    CONTINUE
}

/// Read `HL` register, do not consume the current cycle
pub fn hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.hl);
    CONTINUE
}

/// Read `HL` register, do not consume the current cycle
pub fn af(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.af);
    CONTINUE
}

/// Read `SP` register, do not consume the current cycle
pub fn sp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.sp);
    CONTINUE
}

/// Read `PC` register, do not consume the current cycle
pub fn pc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push_u16(state.regs.pc);
    CONTINUE
}

/// Read the byte from the u16 addr stored in the cache, do consume the cycle
pub fn ind(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = ctl.pop_u16();
    let value = state.read_bus(addr);
    ctl.push(value);
    CONTINUE
}

/// Read the byte from the address resulting of `u8 + 0xFF00` where `u8` come from the cache
pub fn hram(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = 0xFF00 + ctl.pop() as u16;
    let value = state.read_bus(addr);
    ctl.push(value);
    CONTINUE
}
