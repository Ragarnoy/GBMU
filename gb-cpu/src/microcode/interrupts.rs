use super::{
    controller::OpcodeType, dec, jump::jump, opcode::Opcode, read, utils::sleep, write,
    MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION,
};

pub fn is_interrupt_ready(ctl: &mut MicrocodeController) -> bool {
    if !ctl.interrupt_master_enable {
        return false;
    }
    let interrupt_flag = ctl.interrupt_flag;
    let interrupt_enable = ctl.interrupt_enable;
    interrupt_flag & interrupt_enable != 0
}

pub fn handle_interrupts(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    let interrupt_flag = ctl.interrupt_flag;
    let interrupt_enable = ctl.interrupt_enable;
    let mut interrupt_match = interrupt_flag & interrupt_enable;

    // Shift to right until first bit is set
    let mut source_bit: u8 = 0;
    while interrupt_match & 0x1 == 0 {
        interrupt_match >>= 1;
        source_bit += 1;
    }
    if source_bit > 4 {
        panic!("Trying to access an interrupt source that doesn't exist.")
    }
    // Reset IME to avoid any new interrupt while processing current one
    ctl.interrupt_master_enable = false;

    // Reset bit from source in interrupt flag
    let bit_to_res = 1_u8 << source_bit;
    ctl.interrupt_flag &= !bit_to_res;

    // Push interrupt source address to cache
    ctl.push_u16(0x0040 | ((source_bit as u16) << 3));

    ctl.push_actions(&[
        // Sleep (2 mcycles)
        sleep,
        sleep,
        // Store pc into stack (2 mcycles)
        read::pc,
        dec::sp,
        read::sp,
        write::ind,
        dec::sp,
        read::sp,
        write::ind,
        // Jump to interrupt source address (1 mcycle)
        jump,
    ]);
    OK_PLAY_NEXT_ACTION
}

pub fn toggle_ime(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    ctl.interrupt_master_enable = match ctl.opcode {
        Some(OpcodeType::Unprefixed(Opcode::Di)) => false,
        Some(OpcodeType::Unprefixed(Opcode::Ei)) => true,
        Some(OpcodeType::Unprefixed(Opcode::Reti)) => true,
        _ => panic!("toggle_ime action should not be used during this instruction."),
    };
    OK_PLAY_NEXT_ACTION
}
