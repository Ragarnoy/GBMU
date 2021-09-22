use super::{ControlFlow, MicrocodeController, State, ident::{Ident, self}};
use gb_bus::Bus;

pub fn inc16<B: Bus<u8>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> ControlFlow {
    if let Ident::Reg16(r16) = ctl.get_dest() {
        use ident::Reg16;
        match r16 {
            Reg16::BC => state.regs.bc += 1,
            Reg16::DE => state.regs.de += 1,
            Reg16::HL => state.regs.hl += 1,
            Reg16::SP => state.regs.sp += 1,
        }
        ControlFlow::Ok;
    } else {
        panic!("call inc16 with something other than a reg16");
    }
}

pub fn inc8<B: Bus<u8>>(ctl: &mut MicrocodeController<B>, state: &mut State<B>) -> ControlFlow {
    if let Ident::Reg8(r8) = ctl.get_dest() {
        use ident::Reg8;
        match r8 {
            Reg8::A => state.regs.a += 1,
        }
        ControlFlow::Ok;
    } else {
        panic!("call inc8 with something other than a reg8");
    }
}

#[derive(Default)]
struct Flag {
    half_carry: Option<bool>,
    carry: Option<bool>,
    negative: Option<bool>,
    zero: Option<bool>,
}

fn add_reg_flags(value: u8, amount: u8) -> (u8, Flag) {
    let mut flags = Flag::default()
    let res = value + amount;
    flags.zero = Some(res == 0)
    (res, flags)
}
