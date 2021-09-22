use super::{
    condition::{carry, not_carry, not_zero, zero},
    fetch_cb::fetch_cb,
    ident::{Reg16, Reg8},
    inc, jump,
    opcode::Opcode,
    read::read,
    ControlFlow, MicrocodeController, State,
};
use std::{cell::RefCell, convert::TryFrom, rc::Rc};

pub fn fetch(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let ctl_ref = Rc::new(RefCell::new(ctl));
    let byte = state.read();
    Opcode::try_from(byte).map_or_else(
        |e| {
            ctl_ref.borrow_mut().opcode = None;
            log::warn!("invalid opcode {}", e);
            ControlFlow::Err
        },
        |opcode| {
            let mut ctl = ctl_ref.borrow_mut();
            ctl.opcode = Some(opcode.into());
            match opcode {
                Opcode::Jp => ctl.push_actions(&[read, read, jump::jump]),
                Opcode::JpZ => ctl.push_actions(&[read, read, zero, jump::jump]),
                Opcode::JpNz => ctl.push_actions(&[read, read, not_zero, jump::jump]),
                Opcode::JpC => ctl.push_actions(&[read, read, carry, jump::jump]),
                Opcode::JpNc => ctl.push_actions(&[read, read, not_carry, jump::jump]),
                Opcode::JpHl => ctl.push_actions(&[jump::jump_hl]),

                Opcode::IncBC => ctl.push_actions(&[inc::inc16]).set_dest(Reg16::BC.into()),
                Opcode::IncDE => ctl.push_actions(&[inc::inc16]).set_dest(Reg16::DE.into()),
                Opcode::IncHL => ctl.push_actions(&[inc::inc16]).set_dest(Reg16::HL.into()),
                Opcode::IncSP => ctl.push_actions(&[inc::inc16]).set_dest(Reg16::SP.into()),

                Opcode::IncA => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::A.into()),
                Opcode::IncB => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::B.into()),
                Opcode::IncC => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::C.into()),
                Opcode::IncD => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::D.into()),
                Opcode::IncE => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::E.into()),
                Opcode::IncH => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::H.into()),
                Opcode::IncL => ctl.push_actions(&[inc::inc8]).set_dest(Reg8::L.into()),
                Opcode::IncHLind => ctl.push_actions(&[inc::inc_hl]),

                Opcode::Nop => &mut ctl,

                Opcode::PrefixCb => ctl.push_action(fetch_cb),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            ControlFlow::Ok
        },
    )
}
