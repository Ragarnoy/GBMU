use super::{
    fetch::fetch, interrupts::handle_interrupts, opcode::Opcode, opcode_cb::OpcodeCB,
    MicrocodeFlow, State,
};
use crate::{
    io_registers::IORegisters, registers::Registers, CACHE_LEN, MAX_CYCLES_IN_HALT_MODE,
    NB_MAX_ACTIONS, NB_MAX_CYCLES,
};
use gb_bus::Bus;
use std::fmt::{self, Debug, Display};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Halt,
    Stop,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub struct MicrocodeController {
    /// current opcode
    pub opcode: Option<OpcodeType>,
    /// Stores all the cycles to do of the current opcode
    pub cycles: Vec<Vec<ActionFn>>,
    /// Stores the next microcode actions to execute
    pub current_cycle: Vec<ActionFn>,
    /// Current mode of the controller
    pub mode: Mode,
    /// Handles a peculiar case of the stop opcode
    /// It appends when the switch of speed is made and that an interrupt isn't pending
    /// In this case it will enter the halt mode and will exit it automatically after about 0x2000 t-cycles
    /// https://gbdev.io/pandocs/Reducing_Power_Consumption.html?highlight=stop#using-the-stop-instruction
    pub halted_from_stop: bool,
    pub cycles_in_halt_mode: usize,
    /// Cache use for microcode action
    cache: Vec<u8>,
    /// Debug helper to catch the event of end of instruction
    pub is_instruction_finished: bool,
    pub cgb_mode: bool,
}

impl Debug for MicrocodeController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MicrocodeController {{ opcode: {:?}, actions: {}, cache: {:?} }}",
            self.opcode,
            self.current_cycle.len(),
            self.cache
        )
    }
}

type ActionFn = fn(controller: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow;

impl MicrocodeController {
    pub fn new(cgb_mode: bool) -> Self {
        Self {
            opcode: None,
            cycles: Vec::with_capacity(NB_MAX_CYCLES),
            current_cycle: Vec::with_capacity(NB_MAX_ACTIONS),
            cache: Vec::with_capacity(CACHE_LEN),
            mode: Mode::default(),
            halted_from_stop: false,
            cycles_in_halt_mode: 0,
            is_instruction_finished: true,
            cgb_mode,
        }
    }

    pub fn step(
        &mut self,
        int_flags: Rc<RefCell<IORegisters>>,
        regs: &mut Registers,
        bus: &mut dyn Bus<u8>,
    ) {
        self.is_instruction_finished = false;

        let mut state = State::new(regs, bus, int_flags.clone());

        if let Some(cycle) = self.cycles.pop() {
            self.push_to_current_cycle(&cycle);
        } else {
            self.clear();
            self.pull_next_task(&mut state, int_flags);
        }

        self.execute_actions(&mut state);

        if self.cycles.is_empty() {
            self.is_instruction_finished = true;
        }
    }

    /// Pull the next task the cpu will do according to it's current mode
    fn pull_next_task(&mut self, state: &mut State, int_flags: Rc<RefCell<IORegisters>>) {
        match self.mode {
            Mode::Normal => self.normal_mode(state, int_flags),
            Mode::Halt => self.halt_mode(state, int_flags),
            Mode::Stop => self.stop_mode(state, int_flags),
        }
    }

    /// When the cpu is in it's normal execution mode.
    /// Will execute interrupt before fetching for new opcode.
    fn normal_mode(&mut self, state: &mut State, int_flags: Rc<RefCell<IORegisters>>) {
        let previous_opcode = match self.opcode {
            Some(OpcodeType::Unprefixed(opcode)) => opcode,
            _ => Opcode::Nop,
        };
        let borrow_int_flags = int_flags.borrow();

        if previous_opcode != Opcode::Ei && borrow_int_flags.interrupt_to_handle() {
            drop(borrow_int_flags);
            handle_interrupts(self, state);
        } else {
            fetch(self, state);
        }
    }

    /// When the cpu is halted.
    /// Wait for any interrupt to be triggered to return to the normal execution mode.
    /// Directly service the triggered interrupt if IME is enabled.
    fn halt_mode(&mut self, state: &mut State, int_flags: Rc<RefCell<IORegisters>>) {
        let borrow_int_flags = int_flags.borrow();

        if self.halted_from_stop {
            self.cycles_in_halt_mode += 1;
        }

        if borrow_int_flags.is_interrupt_ready() {
            self.mode = Mode::Normal;
            if borrow_int_flags.should_handle_interrupt() {
                drop(borrow_int_flags);
                handle_interrupts(self, state);
            }
            self.halted_from_stop = false;
        } else if self.halted_from_stop && self.cycles_in_halt_mode >= MAX_CYCLES_IN_HALT_MODE {
            self.mode = Mode::Normal;
            self.halted_from_stop = false;
        }
    }

    /// When the cpu is stopped.
    /// Wait for the joypad to be pressed
    fn stop_mode(&mut self, _state: &mut State, int_flags: Rc<RefCell<IORegisters>>) {
        use crate::constant::JOYPAD_INT;

        if int_flags.borrow().flag & JOYPAD_INT == JOYPAD_INT {
            self.mode = Mode::Normal;
        }
        unimplemented!("stop waiting");
    }

    fn execute_actions(&mut self, state: &mut State) {
        use std::ops::ControlFlow;

        if let Some(action) = self.current_cycle.pop() {
            match action(self, state) {
                ControlFlow::Continue(()) => self.execute_actions(state),
                ControlFlow::Break(()) => {
                    self.clear();
                }
            }
        }
    }

    /// Clear volatile date saved in controller.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.current_cycle.clear();
        self.cycles.clear();
    }

    /// Push the action a the back of the queue.
    /// The last pushed action will be the first to be executed
    pub fn push_action(&mut self, action: ActionFn) -> &mut Self {
        self.current_cycle.push(action);
        self
    }

    /// Push the actions in the queue.
    /// The actions while be push in the queue in a way that allow the first action of the slice
    /// to be executed in first.
    pub fn push_to_current_cycle(&mut self, actions: &[ActionFn]) -> &mut Self {
        self.current_cycle.extend(actions.iter().rev());
        self
    }

    pub fn push_cycles(&mut self, cycles: &[&[ActionFn]]) -> &mut Self {
        self.cycles = cycles
            .iter()
            .rev()
            .map(|actions| actions.to_vec())
            .collect::<Vec<Vec<ActionFn>>>();
        self
    }

    pub fn push_cycle(&mut self, cycle: &[ActionFn]) -> &mut Self {
        self.cycles.push(cycle.to_vec());
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
        self.cache.pop().unwrap_or_else(|| {
            panic!(
                "not enough value stored in cache for opcode {:?}",
                self.opcode
            )
        })
    }

    /// Pop the last u16 from the cache.
    pub fn pop_u16(&mut self) -> u16 {
        u16::from_be_bytes([self.pop(), self.pop()])
    }
}
