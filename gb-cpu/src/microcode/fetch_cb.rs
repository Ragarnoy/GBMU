use crate::microcode::write;

use super::{
    bitwise, opcode_cb::OpcodeCB, read, MicrocodeController, MicrocodeFlow, State, CONTINUE,
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
            log::debug!("new cb opcode: {:?}", opcode);
            ctl.opcode = Some(opcode.into());
            match opcode {
                OpcodeCB::Bit0B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_0]),
                OpcodeCB::Bit0C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_0]),
                OpcodeCB::Bit0D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_0]),
                OpcodeCB::Bit0E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_0]),
                OpcodeCB::Bit0H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_0]),
                OpcodeCB::Bit0L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_0]),
                OpcodeCB::Bit0HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_0])
                }
                OpcodeCB::Bit0A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_0]),

                OpcodeCB::Bit1B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_1]),
                OpcodeCB::Bit1C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_1]),
                OpcodeCB::Bit1D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_1]),
                OpcodeCB::Bit1E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_1]),
                OpcodeCB::Bit1H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_1]),
                OpcodeCB::Bit1L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_1]),
                OpcodeCB::Bit1HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_1])
                }
                OpcodeCB::Bit1A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_1]),

                OpcodeCB::Bit2B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_2]),
                OpcodeCB::Bit2C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_2]),
                OpcodeCB::Bit2D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_2]),
                OpcodeCB::Bit2E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_2]),
                OpcodeCB::Bit2H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_2]),
                OpcodeCB::Bit2L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_2]),
                OpcodeCB::Bit2HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_2])
                }
                OpcodeCB::Bit2A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_2]),

                OpcodeCB::Bit3B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_3]),
                OpcodeCB::Bit3C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_3]),
                OpcodeCB::Bit3D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_3]),
                OpcodeCB::Bit3E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_3]),
                OpcodeCB::Bit3H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_3]),
                OpcodeCB::Bit3L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_3]),
                OpcodeCB::Bit3HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_3])
                }
                OpcodeCB::Bit3A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_3]),

                OpcodeCB::Bit4B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_4]),
                OpcodeCB::Bit4C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_4]),
                OpcodeCB::Bit4D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_4]),
                OpcodeCB::Bit4E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_4]),
                OpcodeCB::Bit4H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_4]),
                OpcodeCB::Bit4L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_4]),
                OpcodeCB::Bit4HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_4])
                }
                OpcodeCB::Bit4A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_4]),

                OpcodeCB::Bit5B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_5]),
                OpcodeCB::Bit5C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_5]),
                OpcodeCB::Bit5D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_5]),
                OpcodeCB::Bit5E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_5]),
                OpcodeCB::Bit5H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_5]),
                OpcodeCB::Bit5L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_5]),
                OpcodeCB::Bit5HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_5])
                }
                OpcodeCB::Bit5A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_5]),

                OpcodeCB::Bit6B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_6]),
                OpcodeCB::Bit6C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_6]),
                OpcodeCB::Bit6D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_6]),
                OpcodeCB::Bit6E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_6]),
                OpcodeCB::Bit6H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_6]),
                OpcodeCB::Bit6L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_6]),
                OpcodeCB::Bit6HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_6])
                }
                OpcodeCB::Bit6A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_6]),

                OpcodeCB::Bit7B => ctl.push_to_current_cycle(&[read::b, bitwise::bit_7]),
                OpcodeCB::Bit7C => ctl.push_to_current_cycle(&[read::c, bitwise::bit_7]),
                OpcodeCB::Bit7D => ctl.push_to_current_cycle(&[read::d, bitwise::bit_7]),
                OpcodeCB::Bit7E => ctl.push_to_current_cycle(&[read::e, bitwise::bit_7]),
                OpcodeCB::Bit7H => ctl.push_to_current_cycle(&[read::h, bitwise::bit_7]),
                OpcodeCB::Bit7L => ctl.push_to_current_cycle(&[read::l, bitwise::bit_7]),
                OpcodeCB::Bit7HL => {
                    ctl.push_to_current_cycle(&[read::hl, read::ind, bitwise::bit_7])
                }
                OpcodeCB::Bit7A => ctl.push_to_current_cycle(&[read::a, bitwise::bit_7]),

                OpcodeCB::Set0B => ctl.push_to_current_cycle(&[read::b, bitwise::set_0, write::b]),
                OpcodeCB::Set0C => ctl.push_to_current_cycle(&[read::c, bitwise::set_0, write::c]),
                OpcodeCB::Set0D => ctl.push_to_current_cycle(&[read::d, bitwise::set_0, write::d]),
                OpcodeCB::Set0E => ctl.push_to_current_cycle(&[read::e, bitwise::set_0, write::e]),
                OpcodeCB::Set0H => ctl.push_to_current_cycle(&[read::h, bitwise::set_0, write::h]),
                OpcodeCB::Set0L => ctl.push_to_current_cycle(&[read::l, bitwise::set_0, write::l]),
                OpcodeCB::Set0HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_0,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set0A => ctl.push_to_current_cycle(&[read::a, bitwise::set_0, write::a]),

                OpcodeCB::Set1B => ctl.push_to_current_cycle(&[read::b, bitwise::set_1, write::b]),
                OpcodeCB::Set1C => ctl.push_to_current_cycle(&[read::c, bitwise::set_1, write::c]),
                OpcodeCB::Set1D => ctl.push_to_current_cycle(&[read::d, bitwise::set_1, write::d]),
                OpcodeCB::Set1E => ctl.push_to_current_cycle(&[read::e, bitwise::set_1, write::e]),
                OpcodeCB::Set1H => ctl.push_to_current_cycle(&[read::h, bitwise::set_1, write::h]),
                OpcodeCB::Set1L => ctl.push_to_current_cycle(&[read::l, bitwise::set_1, write::l]),
                OpcodeCB::Set1HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_1,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set1A => ctl.push_to_current_cycle(&[read::a, bitwise::set_1, write::a]),

                OpcodeCB::Set2B => ctl.push_to_current_cycle(&[read::b, bitwise::set_2, write::b]),
                OpcodeCB::Set2C => ctl.push_to_current_cycle(&[read::c, bitwise::set_2, write::c]),
                OpcodeCB::Set2D => ctl.push_to_current_cycle(&[read::d, bitwise::set_2, write::d]),
                OpcodeCB::Set2E => ctl.push_to_current_cycle(&[read::e, bitwise::set_2, write::e]),
                OpcodeCB::Set2H => ctl.push_to_current_cycle(&[read::h, bitwise::set_2, write::h]),
                OpcodeCB::Set2L => ctl.push_to_current_cycle(&[read::l, bitwise::set_2, write::l]),
                OpcodeCB::Set2HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_2,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set2A => ctl.push_to_current_cycle(&[read::a, bitwise::set_2, write::a]),

                OpcodeCB::Set3B => ctl.push_to_current_cycle(&[read::b, bitwise::set_3, write::b]),
                OpcodeCB::Set3C => ctl.push_to_current_cycle(&[read::c, bitwise::set_3, write::c]),
                OpcodeCB::Set3D => ctl.push_to_current_cycle(&[read::d, bitwise::set_3, write::d]),
                OpcodeCB::Set3E => ctl.push_to_current_cycle(&[read::e, bitwise::set_3, write::e]),
                OpcodeCB::Set3H => ctl.push_to_current_cycle(&[read::h, bitwise::set_3, write::h]),
                OpcodeCB::Set3L => ctl.push_to_current_cycle(&[read::l, bitwise::set_3, write::l]),
                OpcodeCB::Set3HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_3,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set3A => ctl.push_to_current_cycle(&[read::a, bitwise::set_3, write::a]),

                OpcodeCB::Set4B => ctl.push_to_current_cycle(&[read::b, bitwise::set_4, write::b]),
                OpcodeCB::Set4C => ctl.push_to_current_cycle(&[read::c, bitwise::set_4, write::c]),
                OpcodeCB::Set4D => ctl.push_to_current_cycle(&[read::d, bitwise::set_4, write::d]),
                OpcodeCB::Set4E => ctl.push_to_current_cycle(&[read::e, bitwise::set_4, write::e]),
                OpcodeCB::Set4H => ctl.push_to_current_cycle(&[read::h, bitwise::set_4, write::h]),
                OpcodeCB::Set4L => ctl.push_to_current_cycle(&[read::l, bitwise::set_4, write::l]),
                OpcodeCB::Set4HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_4,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set4A => ctl.push_to_current_cycle(&[read::a, bitwise::set_4, write::a]),

                OpcodeCB::Set5B => ctl.push_to_current_cycle(&[read::b, bitwise::set_5, write::b]),
                OpcodeCB::Set5C => ctl.push_to_current_cycle(&[read::c, bitwise::set_5, write::c]),
                OpcodeCB::Set5D => ctl.push_to_current_cycle(&[read::d, bitwise::set_5, write::d]),
                OpcodeCB::Set5E => ctl.push_to_current_cycle(&[read::e, bitwise::set_5, write::e]),
                OpcodeCB::Set5H => ctl.push_to_current_cycle(&[read::h, bitwise::set_5, write::h]),
                OpcodeCB::Set5L => ctl.push_to_current_cycle(&[read::l, bitwise::set_5, write::l]),
                OpcodeCB::Set5HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_5,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set5A => ctl.push_to_current_cycle(&[read::a, bitwise::set_5, write::a]),

                OpcodeCB::Set6B => ctl.push_to_current_cycle(&[read::b, bitwise::set_6, write::b]),
                OpcodeCB::Set6C => ctl.push_to_current_cycle(&[read::c, bitwise::set_6, write::c]),
                OpcodeCB::Set6D => ctl.push_to_current_cycle(&[read::d, bitwise::set_6, write::d]),
                OpcodeCB::Set6E => ctl.push_to_current_cycle(&[read::e, bitwise::set_6, write::e]),
                OpcodeCB::Set6H => ctl.push_to_current_cycle(&[read::h, bitwise::set_6, write::h]),
                OpcodeCB::Set6L => ctl.push_to_current_cycle(&[read::l, bitwise::set_6, write::l]),
                OpcodeCB::Set6HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_6,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set6A => ctl.push_to_current_cycle(&[read::a, bitwise::set_6, write::a]),

                OpcodeCB::Set7B => ctl.push_to_current_cycle(&[read::b, bitwise::set_7, write::b]),
                OpcodeCB::Set7C => ctl.push_to_current_cycle(&[read::c, bitwise::set_7, write::c]),
                OpcodeCB::Set7D => ctl.push_to_current_cycle(&[read::d, bitwise::set_7, write::d]),
                OpcodeCB::Set7E => ctl.push_to_current_cycle(&[read::e, bitwise::set_7, write::e]),
                OpcodeCB::Set7H => ctl.push_to_current_cycle(&[read::h, bitwise::set_7, write::h]),
                OpcodeCB::Set7L => ctl.push_to_current_cycle(&[read::l, bitwise::set_7, write::l]),
                OpcodeCB::Set7HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::set_7,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Set7A => ctl.push_to_current_cycle(&[read::a, bitwise::set_7, write::a]),

                OpcodeCB::Res0B => ctl.push_to_current_cycle(&[read::b, bitwise::res_0, write::b]),
                OpcodeCB::Res0C => ctl.push_to_current_cycle(&[read::c, bitwise::res_0, write::c]),
                OpcodeCB::Res0D => ctl.push_to_current_cycle(&[read::d, bitwise::res_0, write::d]),
                OpcodeCB::Res0E => ctl.push_to_current_cycle(&[read::e, bitwise::res_0, write::e]),
                OpcodeCB::Res0H => ctl.push_to_current_cycle(&[read::h, bitwise::res_0, write::h]),
                OpcodeCB::Res0L => ctl.push_to_current_cycle(&[read::l, bitwise::res_0, write::l]),
                OpcodeCB::Res0HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_0,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res0A => ctl.push_to_current_cycle(&[read::a, bitwise::res_0, write::a]),

                OpcodeCB::Res1B => ctl.push_to_current_cycle(&[read::b, bitwise::res_1, write::b]),
                OpcodeCB::Res1C => ctl.push_to_current_cycle(&[read::c, bitwise::res_1, write::c]),
                OpcodeCB::Res1D => ctl.push_to_current_cycle(&[read::d, bitwise::res_1, write::d]),
                OpcodeCB::Res1E => ctl.push_to_current_cycle(&[read::e, bitwise::res_1, write::e]),
                OpcodeCB::Res1H => ctl.push_to_current_cycle(&[read::h, bitwise::res_1, write::h]),
                OpcodeCB::Res1L => ctl.push_to_current_cycle(&[read::l, bitwise::res_1, write::l]),
                OpcodeCB::Res1HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_1,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res1A => ctl.push_to_current_cycle(&[read::a, bitwise::res_1, write::a]),

                OpcodeCB::Res2B => ctl.push_to_current_cycle(&[read::b, bitwise::res_2, write::b]),
                OpcodeCB::Res2C => ctl.push_to_current_cycle(&[read::c, bitwise::res_2, write::c]),
                OpcodeCB::Res2D => ctl.push_to_current_cycle(&[read::d, bitwise::res_2, write::d]),
                OpcodeCB::Res2E => ctl.push_to_current_cycle(&[read::e, bitwise::res_2, write::e]),
                OpcodeCB::Res2H => ctl.push_to_current_cycle(&[read::h, bitwise::res_2, write::h]),
                OpcodeCB::Res2L => ctl.push_to_current_cycle(&[read::l, bitwise::res_2, write::l]),
                OpcodeCB::Res2HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_2,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res2A => ctl.push_to_current_cycle(&[read::a, bitwise::res_2, write::a]),

                OpcodeCB::Res3B => ctl.push_to_current_cycle(&[read::b, bitwise::res_3, write::b]),
                OpcodeCB::Res3C => ctl.push_to_current_cycle(&[read::c, bitwise::res_3, write::c]),
                OpcodeCB::Res3D => ctl.push_to_current_cycle(&[read::d, bitwise::res_3, write::d]),
                OpcodeCB::Res3E => ctl.push_to_current_cycle(&[read::e, bitwise::res_3, write::e]),
                OpcodeCB::Res3H => ctl.push_to_current_cycle(&[read::h, bitwise::res_3, write::h]),
                OpcodeCB::Res3L => ctl.push_to_current_cycle(&[read::l, bitwise::res_3, write::l]),
                OpcodeCB::Res3HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_3,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res3A => ctl.push_to_current_cycle(&[read::a, bitwise::res_3, write::a]),

                OpcodeCB::Res4B => ctl.push_to_current_cycle(&[read::b, bitwise::res_4, write::b]),
                OpcodeCB::Res4C => ctl.push_to_current_cycle(&[read::c, bitwise::res_4, write::c]),
                OpcodeCB::Res4D => ctl.push_to_current_cycle(&[read::d, bitwise::res_4, write::d]),
                OpcodeCB::Res4E => ctl.push_to_current_cycle(&[read::e, bitwise::res_4, write::e]),
                OpcodeCB::Res4H => ctl.push_to_current_cycle(&[read::h, bitwise::res_4, write::h]),
                OpcodeCB::Res4L => ctl.push_to_current_cycle(&[read::l, bitwise::res_4, write::l]),
                OpcodeCB::Res4HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_4,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res4A => ctl.push_to_current_cycle(&[read::a, bitwise::res_4, write::a]),

                OpcodeCB::Res5B => ctl.push_to_current_cycle(&[read::b, bitwise::res_5, write::b]),
                OpcodeCB::Res5C => ctl.push_to_current_cycle(&[read::c, bitwise::res_5, write::c]),
                OpcodeCB::Res5D => ctl.push_to_current_cycle(&[read::d, bitwise::res_5, write::d]),
                OpcodeCB::Res5E => ctl.push_to_current_cycle(&[read::e, bitwise::res_5, write::e]),
                OpcodeCB::Res5H => ctl.push_to_current_cycle(&[read::h, bitwise::res_5, write::h]),
                OpcodeCB::Res5L => ctl.push_to_current_cycle(&[read::l, bitwise::res_5, write::l]),
                OpcodeCB::Res5HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_5,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res5A => ctl.push_to_current_cycle(&[read::a, bitwise::res_5, write::a]),

                OpcodeCB::Res6B => ctl.push_to_current_cycle(&[read::b, bitwise::res_6, write::b]),
                OpcodeCB::Res6C => ctl.push_to_current_cycle(&[read::c, bitwise::res_6, write::c]),
                OpcodeCB::Res6D => ctl.push_to_current_cycle(&[read::d, bitwise::res_6, write::d]),
                OpcodeCB::Res6E => ctl.push_to_current_cycle(&[read::e, bitwise::res_6, write::e]),
                OpcodeCB::Res6H => ctl.push_to_current_cycle(&[read::h, bitwise::res_6, write::h]),
                OpcodeCB::Res6L => ctl.push_to_current_cycle(&[read::l, bitwise::res_6, write::l]),
                OpcodeCB::Res6HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_6,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res6A => ctl.push_to_current_cycle(&[read::a, bitwise::res_6, write::a]),

                OpcodeCB::Res7B => ctl.push_to_current_cycle(&[read::b, bitwise::res_7, write::b]),
                OpcodeCB::Res7C => ctl.push_to_current_cycle(&[read::c, bitwise::res_7, write::c]),
                OpcodeCB::Res7D => ctl.push_to_current_cycle(&[read::d, bitwise::res_7, write::d]),
                OpcodeCB::Res7E => ctl.push_to_current_cycle(&[read::e, bitwise::res_7, write::e]),
                OpcodeCB::Res7H => ctl.push_to_current_cycle(&[read::h, bitwise::res_7, write::h]),
                OpcodeCB::Res7L => ctl.push_to_current_cycle(&[read::l, bitwise::res_7, write::l]),
                OpcodeCB::Res7HL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::res_7,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::Res7A => ctl.push_to_current_cycle(&[read::a, bitwise::res_7, write::a]),

                OpcodeCB::RlB => ctl.push_to_current_cycle(&[read::b, bitwise::rl, write::b]),
                OpcodeCB::RlC => ctl.push_to_current_cycle(&[read::c, bitwise::rl, write::c]),
                OpcodeCB::RlD => ctl.push_to_current_cycle(&[read::d, bitwise::rl, write::d]),
                OpcodeCB::RlE => ctl.push_to_current_cycle(&[read::e, bitwise::rl, write::e]),
                OpcodeCB::RlH => ctl.push_to_current_cycle(&[read::h, bitwise::rl, write::h]),
                OpcodeCB::RlL => ctl.push_to_current_cycle(&[read::l, bitwise::rl, write::l]),
                OpcodeCB::RlHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::rl,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::RlA => ctl.push_to_current_cycle(&[read::a, bitwise::rl, write::a]),

                OpcodeCB::RlcB => ctl.push_to_current_cycle(&[read::b, bitwise::rlc, write::b]),
                OpcodeCB::RlcC => ctl.push_to_current_cycle(&[read::c, bitwise::rlc, write::c]),
                OpcodeCB::RlcD => ctl.push_to_current_cycle(&[read::d, bitwise::rlc, write::d]),
                OpcodeCB::RlcE => ctl.push_to_current_cycle(&[read::e, bitwise::rlc, write::e]),
                OpcodeCB::RlcH => ctl.push_to_current_cycle(&[read::h, bitwise::rlc, write::h]),
                OpcodeCB::RlcL => ctl.push_to_current_cycle(&[read::l, bitwise::rlc, write::l]),
                OpcodeCB::RlcHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::rlc,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::RlcA => ctl.push_to_current_cycle(&[read::a, bitwise::rlc, write::a]),

                OpcodeCB::RrcB => ctl.push_to_current_cycle(&[read::b, bitwise::rrc, write::b]),
                OpcodeCB::RrcC => ctl.push_to_current_cycle(&[read::c, bitwise::rrc, write::c]),
                OpcodeCB::RrcD => ctl.push_to_current_cycle(&[read::d, bitwise::rrc, write::d]),
                OpcodeCB::RrcE => ctl.push_to_current_cycle(&[read::e, bitwise::rrc, write::e]),
                OpcodeCB::RrcH => ctl.push_to_current_cycle(&[read::h, bitwise::rrc, write::h]),
                OpcodeCB::RrcL => ctl.push_to_current_cycle(&[read::l, bitwise::rrc, write::l]),
                OpcodeCB::RrcHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::rrc,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::RrcA => ctl.push_to_current_cycle(&[read::a, bitwise::rrc, write::a]),

                OpcodeCB::RrB => ctl.push_to_current_cycle(&[read::b, bitwise::rr, write::b]),
                OpcodeCB::RrC => ctl.push_to_current_cycle(&[read::c, bitwise::rr, write::c]),
                OpcodeCB::RrD => ctl.push_to_current_cycle(&[read::d, bitwise::rr, write::d]),
                OpcodeCB::RrE => ctl.push_to_current_cycle(&[read::e, bitwise::rr, write::e]),
                OpcodeCB::RrH => ctl.push_to_current_cycle(&[read::h, bitwise::rr, write::h]),
                OpcodeCB::RrL => ctl.push_to_current_cycle(&[read::l, bitwise::rr, write::l]),
                OpcodeCB::RrHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::rr,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::RrA => ctl.push_to_current_cycle(&[read::a, bitwise::rr, write::a]),

                OpcodeCB::SrlB => ctl.push_to_current_cycle(&[read::b, bitwise::srl, write::b]),
                OpcodeCB::SrlC => ctl.push_to_current_cycle(&[read::c, bitwise::srl, write::c]),
                OpcodeCB::SrlD => ctl.push_to_current_cycle(&[read::d, bitwise::srl, write::d]),
                OpcodeCB::SrlE => ctl.push_to_current_cycle(&[read::e, bitwise::srl, write::e]),
                OpcodeCB::SrlH => ctl.push_to_current_cycle(&[read::h, bitwise::srl, write::h]),
                OpcodeCB::SrlL => ctl.push_to_current_cycle(&[read::l, bitwise::srl, write::l]),
                OpcodeCB::SrlHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::srl,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::SrlA => ctl.push_to_current_cycle(&[read::a, bitwise::srl, write::a]),

                OpcodeCB::SlaB => ctl.push_to_current_cycle(&[read::b, bitwise::sla, write::b]),
                OpcodeCB::SlaC => ctl.push_to_current_cycle(&[read::c, bitwise::sla, write::c]),
                OpcodeCB::SlaD => ctl.push_to_current_cycle(&[read::d, bitwise::sla, write::d]),
                OpcodeCB::SlaE => ctl.push_to_current_cycle(&[read::e, bitwise::sla, write::e]),
                OpcodeCB::SlaH => ctl.push_to_current_cycle(&[read::h, bitwise::sla, write::h]),
                OpcodeCB::SlaL => ctl.push_to_current_cycle(&[read::l, bitwise::sla, write::l]),
                OpcodeCB::SlaHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::sla,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::SlaA => ctl.push_to_current_cycle(&[read::a, bitwise::sla, write::a]),

                OpcodeCB::SraB => ctl.push_to_current_cycle(&[read::b, bitwise::sra, write::b]),
                OpcodeCB::SraC => ctl.push_to_current_cycle(&[read::c, bitwise::sra, write::c]),
                OpcodeCB::SraD => ctl.push_to_current_cycle(&[read::d, bitwise::sra, write::d]),
                OpcodeCB::SraE => ctl.push_to_current_cycle(&[read::e, bitwise::sra, write::e]),
                OpcodeCB::SraH => ctl.push_to_current_cycle(&[read::h, bitwise::sra, write::h]),
                OpcodeCB::SraL => ctl.push_to_current_cycle(&[read::l, bitwise::sra, write::l]),
                OpcodeCB::SraHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::sra,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::SraA => ctl.push_to_current_cycle(&[read::a, bitwise::sra, write::a]),

                OpcodeCB::SwapB => ctl.push_to_current_cycle(&[read::b, bitwise::swap, write::b]),
                OpcodeCB::SwapC => ctl.push_to_current_cycle(&[read::c, bitwise::swap, write::c]),
                OpcodeCB::SwapD => ctl.push_to_current_cycle(&[read::d, bitwise::swap, write::d]),
                OpcodeCB::SwapE => ctl.push_to_current_cycle(&[read::e, bitwise::swap, write::e]),
                OpcodeCB::SwapH => ctl.push_to_current_cycle(&[read::h, bitwise::swap, write::h]),
                OpcodeCB::SwapL => ctl.push_to_current_cycle(&[read::l, bitwise::swap, write::l]),
                OpcodeCB::SwapHL => ctl.push_to_current_cycle(&[
                    read::hl,
                    read::ind,
                    bitwise::swap,
                    read::hl,
                    write::ind,
                ]),
                OpcodeCB::SwapA => ctl.push_to_current_cycle(&[read::a, bitwise::swap, write::a]),
            };
            CONTINUE
        },
    )
}
