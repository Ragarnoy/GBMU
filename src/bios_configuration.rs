#[derive(Default)]
pub struct BiosConfiguration {
    pub enable_dmg: bool,
    pub dmg_bios_file: Option<String>,
    pub enable_cbg: bool,
    pub cgb_bios_file: Option<String>,
}
