use gb_roms::{
    controllers::{generate_rom_controller, Generic, GenericState, Partial},
    Header,
};
use std::{fs::File, path::Path};

/// Return an initalised MBCs with it auto game save if possible
pub(crate) fn mbc_with_save_state(
    romname: &str,
    header: &Header,
    file: std::fs::File,
) -> anyhow::Result<Generic> {
    let mut mbc = generate_rom_controller(file, header.clone())?;

    {
        use rmp_serde::decode::from_read;

        let filename = game_save_path(romname);
        if let Ok(file) = File::open(&filename) {
            log::info!("found auto save file at {}", filename.to_string_lossy());
            if let Err(e) =
                from_read(file).map(|state: GenericState<Partial>| mbc.load_partial(state))
            {
                log::error!(
                    "while loading data into mbc, got the following error: {}",
                    e
                )
            } else {
                log::info!(
                    "successfuly load mbc data from {}",
                    filename.to_string_lossy()
                );
            }
        }
    }

    Ok(mbc)
}

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
    let mut path = PathBuf::from_str("~/.config/").expect("cannot create path");
    path.push(crate::constant::APP_NAME);
    path
}

/// Create a standardize rom name id
fn game_id(rom_filename: &str) -> PathBuf {
    let mut rom_path = PathBuf::from(rom_filename);
    rom_path.set_extension("");
    rom_path
}
