use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum InputType {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

#[derive(Debug)]
pub struct Joypad {
    window_id: u32,
    input_map: HashMap<Scancode, InputType>,
    input_states: HashMap<InputType, bool>,
}

impl Joypad {
    pub fn new(window_id: u32) -> Self {
        Joypad {
            window_id,
            input_map: HashMap::from_iter([
                (Scancode::Up, InputType::Up),
                (Scancode::Down, InputType::Down),
                (Scancode::Left, InputType::Left),
                (Scancode::Right, InputType::Right),
                (Scancode::Return, InputType::Start),
                (Scancode::Space, InputType::Select),
                (Scancode::B, InputType::B),
                (Scancode::A, InputType::A),
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
        }
    }
    pub fn send_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                window_id,
                scancode,
                ..
            } => {
                if window_id == &self.window_id {
                    if let Some(scancode) = scancode {
                        if let Some(input_type) = self.input_map.get(scancode) {
                            self.input_states.insert(*input_type, true);
                        }
                    }
                }
            }
            Event::KeyUp {
                window_id,
                scancode,
                ..
            } => {
                if window_id == &self.window_id {
                    if let Some(scancode) = scancode {
                        if let Some(input_type) = self.input_map.get(scancode) {
                            self.input_states.insert(*input_type, false);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
