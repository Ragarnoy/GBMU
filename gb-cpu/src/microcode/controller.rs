use super::{fetch::fetch, opcode::Opcode, opcode_cb::OpcodeCB, ControlFlow, State};
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

pub struct MicrocodeController<B: Bus<u8>> {
    /// current opcode
    pub opcode: Option<OpcodeType>,
    /// Microcode actions, their role is to execute one step of an Opcode
    /// Each Actions take at most 1 `M-Cycle`
    actions: Vec<ActionFn<B>>,
    /// Cache use for microcode action
    cache: Vec<u8>,
}

type ActionFn<B> = fn(controller: &mut MicrocodeController<B>, state: &mut State<B>) -> ControlFlow;

impl<B: Bus<u8>> Default for MicrocodeController<B> {
    fn default() -> Self {
        Self {
            opcode: None,
            actions: Vec::with_capacity(8),
            cache: Vec::with_capacity(4),
        }
    }
}

impl<B: Bus<u8>> MicrocodeController<B> {
    pub fn step(&mut self, regs: &mut Registers, bus: &mut B) {
        let mut state = State::new(regs, bus);
        let action = self.actions.pop().unwrap_or_else(|| {
            self.cache.clear();
            fetch
        });

        let res = action(self, &mut state);
        match res {
            ControlFlow::Chain => self.step(regs, bus),
            ControlFlow::Break => self.actions.clear(),
            _ => {}
        }
    }

    pub fn push_action(&mut self, action: ActionFn<B>) {
        self.actions.push(action);
    }

    pub fn push(&mut self, byte: u8) {
        self.cache.push(byte)
    }

    pub fn pop(&mut self) -> u8 {
        self.cache.pop().expect("not enough value stored in cache")
    }
}
