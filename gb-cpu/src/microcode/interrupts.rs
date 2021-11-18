use super::{
    dec, jump::jump, read, utils::sleep, write, MicrocodeController, MicrocodeFlow, State,
    OK_PLAY_NEXT_ACTION,
};

pub fn handle_interrupts(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut int_flags = state.int_flags.borrow_mut();
    let interrupt_flag = int_flags.flag;
    let interrupt_enable = int_flags.enable_mask;
    let mut interrupt_match = interrupt_flag & interrupt_enable;

    // Shift to right until first bit is set
    let mut source_bit: u8 = 0;
    assert_ne!(
        interrupt_match, 0,
        "Zero value would provoke an infinite loop."
    );
    while interrupt_match & 0x1 == 0 {
        interrupt_match >>= 1;
        source_bit += 1;
    }
    if source_bit > 4 {
        return OK_PLAY_NEXT_ACTION;
    }
    // Reset IME to avoid any new interrupt while processing current one
    int_flags.master_enable = false;

    // Reset bit from source in interrupt flag
    let bit_to_res = 1_u8 << source_bit;
    int_flags.flag ^= bit_to_res;

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

pub fn disable_ime(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.int_flags.borrow_mut().master_enable = false;
    OK_PLAY_NEXT_ACTION
}

pub fn enable_ime(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.int_flags.borrow_mut().master_enable = true;
    OK_PLAY_NEXT_ACTION
}
