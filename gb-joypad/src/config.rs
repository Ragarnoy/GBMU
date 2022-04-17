use crate::InputType;
use std::collections::HashMap;
use winit::event::{KeyboardInput, ScanCode, VirtualKeyCode};

/// Store a joypad configuration.
///
/// Since it implement Serialise and Deserialize, it can be used to quickly save/load a joypad configuration into/from a file.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Config(pub(crate) HashMap<InputType, KeyEntry>);

lazy_static::lazy_static! {
    static ref DEFAULT_INPUT_MAP: HashMap<InputType, KeyEntry> = HashMap::from([
        (InputType::Up, KeyEntry::UP),
        (InputType::Down, KeyEntry::DOWN),
        (InputType::Left, KeyEntry::LEFT),
        (InputType::Right, KeyEntry::RIGHT),
        (InputType::Start, KeyEntry::RETURN),
        (InputType::Select, KeyEntry::RSHIFT),
        (InputType::B, KeyEntry::B),
        (InputType::A, KeyEntry::A),
    ]);
}

impl Default for Config {
    fn default() -> Self {
        Self(DEFAULT_INPUT_MAP.clone())
    }
}

impl Config {
    /// Try to get the [InputType] for a specific [KeyEntry]
    pub fn get_input_type(&self, key: &KeyEntry) -> Option<InputType> {
        self.0
            .iter()
            .find(|(_type, candidate_key)| *candidate_key == key)
            .map(|(input_type, _)| *input_type)
    }

    pub fn get_key_entry(&self, input_type: &InputType) -> Option<&KeyEntry> {
        self.0.get(input_type)
    }

    /// Update mapping for [KeyEntry]->[InputType]
    pub fn update_keybinding(&mut self, input_type: InputType, new_key: KeyEntry) {
        self.0.insert(input_type, new_key);
    }
}

#[derive(Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeyEntry {
    ScanCode(ScanCode),
    VirtualKeyCode(VirtualKeyCode),
}

impl From<KeyboardInput> for KeyEntry {
    fn from(ipt: KeyboardInput) -> Self {
        if let Some(virtual_keycode) = ipt.virtual_keycode {
            Self::VirtualKeyCode(virtual_keycode)
        } else {
            Self::ScanCode(ipt.scancode)
        }
    }
}

impl KeyEntry {
    pub const A: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::A);
    pub const B: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::B);
    pub const UP: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Up);
    pub const DOWN: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Down);
    pub const LEFT: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Left);
    pub const RIGHT: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Right);
    pub const RETURN: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Return);
    pub const RSHIFT: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::RShift);
}

impl KeyEntry {
    /// Return the name of the KeyEntry
    pub fn name(&self) -> String {
        match self {
            KeyEntry::ScanCode(code) => format!("{code}"),
            KeyEntry::VirtualKeyCode(code) => format!("{code:?}"),
        }
    }
}
