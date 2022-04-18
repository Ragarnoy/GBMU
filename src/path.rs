use std::path::PathBuf;

/// Return the path where the game save file will be located
pub fn game_save_path(rom_filename: &str) -> PathBuf {
    let rom_id = game_id(rom_filename);
    let mut root = game_root_config_path();

    root.push(rom_id);
    root.set_extension(crate::constant::GAME_SAVE_EXT);
    root
}

/// Return the root path of the config folder
pub fn game_root_config_path() -> PathBuf {
    let mut path = if let Some(home_dir) = std::env::var_os("HOME") {
        PathBuf::from(home_dir)
    } else {
        PathBuf::from(".")
    };
    path.push(".config");
    path.push(crate::constant::APP_NAME.to_lowercase());
    path
}

/// Create a standardize rom name id
fn game_id(rom_filename: &str) -> PathBuf {
    let mut rom_path = PathBuf::from(rom_filename);
    rom_path.set_extension("");
    rom_path
}
