use crate::microcode::utils;

use super::{
    arithmetic,
    condition::{carry, not_carry, not_zero, zero},
    dec,
    fetch_cb::fetch_cb,
    inc, jump, logic,
    opcode::Opcode,
    read, write, CycleDigest, MicrocodeController, MicrocodeFlow, State,
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
                Opcode::Jp => ctl.push_actions(&[read::byte, read::byte, jump::jump]),
                Opcode::JpZ => ctl.push_actions(&[read::byte, read::byte, zero, jump::jump]),
                Opcode::JpNz => ctl.push_actions(&[read::byte, read::byte, not_zero, jump::jump]),
                Opcode::JpC => ctl.push_actions(&[read::byte, read::byte, carry, jump::jump]),
                Opcode::JpNc => ctl.push_actions(&[read::byte, read::byte, not_carry, jump::jump]),
                Opcode::JpHl => ctl.push_actions(&[jump::jump_hl]),

                Opcode::Jr => ctl.push_actions(&[read::byte, jump::jump_relative]),
                Opcode::JrZ => ctl.push_actions(&[read::byte, zero, jump::jump_relative]),
                Opcode::JrNz => ctl.push_actions(&[read::byte, not_zero, jump::jump_relative]),
                Opcode::JrC => ctl.push_actions(&[read::byte, carry, jump::jump_relative]),
                Opcode::JrNc => ctl.push_actions(&[read::byte, not_carry, jump::jump_relative]),

                Opcode::IncBC => ctl.push_actions(&[read::bc, inc::inc16, write::bc]),
                Opcode::IncDE => ctl.push_actions(&[read::de, inc::inc16, write::de]),
                Opcode::IncHL => ctl.push_actions(&[read::hl, inc::inc16, write::hl]),
                Opcode::IncSP => ctl.push_actions(&[read::sp, inc::inc16, write::sp]),

                Opcode::IncA => ctl.push_actions(&[read::a, inc::inc8, write::a]),
                Opcode::IncB => ctl.push_actions(&[read::b, inc::inc8, write::b]),
                Opcode::IncC => ctl.push_actions(&[read::c, inc::inc8, write::c]),
                Opcode::IncD => ctl.push_actions(&[read::d, inc::inc8, write::d]),
                Opcode::IncE => ctl.push_actions(&[read::e, inc::inc8, write::e]),
                Opcode::IncH => ctl.push_actions(&[read::h, inc::inc8, write::h]),
                Opcode::IncL => ctl.push_actions(&[read::l, inc::inc8, write::l]),
                Opcode::IncHLind => {
                    ctl.push_actions(&[read::hl, read::ind, inc::inc8, read::hl, write::ind])
                }
                Opcode::DecBC => ctl.push_actions(&[read::bc, dec::dec16, write::bc]),
                Opcode::DecDE => ctl.push_actions(&[read::de, dec::dec16, write::de]),
                Opcode::DecHL => ctl.push_actions(&[read::hl, dec::dec16, write::hl]),
                Opcode::DecSP => ctl.push_actions(&[read::sp, dec::dec16, write::sp]),

                Opcode::DecA => ctl.push_actions(&[read::a, dec::dec8, write::a]),
                Opcode::DecB => ctl.push_actions(&[read::b, dec::dec8, write::b]),
                Opcode::DecC => ctl.push_actions(&[read::c, dec::dec8, write::c]),
                Opcode::DecD => ctl.push_actions(&[read::d, dec::dec8, write::d]),
                Opcode::DecE => ctl.push_actions(&[read::e, dec::dec8, write::e]),
                Opcode::DecH => ctl.push_actions(&[read::h, dec::dec8, write::h]),
                Opcode::DecL => ctl.push_actions(&[read::l, dec::dec8, write::l]),
                Opcode::DecHLind => {
                    ctl.push_actions(&[read::hl, read::ind, dec::dec8, read::hl, write::ind])
                }
                Opcode::CpAA => ctl.push_actions(&[read::a, read::a, logic::cp]),
                Opcode::CpAB => ctl.push_actions(&[read::b, read::a, logic::cp]),
                Opcode::CpAC => ctl.push_actions(&[read::c, read::a, logic::cp]),
                Opcode::CpAD => ctl.push_actions(&[read::d, read::a, logic::cp]),
                Opcode::CpAE => ctl.push_actions(&[read::e, read::a, logic::cp]),
                Opcode::CpAH => ctl.push_actions(&[read::h, read::a, logic::cp]),
                Opcode::CpAL => ctl.push_actions(&[read::l, read::a, logic::cp]),
                Opcode::CpAHL => ctl.push_actions(&[read::hl, read::ind, read::a, logic::cp]),
                Opcode::CpA8 => ctl.push_actions(&[read::byte, read::a, logic::cp]),

                Opcode::XorAA => ctl.push_actions(&[read::a, read::a, logic::xor, write::a]),
                Opcode::XorAB => ctl.push_actions(&[read::b, read::a, logic::xor, write::a]),
                Opcode::XorAC => ctl.push_actions(&[read::c, read::a, logic::xor, write::a]),
                Opcode::XorAD => ctl.push_actions(&[read::d, read::a, logic::xor, write::a]),
                Opcode::XorAE => ctl.push_actions(&[read::e, read::a, logic::xor, write::a]),
                Opcode::XorAH => ctl.push_actions(&[read::h, read::a, logic::xor, write::a]),
                Opcode::XorAL => ctl.push_actions(&[read::l, read::a, logic::xor, write::a]),
                Opcode::XorAHL => {
                    ctl.push_actions(&[read::hl, read::ind, read::a, logic::xor, write::a])
                }
                Opcode::XorA8 => ctl.push_actions(&[read::byte, read::a, logic::xor, write::a]),

                Opcode::SubAA => ctl.push_actions(&[read::a, read::a, arithmetic::sub, write::a]),
                Opcode::SubAB => ctl.push_actions(&[read::b, read::a, arithmetic::sub, write::a]),
                Opcode::SubAC => ctl.push_actions(&[read::c, read::a, arithmetic::sub, write::a]),
                Opcode::SubAD => ctl.push_actions(&[read::d, read::a, arithmetic::sub, write::a]),
                Opcode::SubAE => ctl.push_actions(&[read::e, read::a, arithmetic::sub, write::a]),
                Opcode::SubAH => ctl.push_actions(&[read::h, read::a, arithmetic::sub, write::a]),
                Opcode::SubAL => ctl.push_actions(&[read::l, read::a, arithmetic::sub, write::a]),
                Opcode::SubAHL => {
                    ctl.push_actions(&[read::hl, read::ind, read::a, arithmetic::sub, write::a])
                }
                Opcode::SubA8 => {
                    ctl.push_actions(&[read::byte, read::a, arithmetic::sub, write::a])
                }

                Opcode::AddAA => ctl.push_actions(&[read::a, read::a, arithmetic::add, write::a]),
                Opcode::AddAB => ctl.push_actions(&[read::b, read::a, arithmetic::add, write::a]),
                Opcode::AddAC => ctl.push_actions(&[read::c, read::a, arithmetic::add, write::a]),
                Opcode::AddAD => ctl.push_actions(&[read::d, read::a, arithmetic::add, write::a]),
                Opcode::AddAE => ctl.push_actions(&[read::e, read::a, arithmetic::add, write::a]),
                Opcode::AddAH => ctl.push_actions(&[read::h, read::a, arithmetic::add, write::a]),
                Opcode::AddAL => ctl.push_actions(&[read::l, read::a, arithmetic::add, write::a]),
                Opcode::AddAHL => {
                    ctl.push_actions(&[read::hl, read::ind, read::a, arithmetic::add, write::a])
                }
                Opcode::AddA8 => {
                    ctl.push_actions(&[read::byte, read::a, arithmetic::add, write::a])
                }

                Opcode::LdBC16 => ctl.push_actions(&[read::byte, read::byte, write::bc]),
                Opcode::LdDE16 => ctl.push_actions(&[read::byte, read::byte, write::de]),
                Opcode::LdHL16 => ctl.push_actions(&[read::byte, read::byte, write::hl]),
                Opcode::LdSP16 => ctl.push_actions(&[read::byte, read::byte, write::sp]),

                Opcode::PushBc => ctl.push_actions(&[
                    utils::sleep,
                    read::bc,
                    dec::sp,
                    read::sp,
                    write::ind,
                    dec::sp,
                    read::sp,
                    write::ind,
                ]),
                Opcode::PushDe => ctl.push_actions(&[
                    utils::sleep,
                    read::de,
                    dec::sp,
                    read::sp,
                    write::ind,
                    dec::sp,
                    read::sp,
                    write::ind,
                ]),
                Opcode::PushHl => ctl.push_actions(&[
                    utils::sleep,
                    read::hl,
                    dec::sp,
                    read::sp,
                    write::ind,
                    dec::sp,
                    read::sp,
                    write::ind,
                ]),
                Opcode::PushAf => ctl.push_actions(&[
                    utils::sleep,
                    read::af,
                    dec::sp,
                    read::sp,
                    write::ind,
                    dec::sp,
                    read::sp,
                    write::ind,
                ]),

                Opcode::PopBc => ctl.push_actions(&[
                    read::sp,
                    inc::sp,
                    read::ind,
                    read::sp,
                    inc::sp,
                    read::ind,
                    write::bc,
                ]),
                Opcode::PopDe => ctl.push_actions(&[
                    read::sp,
                    inc::sp,
                    read::ind,
                    read::sp,
                    inc::sp,
                    read::ind,
                    write::de,
                ]),
                Opcode::PopHl => ctl.push_actions(&[
                    read::sp,
                    inc::sp,
                    read::ind,
                    read::sp,
                    inc::sp,
                    read::ind,
                    write::hl,
                ]),
                Opcode::PopAf => ctl.push_actions(&[
                    read::sp,
                    inc::sp,
                    read::ind,
                    read::sp,
                    inc::sp,
                    read::ind,
                    write::af,
                ]),

                Opcode::Ld16A => ctl.push_actions(&[read::a, read::byte, read::byte, write::ind]),

                Opcode::LdiHLA => ctl.push_actions(&[read::a, read::hl, write::ind, inc::hl]),
                Opcode::LdiAHL => ctl.push_actions(&[read::hl, read::ind, write::a, inc::hl]),

                Opcode::LddHLA => ctl.push_actions(&[read::a, read::hl, write::ind, dec::hl]),
                Opcode::LddAHL => ctl.push_actions(&[read::hl, read::ind, write::a, dec::hl]),

                Opcode::LdA8 => ctl.push_actions(&[read::byte, write::a]),
                Opcode::LdB8 => ctl.push_actions(&[read::byte, write::b]),
                Opcode::LdC8 => ctl.push_actions(&[read::byte, write::c]),
                Opcode::LdD8 => ctl.push_actions(&[read::byte, write::d]),
                Opcode::LdE8 => ctl.push_actions(&[read::byte, write::e]),
                Opcode::LdH8 => ctl.push_actions(&[read::byte, write::h]),
                Opcode::LdL8 => ctl.push_actions(&[read::byte, write::l]),
                Opcode::LdHL8 => ctl.push_actions(&[read::byte, read::hl, write::ind]),

                Opcode::Ld16SP => ctl.push_actions(&[
                    read::sp,
                    read::byte,
                    read::byte,
                    write::ind16,
                    utils::sleep,
                ]),
                Opcode::LdSPHL => ctl.push_actions(&[read::hl, write::sp, utils::sleep]),
                Opcode::LdA16 => ctl.push_actions(&[read::byte, read::byte, read::ind, write::a]),

                Opcode::LdAA => ctl.push_actions(&[read::a, write::a]),
                Opcode::LdAB => ctl.push_actions(&[read::b, write::a]),
                Opcode::LdAC => ctl.push_actions(&[read::c, write::a]),
                Opcode::LdAD => ctl.push_actions(&[read::d, write::a]),
                Opcode::LdAE => ctl.push_actions(&[read::e, write::a]),
                Opcode::LdAH => ctl.push_actions(&[read::h, write::a]),
                Opcode::LdAL => ctl.push_actions(&[read::l, write::a]),
                Opcode::LdAHL => ctl.push_actions(&[read::hl, read::ind, write::a]),

                Opcode::LdBA => ctl.push_actions(&[read::a, write::b]),
                Opcode::LdBB => ctl.push_actions(&[read::b, write::b]),
                Opcode::LdBC => ctl.push_actions(&[read::c, write::b]),
                Opcode::LdBD => ctl.push_actions(&[read::d, write::b]),
                Opcode::LdBE => ctl.push_actions(&[read::e, write::b]),
                Opcode::LdBH => ctl.push_actions(&[read::h, write::b]),
                Opcode::LdBL => ctl.push_actions(&[read::l, write::b]),
                Opcode::LdBHL => ctl.push_actions(&[read::hl, read::ind, write::b]),

                Opcode::LdCA => ctl.push_actions(&[read::a, write::c]),
                Opcode::LdCB => ctl.push_actions(&[read::b, write::c]),
                Opcode::LdCC => ctl.push_actions(&[read::c, write::c]),
                Opcode::LdCD => ctl.push_actions(&[read::d, write::c]),
                Opcode::LdCE => ctl.push_actions(&[read::e, write::c]),
                Opcode::LdCH => ctl.push_actions(&[read::h, write::c]),
                Opcode::LdCL => ctl.push_actions(&[read::l, write::c]),
                Opcode::LdCHL => ctl.push_actions(&[read::hl, read::ind, write::c]),

                Opcode::LdDA => ctl.push_actions(&[read::a, write::d]),
                Opcode::LdDB => ctl.push_actions(&[read::b, write::d]),
                Opcode::LdDC => ctl.push_actions(&[read::c, write::d]),
                Opcode::LdDD => ctl.push_actions(&[read::d, write::d]),
                Opcode::LdDE => ctl.push_actions(&[read::e, write::d]),
                Opcode::LdDH => ctl.push_actions(&[read::h, write::d]),
                Opcode::LdDL => ctl.push_actions(&[read::l, write::d]),
                Opcode::LdDHL => ctl.push_actions(&[read::hl, read::ind, write::d]),

                Opcode::LdEA => ctl.push_actions(&[read::a, write::e]),
                Opcode::LdEB => ctl.push_actions(&[read::b, write::e]),
                Opcode::LdEC => ctl.push_actions(&[read::c, write::e]),
                Opcode::LdED => ctl.push_actions(&[read::d, write::e]),
                Opcode::LdEE => ctl.push_actions(&[read::e, write::e]),
                Opcode::LdEH => ctl.push_actions(&[read::h, write::e]),
                Opcode::LdEL => ctl.push_actions(&[read::l, write::e]),
                Opcode::LdEHL => ctl.push_actions(&[read::hl, read::ind, write::e]),

                Opcode::LdHA => ctl.push_actions(&[read::a, write::h]),
                Opcode::LdHB => ctl.push_actions(&[read::b, write::h]),
                Opcode::LdHC => ctl.push_actions(&[read::c, write::h]),
                Opcode::LdHD => ctl.push_actions(&[read::d, write::h]),
                Opcode::LdHE => ctl.push_actions(&[read::e, write::h]),
                Opcode::LdHH => ctl.push_actions(&[read::h, write::h]),
                Opcode::LdHL => ctl.push_actions(&[read::l, write::h]),
                Opcode::LdHHL => ctl.push_actions(&[read::hl, read::ind, write::h]),

                Opcode::LdLA => ctl.push_actions(&[read::a, write::l]),
                Opcode::LdLB => ctl.push_actions(&[read::b, write::l]),
                Opcode::LdLC => ctl.push_actions(&[read::c, write::l]),
                Opcode::LdLD => ctl.push_actions(&[read::d, write::l]),
                Opcode::LdLE => ctl.push_actions(&[read::e, write::l]),
                Opcode::LdLH => ctl.push_actions(&[read::h, write::l]),
                Opcode::LdLL => ctl.push_actions(&[read::l, write::l]),
                Opcode::LdLHL => ctl.push_actions(&[read::hl, read::ind, write::l]),

                Opcode::LdHLA => ctl.push_actions(&[read::a, read::hl, write::ind]),
                Opcode::LdHLB => ctl.push_actions(&[read::b, read::hl, write::ind]),
                Opcode::LdHLC => ctl.push_actions(&[read::c, read::hl, write::ind]),
                Opcode::LdHLD => ctl.push_actions(&[read::d, read::hl, write::ind]),
                Opcode::LdHLE => ctl.push_actions(&[read::e, read::hl, write::ind]),
                Opcode::LdHLH => ctl.push_actions(&[read::h, read::hl, write::ind]),
                Opcode::LdHLL => ctl.push_actions(&[read::l, read::hl, write::ind]),

                Opcode::LdhAC => ctl.push_actions(&[read::c, read::hram, write::a]),
                Opcode::LdhA8 => ctl.push_actions(&[read::byte, read::hram, write::a]),

                Opcode::Ldh8A => ctl.push_actions(&[read::a, read::byte, write::hram]),
                Opcode::LdhCA => ctl.push_actions(&[read::a, read::c, write::hram]),

                Opcode::Call16 => ctl.push_actions(&[
                    read::byte,
                    read::byte,
                    utils::sleep,
                    dec::sp,
                    read::sp,
                    write::ind,
                    dec::sp,
                    read::sp,
                    write::ind,
                ]),

                Opcode::Nop => &mut ctl,
                Opcode::PrefixCb => ctl.push_action(fetch_cb),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
