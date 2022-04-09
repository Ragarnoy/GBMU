/// Name of the application
pub const APP_NAME: &str = "gbmu";
/// Name of the organization
// pub const ORG_NAME: &str = "";
// #[cfg(feature = "save_state")]
/// File extension for a file that should contain a `save state` save
// pub const SAVE_STATE_EXT: &str = "savepack";
/// File extension for a file that should contain a `game save` save
// pub const GAME_SAVE_EXT: &str = "gamepack";
/// List of prefered extensions for ROM file
pub const PREFERED_ROM_EXTS: [&str; 3] = ["rom", "gb", "gbc"];
// #[cfg(feature = "save_state")]
/// List of prefered extensions for `save state` file
// pub const PREFERED_SAVE_STATE_EXT: [&str; 1] = [SAVE_STATE_EXT];
// const TARGET_FPS_X10: u64 = 597;    // the true value
// pub const TARGET_FPS_X10: u64 = 600;
pub const GB_SCREEN_WIDTH: u32 = 160;
pub const GB_SCREEN_HEIGHT: u32 = 144;

pub const MENU_BAR_SIZE: f32 = 30.;
