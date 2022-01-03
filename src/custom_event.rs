use std::path::PathBuf;

#[derive(Debug)]
pub enum CustomEvent {
    /// Event that will load a ROM file
    LoadFile(PathBuf),
    /// Event that will generate a `save state` file
    SaveState(PathBuf),
    /// Event that will load a `save state` save
    LoadState(PathBuf),
}
