use super::{fetch::fetch, opcode::Opcode, opcode_cb::OpcodeCB, Continuum, State};
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
}

type ActionFn<B> = fn(controller: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum;

impl<B: Bus<u8>> Default for MicrocodeController<B> {
    fn default() -> Self {
        Self {
            opcode: None,
            actions: Vec::with_capacity(8),
        }
    }
}

impl<B: Bus<u8>> MicrocodeController<B> {
    pub fn step(&mut self, regs: &mut Registers, bus: &mut B) {
        let mut state = State::new(regs, bus);
        let action = self.actions.pop().unwrap_or(fetch);

        let res = action(self, &mut state);
        match res {
            Continuum::Chain => self.step(regs, bus),
            Continuum::Break => self.actions.clear(),
            _ => {}
        }
    }

    pub fn push_action(&mut self, action: ActionFn<B>) {
        self.actions.push(action);
    }
}
