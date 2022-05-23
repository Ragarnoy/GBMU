use crate::{
    config::KeyEntry,
    utils::{register_from_state, trigger_interrupt, Mode},
    Config, InputType,
};
use gb_bus::{Address, Bus, Error, FileOperation, IORegArea, Source};
use gb_clock::{Tick, Ticker};
use std::iter::FromIterator;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
/// Translate events from keyboard input inputs for the gameboy.
pub struct Joypad {
    config: Rc<RefCell<Config>>,
    input_states: HashMap<InputType, bool>,
    mode: Mode,
    reg_val: u8,
}

impl Joypad {
    const READ_MASK: u8 = 0b1100_0000;
    const WRITABLE_BITS: u8 = 0b0011_0000;

    pub fn from_config(config: Rc<RefCell<Config>>) -> Self {
        Joypad {
            config,
            input_states: HashMap::from_iter([
                (InputType::Up, false),
                (InputType::Down, false),
                (InputType::Left, false),
                (InputType::Right, false),
                (InputType::Start, false),
                (InputType::Select, false),
                (InputType::B, false),
                (InputType::A, false),
            ]),
            mode: Default::default(),
            reg_val: 0xff,
        }
    }

    /// Update the state of the joypad on key event (release / pressed)
    /// Return true when the key event is used by the joypad
    pub fn on_key_event(&mut self, key: KeyEntry, pressed: bool) -> bool {
        if let Some(input_type) = self.config.borrow().get_input_type(&key) {
            #[cfg(feature = "debug_state")]
            let mut changed = false;
            #[cfg(feature = "toggle_joypad")]
            if state {
                self.input_states.insert(
                    *input_type,
                    !self.input_states.get(input_type).unwrap_or(&false),
                );
                #[cfg(feature = "debug_state")]
                {
                    changed = true;
                }
            }
            #[cfg(not(feature = "toggle_joypad"))]
            if self.input_states[&input_type] != pressed {
                self.input_states.insert(input_type, pressed);
                #[cfg(feature = "debug_state")]
                {
                    changed = true;
                }
            }
            #[cfg(feature = "debug_state")]
            if changed {
                let reg = register_from_state(self.mode, self.input_states.iter());
                log::debug!(
                    "change state: state={:08b}, mode={:9?}, key={:5?}, pressed={}",
                    reg,
                    self.mode,
                    input_type,
                    state,
                )
            }
            true
        } else {
            false
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Joypad
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        if IORegArea::Joy == addr.area_type() {
            let v = !v & Self::WRITABLE_BITS;
            self.mode = Mode::from(v);
            Ok(())
        } else {
            Err(Error::SegmentationFault(addr.into()))
        }
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        if IORegArea::Joy == addr.area_type() {
            Ok(Self::READ_MASK | self.reg_val)
        } else {
            Err(Error::SegmentationFault(addr.into()))
        }
    }
}

impl Ticker for Joypad {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        macro_rules! fedge_detector {
            ($old: expr, $new: expr, $mask: literal) => {
                ($old & $mask) > ($new & $mask)
            };
        }

        let new_reg = register_from_state(self.mode, self.input_states.iter());

        if fedge_detector!(self.reg_val, new_reg, 0b1)
            || fedge_detector!(self.reg_val, new_reg, 0b10)
            || fedge_detector!(self.reg_val, new_reg, 0b100)
        {
            trigger_interrupt(addr_bus);
        }
        self.reg_val = new_reg;
    }
}
