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
                OpcodeCB::Bit0HL => ctl.push_actions(&[read::hl, bitwise::bit_0]),
                OpcodeCB::Bit0A => ctl.push_actions(&[read::a, bitwise::bit_0]),

                OpcodeCB::Bit1B => ctl.push_actions(&[read::b, bitwise::bit_1]),
                OpcodeCB::Bit1C => ctl.push_actions(&[read::c, bitwise::bit_1]),
                OpcodeCB::Bit1D => ctl.push_actions(&[read::d, bitwise::bit_1]),
                OpcodeCB::Bit1E => ctl.push_actions(&[read::e, bitwise::bit_1]),
                OpcodeCB::Bit1H => ctl.push_actions(&[read::h, bitwise::bit_1]),
                OpcodeCB::Bit1L => ctl.push_actions(&[read::l, bitwise::bit_1]),
                OpcodeCB::Bit1HL => ctl.push_actions(&[read::hl, bitwise::bit_1]),
                OpcodeCB::Bit1A => ctl.push_actions(&[read::a, bitwise::bit_1]),

                OpcodeCB::Bit2B => ctl.push_actions(&[read::b, bitwise::bit_2]),
                OpcodeCB::Bit2C => ctl.push_actions(&[read::c, bitwise::bit_2]),
                OpcodeCB::Bit2D => ctl.push_actions(&[read::d, bitwise::bit_2]),
                OpcodeCB::Bit2E => ctl.push_actions(&[read::e, bitwise::bit_2]),
                OpcodeCB::Bit2H => ctl.push_actions(&[read::h, bitwise::bit_2]),
                OpcodeCB::Bit2L => ctl.push_actions(&[read::l, bitwise::bit_2]),
                OpcodeCB::Bit2HL => ctl.push_actions(&[read::hl, bitwise::bit_2]),
                OpcodeCB::Bit2A => ctl.push_actions(&[read::a, bitwise::bit_2]),

                OpcodeCB::Bit3B => ctl.push_actions(&[read::b, bitwise::bit_3]),
                OpcodeCB::Bit3C => ctl.push_actions(&[read::c, bitwise::bit_3]),
                OpcodeCB::Bit3D => ctl.push_actions(&[read::d, bitwise::bit_3]),
                OpcodeCB::Bit3E => ctl.push_actions(&[read::e, bitwise::bit_3]),
                OpcodeCB::Bit3H => ctl.push_actions(&[read::h, bitwise::bit_3]),
                OpcodeCB::Bit3L => ctl.push_actions(&[read::l, bitwise::bit_3]),
                OpcodeCB::Bit3HL => ctl.push_actions(&[read::hl, bitwise::bit_3]),
                OpcodeCB::Bit3A => ctl.push_actions(&[read::a, bitwise::bit_3]),

                OpcodeCB::Bit4B => ctl.push_actions(&[read::b, bitwise::bit_4]),
                OpcodeCB::Bit4C => ctl.push_actions(&[read::c, bitwise::bit_4]),
                OpcodeCB::Bit4D => ctl.push_actions(&[read::d, bitwise::bit_4]),
                OpcodeCB::Bit4E => ctl.push_actions(&[read::e, bitwise::bit_4]),
                OpcodeCB::Bit4H => ctl.push_actions(&[read::h, bitwise::bit_4]),
                OpcodeCB::Bit4L => ctl.push_actions(&[read::l, bitwise::bit_4]),
                OpcodeCB::Bit4HL => ctl.push_actions(&[read::hl, bitwise::bit_4]),
                OpcodeCB::Bit4A => ctl.push_actions(&[read::a, bitwise::bit_4]),

                OpcodeCB::Bit5B => ctl.push_actions(&[read::b, bitwise::bit_5]),
                OpcodeCB::Bit5C => ctl.push_actions(&[read::c, bitwise::bit_5]),
                OpcodeCB::Bit5D => ctl.push_actions(&[read::d, bitwise::bit_5]),
                OpcodeCB::Bit5E => ctl.push_actions(&[read::e, bitwise::bit_5]),
                OpcodeCB::Bit5H => ctl.push_actions(&[read::h, bitwise::bit_5]),
                OpcodeCB::Bit5L => ctl.push_actions(&[read::l, bitwise::bit_5]),
                OpcodeCB::Bit5HL => ctl.push_actions(&[read::hl, bitwise::bit_5]),
                OpcodeCB::Bit5A => ctl.push_actions(&[read::a, bitwise::bit_5]),

                OpcodeCB::Bit6B => ctl.push_actions(&[read::b, bitwise::bit_6]),
                OpcodeCB::Bit6C => ctl.push_actions(&[read::c, bitwise::bit_6]),
                OpcodeCB::Bit6D => ctl.push_actions(&[read::d, bitwise::bit_6]),
                OpcodeCB::Bit6E => ctl.push_actions(&[read::e, bitwise::bit_6]),
                OpcodeCB::Bit6H => ctl.push_actions(&[read::h, bitwise::bit_6]),
                OpcodeCB::Bit6L => ctl.push_actions(&[read::l, bitwise::bit_6]),
                OpcodeCB::Bit6HL => ctl.push_actions(&[read::hl, bitwise::bit_6]),
                OpcodeCB::Bit6A => ctl.push_actions(&[read::a, bitwise::bit_6]),

                OpcodeCB::Bit7B => ctl.push_actions(&[read::b, bitwise::bit_7]),
                OpcodeCB::Bit7C => ctl.push_actions(&[read::c, bitwise::bit_7]),
                OpcodeCB::Bit7D => ctl.push_actions(&[read::d, bitwise::bit_7]),
                OpcodeCB::Bit7E => ctl.push_actions(&[read::e, bitwise::bit_7]),
                OpcodeCB::Bit7H => ctl.push_actions(&[read::h, bitwise::bit_7]),
                OpcodeCB::Bit7L => ctl.push_actions(&[read::l, bitwise::bit_7]),
                OpcodeCB::Bit7HL => ctl.push_actions(&[read::hl, bitwise::bit_7]),
                OpcodeCB::Bit7A => ctl.push_actions(&[read::a, bitwise::bit_7]),
                _ => todo!("unimplemented opcode {:?}", opcode),
            };
            MicrocodeFlow::Continue(CycleDigest::Consume)
        },
    )
}
