/// Name of the application
pub const APP_NAME: &str = "gbmu";
/// Name of the organization
pub const ORG_NAME: &str = "";
#[cfg(feature = "save_state")]
/// File extension for a file that should contain a `save state` save
pub const SAVE_STATE_EXT: &str = "savepack";
/// File extension for a file that should contain a `game save` save
pub const GAME_SAVE_EXT: &str = "gamepack";
/// List of prefered extensions for ROM file
pub const PREFERED_ROM_EXTS: [&str; 3] = ["rom", "gb", "gbc"];
#[cfg(feature = "save_state")]
/// List of prefered extensions for `save state` file
pub const PREFERED_SAVE_STATE_EXT: [&str; 1] = [SAVE_STATE_EXT];
/// FPS we want to have
// pub const TARGET_FPS_X10: u64 = 597;    // the true value
pub const TARGET_FPS_X10: u64 = 600;
