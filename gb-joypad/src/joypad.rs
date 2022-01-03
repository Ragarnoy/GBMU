use crate::{
    utils::{register_from_state, trigger_interrupt, Mode},
    Config, InputType,
};
use egui::{CtxRef, Direction, Layout, Separator, Ui};
use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
/// Translate events from keyboard input inputs for the gameboy.
pub struct Joypad {
    window_id: u32,
    input_map: HashMap<Scancode, InputType>,
    input_states: HashMap<InputType, bool>,
    listening: Option<InputType>,
    mode: Mode,
    reg_val: u8,
}

const DEFAULT_UP: Scancode = Scancode::Up;
const DEFAULT_DOWN: Scancode = Scancode::Down;
const DEFAULT_LEFT: Scancode = Scancode::Left;
const DEFAULT_RIGHT: Scancode = Scancode::Right;
const DEFAULT_START: Scancode = Scancode::Return;
const DEFAULT_SELECT: Scancode = Scancode::RShift;
const DEFAULT_B: Scancode = Scancode::B;
const DEFAULT_A: Scancode = Scancode::A;

impl Joypad {
    const INPUT_LIST: [InputType; 8] = [
        InputType::Up,
        InputType::Down,
        InputType::Left,
        InputType::Right,
        InputType::Start,
        InputType::Select,
        InputType::B,
        InputType::A,
    ];

    pub fn new(window_id: u32) -> Self {
        Joypad {
            window_id,
            input_map: HashMap::from_iter([
                (DEFAULT_UP, InputType::Up),
                (DEFAULT_DOWN, InputType::Down),
                (DEFAULT_LEFT, InputType::Left),
                (DEFAULT_RIGHT, InputType::Right),
                (DEFAULT_START, InputType::Start),
                (DEFAULT_SELECT, InputType::Select),
                (DEFAULT_B, InputType::B),
                (DEFAULT_A, InputType::A),
            ]),
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
            listening: None,
            mode: Default::default(),
            reg_val: 0xff,
        }
    }

    pub fn from_config(window_id: u32, conf: Config) -> Self {
        Joypad {
            window_id,
            input_map: conf.mapping,
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
            listening: None,
            mode: Default::default(),
            reg_val: 0xff,
        }
    }

    pub fn get_config(&self) -> Config {
        Config {
            mapping: self.input_map.clone(),
        }
    }

    fn set_input_map(&mut self, scancode: Scancode, input_type: InputType) {
        self.input_map.retain(|_, v| v != &input_type);
        self.input_map.insert(scancode, input_type);
    }

    /// Draw the ui to configure the inputs settings.
    pub fn settings(&mut self, ctx: &CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let height = ui.available_size().y;
            egui::ScrollArea::vertical()
                .max_height(height - 50.0)
                .show(ui, |ui| {
                    ui.set_height(height - 60.0);
                    for i_type in Self::INPUT_LIST.iter() {
                        ui.horizontal(|ui| {
                            if let Some(listened) = self.listening {
                                self.input_row(ui, i_type, &listened == i_type);
                            } else {
                                self.input_row(ui, i_type, false);
                            }
                        });
                    }
                });
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add(Separator::default().horizontal().spacing(30.0));
                    if ui.button("reset   ⟲").clicked() {
                        self.listening = None;
                        self.input_map = HashMap::from_iter([
                            (DEFAULT_UP, InputType::Up),
                            (DEFAULT_DOWN, InputType::Down),
                            (DEFAULT_LEFT, InputType::Left),
                            (DEFAULT_RIGHT, InputType::Right),
                            (DEFAULT_START, InputType::Start),
                            (DEFAULT_SELECT, InputType::Select),
                            (DEFAULT_B, InputType::B),
                            (DEFAULT_A, InputType::A),
                        ]);
                    }
                });
            });
        });
    }

    fn input_row(&mut self, ui: &mut Ui, i_type: &InputType, force_empty: bool) {
        ui.columns(3, |ui| {
            ui[0].label(format!("{:?}:", i_type));
            ui[1].with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    if force_empty {
                        ui.label("---");
                    } else {
                        match self
                            .input_map
                            .iter()
                            .find(|(_, map_val)| &i_type == map_val)
                        {
                            Some((code, _)) => ui.label(code.name()),
                            None => ui.label("---"),
                        };
                    }
                },
            );
            ui[2].with_layout(Layout::right_to_left(), |ui| {
                if !force_empty && ui.button("⚙").clicked() {
                    self.listening = Some(*i_type);
                } else if force_empty && ui.button("❌").clicked() {
                    self.listening = None;
                }
            });
        });
    }

    fn update_mapping(&mut self, scancode: &Option<Scancode>) {
        if let Some(scancode) = scancode {
            if let Some(listened) = self.listening {
                self.set_input_map(*scancode, listened);
                self.listening = None;
            }
        }
    }

    fn update_state(&mut self, scancode: &Option<Scancode>, state: bool) {
        if let Some(scancode) = scancode {
            if let Some(input_type) = self.input_map.get(scancode) {
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
                if self.input_states[input_type] != state {
                    self.input_states.insert(*input_type, state);
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
            }
        }
    }

    /// Every event from the sdl2 should be sent to the joypad so it can keeps its inputs state updated.
    pub fn send_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                window_id,
                scancode,
                ..
            } => {
                if window_id == &self.window_id {
                    self.update_state(scancode, true);
                }
            }
            Event::KeyUp {
                window_id,
                scancode,
                ..
            } => {
                self.update_mapping(scancode);
                if window_id == &self.window_id {
                    self.update_state(scancode, false);
                }
            }
            _ => {}
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Joypad
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match (addr.area_type(), addr.get_address()) {
            (IORegArea::Controller, 0x00) => {
                let v = !v & 0b0011_0000;
                self.mode = Mode::from(v);
                Ok(())
            }
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    fn read(&self, addr: A) -> Result<u8, Error> {
        match (addr.area_type(), addr.get_address()) {
            (IORegArea::Controller, 0x00) => Ok(self.reg_val),
            _ => Err(Error::SegmentationFault(addr.into())),
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
