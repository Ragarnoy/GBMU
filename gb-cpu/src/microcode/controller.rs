use super::{
    dec, fetch::fetch, jump::jump, opcode::Opcode, opcode_cb::OpcodeCB, read, utils::sleep, write,
    MicrocodeFlow, State,
};
use crate::registers::Registers;
use gb_bus::Bus;

pub enum OpcodeType {
    Unprefixed(Opcode),
    CBPrefixed(OpcodeCB),
}

impl From<Opcode> for OpcodeType {
    fn from(opc: Opcode) -> Self {
        OpcodeType::Unprefixed(opc)
    }
}

impl From<OpcodeCB> for OpcodeType {
    fn from(opc: OpcodeCB) -> Self {
        OpcodeType::CBPrefixed(opc)
    }
}

pub struct MicrocodeController {
    /// current opcode
    pub opcode: Option<OpcodeType>,
    /// Microcode actions, their role is to execute one step of an Opcode
    /// Each Actions take at most 1 `M-Cycle`
    /// Used like a LOFI queue
    actions: Vec<ActionFn>,
    /// Cache use for microcode action
    cache: Vec<u8>,
    /// The IME flag is used to disable all interrupts
    interrupt_master_enable: bool,
}

type ActionFn = fn(controller: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow;

impl Default for MicrocodeController {
    fn default() -> Self {
        Self {
            opcode: None,
            actions: Vec::with_capacity(8),
            cache: Vec::with_capacity(4),
            interrupt_master_enable: true,
        }
    }
}

impl MicrocodeController {
    pub fn step(&mut self, regs: &mut Registers, bus: &mut impl Bus<u8>) {
        use super::CycleDigest;
        use std::ops::ControlFlow;

        let mut state = State::new(regs, bus);
        let action = self.actions.pop().unwrap_or_else(|| {
            self.clear();
            self.handle_interrupts(&mut state);
            fetch
        });

        match action(self, &mut state) {
            ControlFlow::Continue(CycleDigest::Again) => self.step(regs, bus),
            ControlFlow::Break(cycle_digest) => {
                self.clear();
                if cycle_digest == CycleDigest::Again {
                    self.step(regs, bus);
                }
            }
            ControlFlow::Continue(CycleDigest::Consume) => {}
        }
    }

    /// Clear volatile date saved in controller.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.actions.clear();
    }

    /// Push the action a the back of the queue.
    /// The last pushed action will be the first to be executed
    pub fn push_action(&mut self, action: ActionFn) -> &mut Self {
        self.actions.push(action);
        self
    }

    /// Push the actions in the queue.
    /// The actions while be push in the queue in a way that allow the first action of the slice
    /// to be executed in first.
    pub fn push_actions(&mut self, actions: &[ActionFn]) -> &mut Self {
        for action in actions.iter().rev() {
            self.push_action(*action);
        }
        self
    }

    /// Push the `byte` to the cache.
    /// The last `byte` pushed will be the first accessed by `MicrocodeController::pop`.
    pub fn push(&mut self, byte: u8) {
        self.cache.push(byte)
    }

    /// Push the value to the cache
    pub fn push_u16(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.cache.push(bytes[0]);
        self.cache.push(bytes[1]);
    }

    /// Pop the last pushed `byte` from the cache.
    pub fn pop(&mut self) -> u8 {
        self.cache.pop().expect("not enough value stored in cache")
    }

    /// Pop the last u16 from the cache.
    pub fn pop_u16(&mut self) -> u16 {
        u16::from_be_bytes([self.pop(), self.pop()])
    }

    fn handle_interrupts(&mut self, state: &mut State) {
        // TODO handle HALT

        if self.interrupt_master_enable {
            self.interrupt_master_enable = false;
            let interrupt_flag = state.read_interrupt_flag();
            let interrupt_enable = state.read_interrupt_enable();
            let mut interrupt_ready = interrupt_flag & interrupt_enable;

            if interrupt_ready != 0 {
                let mut source_bit: u8 = 0;
                while interrupt_ready & 0x1 == 0 {
                    interrupt_ready <<= 1;
                    source_bit += 1;
                }
                if source_bit <= 4 {
                    let bit_to_res = 1_u8 << source_bit;
                    let new_interrupt_flag = !(!interrupt_flag | bit_to_res);
                    state.write_interrupt_flag(new_interrupt_flag);

                    // Sleep 2 mcycles
                    sleep(self, state);
                    sleep(self, state);

                    // Store pc into stack
                    read::pc(self, state);
                    dec::sp(self, state);
                    read::sp(self, state);
                    write::ind(self, state);
                    dec::sp(self, state);
                    read::sp(self, state);
                    write::ind(self, state);

                    // Jump to interrupt source address
                    self.push_u16(0x0040 | ((source_bit as u16) << 3));
                    jump(self, state);
                }
            }
        }
    }
}
