use crate::{Config, InputType};
use egui::{CtxRef, Ui};
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
            egui::Grid::new("input grid")
                .spacing([20.0, 8.0])
                .max_col_width(100.0)
                .min_col_width(100.0)
                .striped(true)
                .show(ui, |ui| {
                    for i_type in Self::INPUT_LIST.iter() {
                        ui.label(format!("{:?}:", i_type));
                        if let Some(listened) = self.listening {
                            if &listened == i_type {
                                ui.vertical_centered(|ui| {
                                    ui.label("---");
                                });
                                ui.vertical_centered(|ui| {
                                    if ui.button("❌").clicked() {
                                        self.listening = None;
                                    }
                                });
                            } else {
                                self.input_label(ui, i_type);
                            }
                        } else {
                            self.input_label(ui, i_type);
                        }
                        ui.end_row();
                    }
                });
            ui.vertical_centered(|ui| {
                ui.separator();
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
            })
        });
    }

    fn input_label(&mut self, ui: &mut Ui, i_type: &InputType) {
        ui.vertical_centered(|ui| {
            match self
                .input_map
                .iter()
                .find(|(_, map_val)| &i_type == map_val)
            {
                Some((code, _)) => ui.label(code.name()),
                None => ui.label("---"),
            };
        });
        ui.vertical_centered(|ui| {
            if ui.button("⚙").clicked() {
                self.listening = Some(*i_type);
            }
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
                if self.input_states[input_type] != state {
                    log::debug!(
                        "{:?} change state: {}",
                        input_type,
                        if state { "pressed" } else { "released" }
                    )
                }
                self.input_states.insert(*input_type, state);
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
