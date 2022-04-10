use crate::InputType;
use std::collections::HashMap;
use winit::event::ScanCode;

/// Store a joypad configuration.
///
/// Since it implement Serialise and Deserialize, it can be used to quickly save/load a joypad configuration into/from a file.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub mapping: HashMap<ScanCode, InputType>,
}
