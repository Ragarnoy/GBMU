use std::path::PathBuf;

/// Return the path where the game save file will be located
pub fn game_save_path(rom_filename: &str) -> PathBuf {
    let rom_id = game_id(rom_filename);
    let mut root = root_config_path();

    root.push(rom_id);
    root.set_extension(crate::constant::GAME_SAVE_EXT);
    root
}

/// Return the root path of the config folder
pub fn root_config_path() -> PathBuf {
    let mut path = if let Some(home_dir) = std::env::var_os("HOME") {
        PathBuf::from(home_dir)
    } else {
        PathBuf::from(".")
    };
    path.push(".config");
    path.push(crate::constant::APP_NAME.to_lowercase());
    path
}

/// Return the root configuration path and create the config directory
pub fn create_root_config_path() -> std::io::Result<PathBuf> {
    let root_config = root_config_path();
    std::fs::create_dir_all(&root_config)?;
    Ok(root_config)
}

/// Create a standardize rom name id
fn game_id(rom_filename: &str) -> String {
    let rom_path = PathBuf::from(rom_filename);
    rom_path
        .file_stem()
        .map(|filename| filename.to_string_lossy().into_owned())
        .unwrap_or_else(|| rom_filename.replace('/', "_"))
}

/// Return the keybindings config file
pub fn keybinding_path() -> PathBuf {
    let mut path = root_config_path();
    path.push("keybindings.yaml");
    path
}

#[test]
fn test_game_id() {
    assert_eq!("foo", game_id("foo"));
    assert_eq!("foo", game_id("foo.bar"));
    assert_eq!("foo", game_id("bar/foo"));
    assert_eq!("foo", game_id("bar/foo.fuz"));
    assert_eq!("foo", game_id("nop/bar/foo"));
    assert_eq!("foo", game_id("nop/bar/foo.fuz"));
}
