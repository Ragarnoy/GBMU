use std::path::PathBuf;

#[derive(Debug)]
pub enum CustomEvent {
    /// Event that will load a ROM file
    LoadFile(PathBuf),
    #[cfg(feature = "save_state")]
    /// Event that will generate a `save state` file
    SaveState(PathBuf),
    #[cfg(feature = "save_state")]
    /// Event that will load a `save state` save
    LoadState(PathBuf),
}
