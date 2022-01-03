use super::{
    fetch::fetch, interrupts::handle_interrupts, opcode::Opcode, opcode_cb::OpcodeCB,
    MicrocodeFlow, State,
};
use crate::{
    io_registers::IORegisters, registers::Registers, CACHE_LEN, NB_MAX_ACTIONS, NB_MAX_CYCLES,
};
use gb_bus::Bus;
use std::fmt::{self, Debug, Display};
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "registers_logs")]
use std::{fs::File, io::BufWriter};

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
    /// Stores all the cycles to do of the current opcode
    pub cycles: Vec<Vec<ActionFn>>,
    /// Stores the next microcode actions to execute
    pub current_cycle: Vec<ActionFn>,
    /// Current mode of the controller
    pub mode: Mode,
    /// Cache use for microcode action
    cache: Vec<u8>,

    #[cfg(feature = "registers_logs")]
    file: Rc<RefCell<BufWriter<File>>>,
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

impl Default for MicrocodeController {
    fn default() -> Self {
        #[cfg(feature = "registers_logs")]
        let file = MicrocodeController::create_new_file().unwrap();
        Self {
            opcode: None,
            cycles: Vec::with_capacity(NB_MAX_CYCLES),
            current_cycle: Vec::with_capacity(NB_MAX_ACTIONS),
            cache: Vec::with_capacity(CACHE_LEN),
            mode: Mode::default(),
            #[cfg(feature = "registers_logs")]
            file: Rc::new(RefCell::new(file)),
        }
    }
}

impl MicrocodeController {
    pub fn step(
        &mut self,
        int_flags: Rc<RefCell<IORegisters>>,
        regs: &mut Registers,
        bus: &mut dyn Bus<u8>,
    ) {
        let mut state = State::new(regs, bus, int_flags.clone());

        if let Some(cycle) = self.cycles.pop() {
            self.push_to_current_cycle(&cycle);
        } else {
            self.clear();
            self.pull_next_task(&mut state, int_flags);
        }

        self.execute_actions(&mut state);
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

        if previous_opcode != Opcode::Ei && int_flags.borrow().interrupt_to_handle() {
            handle_interrupts(self, state);
        } else {
            #[cfg(feature = "registers_logs")]
            self.log_registers_to_file(&state).unwrap_or_default();
            fetch(self, state);
        }
    }

    /// When the cpu is halted.
    /// Wait for any interrupt to be triggered to return to the normal execution mode.
    /// Directly service the triggered interrupt if IME is enabled.
    fn halt_mode(&mut self, state: &mut State, int_flags: Rc<RefCell<IORegisters>>) {
        let borrow_int_flags = int_flags.borrow();

        if borrow_int_flags.is_interrupt_ready() {
            self.mode = Mode::Normal;
            if borrow_int_flags.should_handle_interrupt() {
                handle_interrupts(self, state);
            }
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

    #[cfg(feature = "registers_logs")]
    fn log_registers_to_file(&mut self, state: &State) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = self.file.borrow_mut();

        if let Err(e) = writeln!(file, "{:?}", state) {
            log::error!("Couldn't write to file: {}", e);
        }
        Ok(())
    }

    #[cfg(feature = "registers_logs")]
    fn create_new_file() -> std::io::Result<BufWriter<File>> {
        use std::{env, fs::OpenOptions};

        let registers_logs = {
            use env::{current_dir, var};
            let mut project_path = var("LOG_DIR")
                .map(std::path::PathBuf::from)
                .or_else(|_| current_dir())?;
            project_path.push("registers.log");
            project_path
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(registers_logs)?;
        Ok(BufWriter::new(file))
    }
}
