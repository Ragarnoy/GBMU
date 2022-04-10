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
    pub const Up: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Up);
    pub const Down: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Down);
    pub const Left: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Left);
    pub const Right: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Right);
    pub const Return: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::Return);
    pub const RShift: KeyEntry = KeyEntry::VirtualKeyCode(VirtualKeyCode::RShift);
}
