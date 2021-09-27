use super::{ident, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::{Read8BitsReg, Write8BitsReg};

pub fn cp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = {
        use ident::{Ident, Reg8};

        let src = ctl.get_src();
        match src {
            Ident::Reg8(r8) => match r8 {
                Reg8::A => state.regs.a(),
                Reg8::B => state.regs.b(),
                Reg8::C => state.regs.c(),
                Reg8::D => state.regs.d(),
                Reg8::E => state.regs.e(),
                Reg8::H => state.regs.h(),
                Reg8::L => state.regs.l(),
            },
            Ident::Raw8 | Ident::IndirectHL8 => ctl.pop(),
            _ => panic!("CP don't handle source of type {:?}", src),
        }
    };
    OK_PLAY_NEXT_ACTION
}
