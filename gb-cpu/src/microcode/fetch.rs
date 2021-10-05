use super::{
    condition::{carry, not_carry, not_zero, zero},
    dec,
    fetch_cb::fetch_cb,
    inc, jump, logic,
    opcode::Opcode,
    read::{self, read},
    write, CycleDigest, MicrocodeController, MicrocodeFlow, State,
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
                Opcode::CpAA => ctl.push_actions(&[read::a, logic::cp, write::a]),
                Opcode::CpAB => ctl.push_actions(&[read::b, logic::cp, write::b]),
                Opcode::CpAC => ctl.push_actions(&[read::c, logic::cp, write::c]),
                Opcode::CpAD => ctl.push_actions(&[read::d, logic::cp, write::d]),
                Opcode::CpAE => ctl.push_actions(&[read::e, logic::cp, write::e]),
                Opcode::CpAH => ctl.push_actions(&[read::h, logic::cp, write::h]),
                Opcode::CpAL => ctl.push_actions(&[read::l, logic::cp, write::l]),
                Opcode::CpAHL => ctl.push_actions(&[read::hl, read::ind, logic::cp]),
                Opcode::CpA8 => ctl.push_actions(&[read, logic::cp]),

                Opcode::Nop => &mut ctl,
                Opcode::PrefixCb => ctl.push_action(fetch_cb),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
