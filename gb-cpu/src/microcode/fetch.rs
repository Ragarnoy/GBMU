use super::{
    arithmetic,
    condition::{carry, not_carry, not_zero, zero},
    dec,
    fetch_cb::fetch_cb,
    ident::{Ident, Reg16, Reg8},
    inc, jump, logic,
    opcode::Opcode,
    read::{read, read_hl},
    write::write_hl,
    CycleDigest, MicrocodeController, MicrocodeFlow, State,
};
use std::{cell::RefCell, convert::TryFrom, rc::Rc};

pub fn fetch(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let ctl_ref = Rc::new(RefCell::new(ctl));
    let byte = state.read();
    Opcode::try_from(byte).map_or_else(
        |e| {
            ctl_ref.borrow_mut().opcode = None;
            log::warn!("invalid opcode {}", e);
            MicrocodeFlow::Break(CycleDigest::Consume)
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

                Opcode::Jr => ctl.push_actions(&[read, jump::jump_relative]),
                Opcode::JrZ => ctl.push_actions(&[read, zero, jump::jump_relative]),
                Opcode::JrNz => ctl.push_actions(&[read, not_zero, jump::jump_relative]),
                Opcode::JrC => ctl.push_actions(&[read, carry, jump::jump_relative]),
                Opcode::JrNc => ctl.push_actions(&[read, not_carry, jump::jump_relative]),

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
                Opcode::IncHLind => ctl.push_actions(&[read_hl, inc::inc_hl, write_hl]),

                Opcode::DecBC => ctl.push_actions(&[dec::dec16]).set_dest(Reg16::BC.into()),
                Opcode::DecDE => ctl.push_actions(&[dec::dec16]).set_dest(Reg16::DE.into()),
                Opcode::DecHL => ctl.push_actions(&[dec::dec16]).set_dest(Reg16::HL.into()),
                Opcode::DecSP => ctl.push_actions(&[dec::dec16]).set_dest(Reg16::SP.into()),

                Opcode::DecA => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::A.into()),
                Opcode::DecB => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::B.into()),
                Opcode::DecC => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::C.into()),
                Opcode::DecD => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::D.into()),
                Opcode::DecE => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::E.into()),
                Opcode::DecH => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::H.into()),
                Opcode::DecL => ctl.push_actions(&[dec::dec8]).set_dest(Reg8::L.into()),
                Opcode::DecHLind => ctl.push_actions(&[read_hl, dec::dec_hl, write_hl]),

                Opcode::CpAA => ctl.push_actions(&[logic::cp]).set_src(Reg8::A.into()),
                Opcode::CpAB => ctl.push_actions(&[logic::cp]).set_src(Reg8::B.into()),
                Opcode::CpAC => ctl.push_actions(&[logic::cp]).set_src(Reg8::C.into()),
                Opcode::CpAD => ctl.push_actions(&[logic::cp]).set_src(Reg8::D.into()),
                Opcode::CpAE => ctl.push_actions(&[logic::cp]).set_src(Reg8::E.into()),
                Opcode::CpAH => ctl.push_actions(&[logic::cp]).set_src(Reg8::H.into()),
                Opcode::CpAL => ctl.push_actions(&[logic::cp]).set_src(Reg8::L.into()),
                Opcode::CpAHL => ctl
                    .push_actions(&[read_hl, logic::cp])
                    .set_src(Ident::IndirectHL8),
                Opcode::CpA8 => ctl.push_actions(&[read, logic::cp]).set_src(Ident::Raw8),

                Opcode::SubAA => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::A.into()),
                Opcode::SubAB => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::B.into()),
                Opcode::SubAC => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::C.into()),
                Opcode::SubAD => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::D.into()),
                Opcode::SubAE => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::E.into()),
                Opcode::SubAH => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::H.into()),
                Opcode::SubAL => ctl.push_actions(&[arithmetic::sub]).set_src(Reg8::L.into()),
                Opcode::SubAHL => ctl
                    .push_actions(&[read_hl, arithmetic::sub])
                    .set_src(Ident::IndirectHL8),
                Opcode::SubA8 => ctl
                    .push_actions(&[read, arithmetic::sub])
                    .set_src(Ident::Raw8),

                Opcode::AddAA => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::A.into()),
                Opcode::AddAB => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::B.into()),
                Opcode::AddAC => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::C.into()),
                Opcode::AddAD => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::D.into()),
                Opcode::AddAE => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::E.into()),
                Opcode::AddAH => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::H.into()),
                Opcode::AddAL => ctl.push_actions(&[arithmetic::add]).set_src(Reg8::L.into()),
                Opcode::AddAHL => ctl
                    .push_actions(&[read_hl, arithmetic::add])
                    .set_src(Ident::IndirectHL8),
                Opcode::AddA8 => ctl
                    .push_actions(&[read, arithmetic::add])
                    .set_src(Ident::Raw8),

                Opcode::Nop => &mut ctl,
                Opcode::PrefixCb => ctl.push_action(fetch_cb),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
