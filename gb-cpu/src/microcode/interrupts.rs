use super::{
    controller::Mode, dec, jump::jump, read, write, MicrocodeController, MicrocodeFlow, State,
    CONTINUE,
};

pub fn handle_interrupts(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let mut int_flags = state.int_flags.borrow_mut();
    let interrupt_flag = int_flags.flag;
    let interrupt_enable = int_flags.enable_mask;
    let interrupt_match = interrupt_flag & interrupt_enable;

    // Shift to right until first bit is set
    let source_bit = interrupt_match.trailing_zeros();
    assert_ne!(
        interrupt_match, 0,
        "Zero value would provoke an infinite loop."
    );
    if source_bit > 4 {
        return CONTINUE;
    }
    // Reset IME to avoid any new interrupt while processing current one
    int_flags.master_enable = false;

    // Reset bit from source in interrupt flag
    let bit_to_res = 1_u8 << source_bit;
    int_flags.flag ^= bit_to_res;

    // Push interrupt source address to cache
    ctl.push_u16(0x0040 | ((source_bit as u16) << 3));

    ctl.push_cycles(&[
        // Store pc into stack
        &[read::pc, dec::sp, read::sp],
        &[write::ind],
        &[dec::sp, read::sp],
        &[write::ind],
        // Jump to interrupt source address
        &[jump],
    ]);

    CONTINUE
}

pub fn disable_ime(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.int_flags.borrow_mut().master_enable = false;
    CONTINUE
}

pub fn enable_ime(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.int_flags.borrow_mut().master_enable = true;
    CONTINUE
}

pub fn halt(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    ctl.mode = Mode::Halt;
    CONTINUE
}

/// ## How the stop opcode work
///
/// **STOP** Reached
/// |
/// IS joypad pressed ?
/// |   |
/// NO YES
/// |    \_ IS an interrupt pending ? (`IE & IF != 0`)
/// |       |   |
/// |       NO YES
/// |       |    \_ STOP: 1 byte, mode: unchanged, DIV: unchanged
/// |       |
/// |        \_ STOP: 2 byte, mode: HALT, DIV: unchanged
/// |
/// The Step below is only for CGB mode
/// |
/// Was a speed switch requested ?
/// |   |
/// NO YES
/// |    \_ IS an interrupt pending ? (`IE & IF != 0`)
/// |       |   |
/// |       NO YES
/// |       |    \_ IS **IME** enabled ?
/// |       |       |   |
/// |       |       NO YES
/// |       |       |    \_ The CPU *glitches*
/// |       |       |
/// |       |       \_ STOP: 1 byte, mode: unchanged, DIV: reset, SPEED: change
/// |       |
/// |       \_ STOP: 2 byte, mode: HALT, DIV: reset, SPEED: change
/// |          Note: HALT mode exit automatically after ~0x20_000) Tcycle
/// |
/// IS an interrupt pending ? (`IE & IF != 0`)
/// |   |
/// NO YES
/// |    \_ STOP: 1 byte, mode: STOP, DIV: reset
/// |
/// \_ STOP: 2 byte, mode: STOP, DIV: reset
///
pub fn stop(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    use crate::constant::JOYPAD_INT;

    let int_flags = state.int_flags.borrow();

    if int_flags.flag & JOYPAD_INT == JOYPAD_INT {
        if !int_flags.is_interrupt_ready() {
            drop(int_flags);
            state.read();
            ctl.mode = Mode::Halt;
        }
    } else if cfg!(feature = "cgb") {
        unimplemented!("speed swicth");
    }
    CONTINUE
}
