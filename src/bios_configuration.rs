use std::path::PathBuf;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct BiosConfiguration {
    pub enable_dmg: bool,
    pub dmg_bios_file: Option<PathBuf>,
    pub enable_cbg: bool,
    pub cgb_bios_file: Option<PathBuf>,
}
