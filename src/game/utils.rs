use gb_roms::{
    controllers::{generate_rom_controller, Generic, GenericState, Partial},
    Header,
};
use std::fs::File;

/// Return an initalised MBCs with it auto game save if possible
pub(crate) fn mbc_with_save_state(
    romname: &str,
    header: &Header,
    file: std::fs::File,
) -> anyhow::Result<Generic> {
    let mut mbc = generate_rom_controller(file, header.clone())?;

    {
        use rmp_serde::decode::from_read;

        let filename = crate::path::game_save_path(romname);
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
