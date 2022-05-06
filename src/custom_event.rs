use std::path::PathBuf;

#[cfg(feature = "cgb")]
use crate::config::Mode;
use crate::windows::WindowType;

#[derive(Debug, Clone)]
pub enum CustomEvent {
    /// Event that will load a ROM file
    LoadFile(PathBuf),
    #[cfg(feature = "save_state")]
    /// Event that will generate a `save state` file
    SaveState(PathBuf),
    #[cfg(feature = "save_state")]
    /// Event that will load a `save state` save
    LoadState(PathBuf),

    #[cfg(feature = "cgb")]
    /// Event when we want to force a gameboy mode
    ChangedMode(Option<Mode>),

    OpenWindow(WindowType),
    CloseWindow(WindowType),
    /// Exit the emulator
    Quit,
}
