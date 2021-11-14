use super::{
    fetch::fetch, interrupts::handle_interrupts, opcode::Opcode, opcode_cb::OpcodeCB, CycleDigest,
    MicrocodeFlow, State,
};
use crate::registers::Registers;
use gb_bus::{Area, Bus, FileOperation, IORegArea};
use std::fmt::{self, Debug, Display};

#[derive(Clone, Debug)]
pub enum OpcodeType {
    Unprefixed(Opcode),
    CBPrefixed(OpcodeCB),
}

impl Display for OpcodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpcodeType::Unprefixed(opcode) => write!(f, "{:?}", opcode),
            OpcodeType::CBPrefixed(cb_opcode) => write!(f, "{:?}", cb_opcode),
        }
    }
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

#[derive(Clone)]
pub struct MicrocodeController {
    /// current opcode
    pub opcode: Option<OpcodeType>,
    /// Microcode actions, their role is to execute one step of an Opcode
    /// Each Actions take at most 1 `M-Cycle`
    /// Used like a LOFI queue
    pub actions: Vec<ActionFn>,
    /// Cache use for microcode action
    cache: Vec<u8>,
    /// The IME flag is used to disable all interrupts
    pub interrupt_master_enable: bool,
    pub interrupt_flag: u8,
    pub interrupt_enable: u8,
}

impl Debug for MicrocodeController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MicrocodeController {{ opcode: {:?}, actions: {}, cache: {:?} }}",
            self.opcode,
            self.actions.len(),
            self.cache
        )
    }
}

type ActionFn = fn(controller: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow;

impl Default for MicrocodeController {
    fn default() -> Self {
        Self {
            opcode: None,
            actions: Vec::with_capacity(12),
            cache: Vec::with_capacity(6),
            interrupt_master_enable: true,
            interrupt_flag: 0,
            interrupt_enable: 0,
        }
    }
}

impl MicrocodeController {
    pub fn step(&mut self, regs: &mut Registers, bus: &mut dyn Bus<u8>) {
        use std::ops::ControlFlow;

        let mut state = State::new(regs, bus);
        let action = self.actions.pop().unwrap_or_else(|| {
            self.clear();
            if self.is_interrupt_ready() {
                handle_interrupts
            } else {
                fetch
            }
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
        self.cache.push(bytes[1]);
        self.cache.push(bytes[0]);
    }

    /// Pop the last pushed `byte` from the cache.
    pub fn pop(&mut self) -> u8 {
        self.cache.pop().expect("not enough value stored in cache")
    }

    /// Pop the last u16 from the cache.
    pub fn pop_u16(&mut self) -> u16 {
        u16::from_be_bytes([self.pop(), self.pop()])
    }

    fn is_interrupt_ready(&self) -> bool {
        if !self.interrupt_master_enable {
            return false;
        }
        let interrupt_flag = self.interrupt_flag;
        let interrupt_enable = self.interrupt_enable;
        interrupt_flag & interrupt_enable != 0
    }
}

impl FileOperation<Area> for MicrocodeController {
    fn read(&self, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<u8, gb_bus::Error> {
        Ok(self.interrupt_enable)
    }
    fn write(&mut self, v: u8, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<(), gb_bus::Error> {
        self.interrupt_enable = v;
        Ok(())
    }
}

impl FileOperation<IORegArea> for MicrocodeController {
    fn read(&self, _addr: Box<dyn gb_bus::Address<IORegArea>>) -> Result<u8, gb_bus::Error> {
        Ok(self.interrupt_flag)
    }
    fn write(
        &mut self,
        v: u8,
        _addr: Box<dyn gb_bus::Address<IORegArea>>,
    ) -> Result<(), gb_bus::Error> {
        self.interrupt_flag = v;
        Ok(())
    }
}
