use crate::microcode::write;

use super::{
    bitwise, opcode_cb::OpcodeCB, read, CycleDigest, MicrocodeController, MicrocodeFlow, State,
};
use std::convert::TryFrom;

pub fn fetch_cb(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = state.read();
    OpcodeCB::try_from(byte).map_or_else(
        |e| {
            panic!(
                "how it's possible for an u8({}) to be outside the range of 0..ff: {}",
                byte, e
            );
        },
        |opcode| {
            ctl.opcode = Some(opcode.into());
            match opcode {
                OpcodeCB::Bit0B => ctl.push_actions(&[read::b, bitwise::bit_0]),
                OpcodeCB::Bit0C => ctl.push_actions(&[read::c, bitwise::bit_0]),
                OpcodeCB::Bit0D => ctl.push_actions(&[read::d, bitwise::bit_0]),
                OpcodeCB::Bit0E => ctl.push_actions(&[read::e, bitwise::bit_0]),
                OpcodeCB::Bit0H => ctl.push_actions(&[read::h, bitwise::bit_0]),
                OpcodeCB::Bit0L => ctl.push_actions(&[read::l, bitwise::bit_0]),
                OpcodeCB::Bit0HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_0]),
                OpcodeCB::Bit0A => ctl.push_actions(&[read::a, bitwise::bit_0]),

                OpcodeCB::Bit1B => ctl.push_actions(&[read::b, bitwise::bit_1]),
                OpcodeCB::Bit1C => ctl.push_actions(&[read::c, bitwise::bit_1]),
                OpcodeCB::Bit1D => ctl.push_actions(&[read::d, bitwise::bit_1]),
                OpcodeCB::Bit1E => ctl.push_actions(&[read::e, bitwise::bit_1]),
                OpcodeCB::Bit1H => ctl.push_actions(&[read::h, bitwise::bit_1]),
                OpcodeCB::Bit1L => ctl.push_actions(&[read::l, bitwise::bit_1]),
                OpcodeCB::Bit1HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_1]),
                OpcodeCB::Bit1A => ctl.push_actions(&[read::a, bitwise::bit_1]),

                OpcodeCB::Bit2B => ctl.push_actions(&[read::b, bitwise::bit_2]),
                OpcodeCB::Bit2C => ctl.push_actions(&[read::c, bitwise::bit_2]),
                OpcodeCB::Bit2D => ctl.push_actions(&[read::d, bitwise::bit_2]),
                OpcodeCB::Bit2E => ctl.push_actions(&[read::e, bitwise::bit_2]),
                OpcodeCB::Bit2H => ctl.push_actions(&[read::h, bitwise::bit_2]),
                OpcodeCB::Bit2L => ctl.push_actions(&[read::l, bitwise::bit_2]),
                OpcodeCB::Bit2HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_2]),
                OpcodeCB::Bit2A => ctl.push_actions(&[read::a, bitwise::bit_2]),

                OpcodeCB::Bit3B => ctl.push_actions(&[read::b, bitwise::bit_3]),
                OpcodeCB::Bit3C => ctl.push_actions(&[read::c, bitwise::bit_3]),
                OpcodeCB::Bit3D => ctl.push_actions(&[read::d, bitwise::bit_3]),
                OpcodeCB::Bit3E => ctl.push_actions(&[read::e, bitwise::bit_3]),
                OpcodeCB::Bit3H => ctl.push_actions(&[read::h, bitwise::bit_3]),
                OpcodeCB::Bit3L => ctl.push_actions(&[read::l, bitwise::bit_3]),
                OpcodeCB::Bit3HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_3]),
                OpcodeCB::Bit3A => ctl.push_actions(&[read::a, bitwise::bit_3]),

                OpcodeCB::Bit4B => ctl.push_actions(&[read::b, bitwise::bit_4]),
                OpcodeCB::Bit4C => ctl.push_actions(&[read::c, bitwise::bit_4]),
                OpcodeCB::Bit4D => ctl.push_actions(&[read::d, bitwise::bit_4]),
                OpcodeCB::Bit4E => ctl.push_actions(&[read::e, bitwise::bit_4]),
                OpcodeCB::Bit4H => ctl.push_actions(&[read::h, bitwise::bit_4]),
                OpcodeCB::Bit4L => ctl.push_actions(&[read::l, bitwise::bit_4]),
                OpcodeCB::Bit4HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_4]),
                OpcodeCB::Bit4A => ctl.push_actions(&[read::a, bitwise::bit_4]),

                OpcodeCB::Bit5B => ctl.push_actions(&[read::b, bitwise::bit_5]),
                OpcodeCB::Bit5C => ctl.push_actions(&[read::c, bitwise::bit_5]),
                OpcodeCB::Bit5D => ctl.push_actions(&[read::d, bitwise::bit_5]),
                OpcodeCB::Bit5E => ctl.push_actions(&[read::e, bitwise::bit_5]),
                OpcodeCB::Bit5H => ctl.push_actions(&[read::h, bitwise::bit_5]),
                OpcodeCB::Bit5L => ctl.push_actions(&[read::l, bitwise::bit_5]),
                OpcodeCB::Bit5HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_5]),
                OpcodeCB::Bit5A => ctl.push_actions(&[read::a, bitwise::bit_5]),

                OpcodeCB::Bit6B => ctl.push_actions(&[read::b, bitwise::bit_6]),
                OpcodeCB::Bit6C => ctl.push_actions(&[read::c, bitwise::bit_6]),
                OpcodeCB::Bit6D => ctl.push_actions(&[read::d, bitwise::bit_6]),
                OpcodeCB::Bit6E => ctl.push_actions(&[read::e, bitwise::bit_6]),
                OpcodeCB::Bit6H => ctl.push_actions(&[read::h, bitwise::bit_6]),
                OpcodeCB::Bit6L => ctl.push_actions(&[read::l, bitwise::bit_6]),
                OpcodeCB::Bit6HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_6]),
                OpcodeCB::Bit6A => ctl.push_actions(&[read::a, bitwise::bit_6]),

                OpcodeCB::Bit7B => ctl.push_actions(&[read::b, bitwise::bit_7]),
                OpcodeCB::Bit7C => ctl.push_actions(&[read::c, bitwise::bit_7]),
                OpcodeCB::Bit7D => ctl.push_actions(&[read::d, bitwise::bit_7]),
                OpcodeCB::Bit7E => ctl.push_actions(&[read::e, bitwise::bit_7]),
                OpcodeCB::Bit7H => ctl.push_actions(&[read::h, bitwise::bit_7]),
                OpcodeCB::Bit7L => ctl.push_actions(&[read::l, bitwise::bit_7]),
                OpcodeCB::Bit7HL => ctl.push_actions(&[read::hl, read::ind, bitwise::bit_7]),
                OpcodeCB::Bit7A => ctl.push_actions(&[read::a, bitwise::bit_7]),

                OpcodeCB::Set0B => ctl.push_actions(&[read::b, bitwise::set_0]),
                OpcodeCB::Set0C => ctl.push_actions(&[read::c, bitwise::set_0]),
                OpcodeCB::Set0D => ctl.push_actions(&[read::d, bitwise::set_0]),
                OpcodeCB::Set0E => ctl.push_actions(&[read::e, bitwise::set_0]),
                OpcodeCB::Set0H => ctl.push_actions(&[read::h, bitwise::set_0]),
                OpcodeCB::Set0L => ctl.push_actions(&[read::l, bitwise::set_0]),
                OpcodeCB::Set0HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_0]),
                OpcodeCB::Set0A => ctl.push_actions(&[read::a, bitwise::set_0]),

                OpcodeCB::Set1B => ctl.push_actions(&[read::b, bitwise::set_1]),
                OpcodeCB::Set1C => ctl.push_actions(&[read::c, bitwise::set_1]),
                OpcodeCB::Set1D => ctl.push_actions(&[read::d, bitwise::set_1]),
                OpcodeCB::Set1E => ctl.push_actions(&[read::e, bitwise::set_1]),
                OpcodeCB::Set1H => ctl.push_actions(&[read::h, bitwise::set_1]),
                OpcodeCB::Set1L => ctl.push_actions(&[read::l, bitwise::set_1]),
                OpcodeCB::Set1HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_1]),
                OpcodeCB::Set1A => ctl.push_actions(&[read::a, bitwise::set_1]),

                OpcodeCB::Set2B => ctl.push_actions(&[read::b, bitwise::set_2]),
                OpcodeCB::Set2C => ctl.push_actions(&[read::c, bitwise::set_2]),
                OpcodeCB::Set2D => ctl.push_actions(&[read::d, bitwise::set_2]),
                OpcodeCB::Set2E => ctl.push_actions(&[read::e, bitwise::set_2]),
                OpcodeCB::Set2H => ctl.push_actions(&[read::h, bitwise::set_2]),
                OpcodeCB::Set2L => ctl.push_actions(&[read::l, bitwise::set_2]),
                OpcodeCB::Set2HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_2]),
                OpcodeCB::Set2A => ctl.push_actions(&[read::a, bitwise::set_2]),

                OpcodeCB::Set3B => ctl.push_actions(&[read::b, bitwise::set_3]),
                OpcodeCB::Set3C => ctl.push_actions(&[read::c, bitwise::set_3]),
                OpcodeCB::Set3D => ctl.push_actions(&[read::d, bitwise::set_3]),
                OpcodeCB::Set3E => ctl.push_actions(&[read::e, bitwise::set_3]),
                OpcodeCB::Set3H => ctl.push_actions(&[read::h, bitwise::set_3]),
                OpcodeCB::Set3L => ctl.push_actions(&[read::l, bitwise::set_3]),
                OpcodeCB::Set3HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_3]),
                OpcodeCB::Set3A => ctl.push_actions(&[read::a, bitwise::set_3]),

                OpcodeCB::Set4B => ctl.push_actions(&[read::b, bitwise::set_4]),
                OpcodeCB::Set4C => ctl.push_actions(&[read::c, bitwise::set_4]),
                OpcodeCB::Set4D => ctl.push_actions(&[read::d, bitwise::set_4]),
                OpcodeCB::Set4E => ctl.push_actions(&[read::e, bitwise::set_4]),
                OpcodeCB::Set4H => ctl.push_actions(&[read::h, bitwise::set_4]),
                OpcodeCB::Set4L => ctl.push_actions(&[read::l, bitwise::set_4]),
                OpcodeCB::Set4HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_4]),
                OpcodeCB::Set4A => ctl.push_actions(&[read::a, bitwise::set_4]),

                OpcodeCB::Set5B => ctl.push_actions(&[read::b, bitwise::set_5]),
                OpcodeCB::Set5C => ctl.push_actions(&[read::c, bitwise::set_5]),
                OpcodeCB::Set5D => ctl.push_actions(&[read::d, bitwise::set_5]),
                OpcodeCB::Set5E => ctl.push_actions(&[read::e, bitwise::set_5]),
                OpcodeCB::Set5H => ctl.push_actions(&[read::h, bitwise::set_5]),
                OpcodeCB::Set5L => ctl.push_actions(&[read::l, bitwise::set_5]),
                OpcodeCB::Set5HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_5]),
                OpcodeCB::Set5A => ctl.push_actions(&[read::a, bitwise::set_5]),

                OpcodeCB::Set6B => ctl.push_actions(&[read::b, bitwise::set_6]),
                OpcodeCB::Set6C => ctl.push_actions(&[read::c, bitwise::set_6]),
                OpcodeCB::Set6D => ctl.push_actions(&[read::d, bitwise::set_6]),
                OpcodeCB::Set6E => ctl.push_actions(&[read::e, bitwise::set_6]),
                OpcodeCB::Set6H => ctl.push_actions(&[read::h, bitwise::set_6]),
                OpcodeCB::Set6L => ctl.push_actions(&[read::l, bitwise::set_6]),
                OpcodeCB::Set6HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_6]),
                OpcodeCB::Set6A => ctl.push_actions(&[read::a, bitwise::set_6]),

                OpcodeCB::Set7B => ctl.push_actions(&[read::b, bitwise::set_7]),
                OpcodeCB::Set7C => ctl.push_actions(&[read::c, bitwise::set_7]),
                OpcodeCB::Set7D => ctl.push_actions(&[read::d, bitwise::set_7]),
                OpcodeCB::Set7E => ctl.push_actions(&[read::e, bitwise::set_7]),
                OpcodeCB::Set7H => ctl.push_actions(&[read::h, bitwise::set_7]),
                OpcodeCB::Set7L => ctl.push_actions(&[read::l, bitwise::set_7]),
                OpcodeCB::Set7HL => ctl.push_actions(&[read::hl, read::ind, bitwise::set_7]),
                OpcodeCB::Set7A => ctl.push_actions(&[read::a, bitwise::set_7]),

                OpcodeCB::RlB => ctl.push_actions(&[read::b, bitwise::rl, write::b]),
                OpcodeCB::RlC => ctl.push_actions(&[read::c, bitwise::rl, write::c]),
                OpcodeCB::RlD => ctl.push_actions(&[read::d, bitwise::rl, write::d]),
                OpcodeCB::RlE => ctl.push_actions(&[read::e, bitwise::rl, write::e]),
                OpcodeCB::RlH => ctl.push_actions(&[read::h, bitwise::rl, write::h]),
                OpcodeCB::RlL => ctl.push_actions(&[read::l, bitwise::rl, write::l]),
                OpcodeCB::RlHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::rl, read::hl, write::ind])
                }
                OpcodeCB::RlA => ctl.push_actions(&[read::a, bitwise::rl, write::a]),

                OpcodeCB::RlcB => ctl.push_actions(&[read::b, bitwise::rlc, write::b]),
                OpcodeCB::RlcC => ctl.push_actions(&[read::c, bitwise::rlc, write::c]),
                OpcodeCB::RlcD => ctl.push_actions(&[read::d, bitwise::rlc, write::d]),
                OpcodeCB::RlcE => ctl.push_actions(&[read::e, bitwise::rlc, write::e]),
                OpcodeCB::RlcH => ctl.push_actions(&[read::h, bitwise::rlc, write::h]),
                OpcodeCB::RlcL => ctl.push_actions(&[read::l, bitwise::rlc, write::l]),
                OpcodeCB::RlcHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::rlc, read::hl, write::ind])
                }
                OpcodeCB::RlcA => ctl.push_actions(&[read::a, bitwise::rlc, write::a]),

                OpcodeCB::RrcB => ctl.push_actions(&[read::b, bitwise::rrc, write::b]),
                OpcodeCB::RrcC => ctl.push_actions(&[read::c, bitwise::rrc, write::c]),
                OpcodeCB::RrcD => ctl.push_actions(&[read::d, bitwise::rrc, write::d]),
                OpcodeCB::RrcE => ctl.push_actions(&[read::e, bitwise::rrc, write::e]),
                OpcodeCB::RrcH => ctl.push_actions(&[read::h, bitwise::rrc, write::h]),
                OpcodeCB::RrcL => ctl.push_actions(&[read::l, bitwise::rrc, write::l]),
                OpcodeCB::RrcHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::rrc, read::hl, write::ind])
                }
                OpcodeCB::RrcA => ctl.push_actions(&[read::a, bitwise::rrc, write::a]),

                OpcodeCB::RrB => ctl.push_actions(&[read::b, bitwise::rr, write::b]),
                OpcodeCB::RrC => ctl.push_actions(&[read::c, bitwise::rr, write::c]),
                OpcodeCB::RrD => ctl.push_actions(&[read::d, bitwise::rr, write::d]),
                OpcodeCB::RrE => ctl.push_actions(&[read::e, bitwise::rr, write::e]),
                OpcodeCB::RrH => ctl.push_actions(&[read::h, bitwise::rr, write::h]),
                OpcodeCB::RrL => ctl.push_actions(&[read::l, bitwise::rr, write::l]),
                OpcodeCB::RrHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::rr, read::hl, write::ind])
                }
                OpcodeCB::RrA => ctl.push_actions(&[read::a, bitwise::rr, write::a]),

                OpcodeCB::SrlB => ctl.push_actions(&[read::b, bitwise::srl, write::b]),
                OpcodeCB::SrlC => ctl.push_actions(&[read::c, bitwise::srl, write::c]),
                OpcodeCB::SrlD => ctl.push_actions(&[read::d, bitwise::srl, write::d]),
                OpcodeCB::SrlE => ctl.push_actions(&[read::e, bitwise::srl, write::e]),
                OpcodeCB::SrlH => ctl.push_actions(&[read::h, bitwise::srl, write::h]),
                OpcodeCB::SrlL => ctl.push_actions(&[read::l, bitwise::srl, write::l]),
                OpcodeCB::SrlHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::srl, read::hl, write::ind])
                }
                OpcodeCB::SrlA => ctl.push_actions(&[read::a, bitwise::srl, write::a]),

                OpcodeCB::SlaB => ctl.push_actions(&[read::b, bitwise::sla, write::b]),
                OpcodeCB::SlaC => ctl.push_actions(&[read::c, bitwise::sla, write::c]),
                OpcodeCB::SlaD => ctl.push_actions(&[read::d, bitwise::sla, write::d]),
                OpcodeCB::SlaE => ctl.push_actions(&[read::e, bitwise::sla, write::e]),
                OpcodeCB::SlaH => ctl.push_actions(&[read::h, bitwise::sla, write::h]),
                OpcodeCB::SlaL => ctl.push_actions(&[read::l, bitwise::sla, write::l]),
                OpcodeCB::SlaHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::sla, read::hl, write::ind])
                }
                OpcodeCB::SlaA => ctl.push_actions(&[read::a, bitwise::sla, write::a]),

                OpcodeCB::SraB => ctl.push_actions(&[read::b, bitwise::sra, write::b]),
                OpcodeCB::SraC => ctl.push_actions(&[read::c, bitwise::sra, write::c]),
                OpcodeCB::SraD => ctl.push_actions(&[read::d, bitwise::sra, write::d]),
                OpcodeCB::SraE => ctl.push_actions(&[read::e, bitwise::sra, write::e]),
                OpcodeCB::SraH => ctl.push_actions(&[read::h, bitwise::sra, write::h]),
                OpcodeCB::SraL => ctl.push_actions(&[read::l, bitwise::sra, write::l]),
                OpcodeCB::SraHL => {
                    ctl.push_actions(&[read::hl, read::ind, bitwise::sra, read::hl, write::ind])
                }
                OpcodeCB::SraA => ctl.push_actions(&[read::a, bitwise::sra, write::a]),

                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
