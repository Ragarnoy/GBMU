use super::{
    condition::{carry, not_carry, not_zero, zero},
    fetch_cb::fetch_cb,
    jump,
    opcode::Opcode,
    read::read,
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

                Opcode::Nop => {}

                Opcode::PrefixCb => ctl.push_action(fetch_cb),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
