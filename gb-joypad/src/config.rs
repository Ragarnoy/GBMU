use crate::InputType;
use std::collections::HashMap;
use winit::event::{KeyboardInput, ScanCode, VirtualKeyCode};

/// Store a joypad configuration.
///
/// Since it implement Serialise and Deserialize, it can be used to quickly save/load a joypad configuration into/from a file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub mapping: HashMap<KeyEntry, InputType>,
}

#[derive(Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
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
