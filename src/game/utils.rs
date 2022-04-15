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
            log::info!("found auto save file at {}", filename);
            if let Err(e) =
                from_read(file).map(|state: GenericState<Partial>| mbc.load_partial(state))
            {
                log::error!(
                    "while loading data into mbc, got the following error: {}",
                    e
                )
            } else {
                log::info!("successfuly load mbc data from {}", filename);
            }
        }
    }

    Ok(mbc)
}

/// Return the path where the game save file will be located
pub fn game_save_path(rom_filename: &str) -> String {
    let rom_id = game_id(rom_filename);
    let root = game_root_config_path();

    std::path::Path::new(&root)
        .join(format!("{}.{}", rom_id, crate::constant::GAME_SAVE_EXT))
        .to_string_lossy()
        .to_string()
}

/// Return the root path of the config folder
pub fn game_root_config_path() -> String {
    sdl2::filesystem::pref_path(crate::constant::ORG_NAME, crate::constant::APP_NAME)
        .expect("a prefered config")
}

/// Create a standardize rom name id
fn game_id(rom_filename: &str) -> String {
    let rom_path = Path::new(rom_filename);
    rom_path
        .file_stem()
        .map_or_else(
            || rom_filename.to_string(),
            |filename| filename.to_string_lossy().to_string(),
        )
        .replace(" ", "-")
        .to_lowercase()
}
