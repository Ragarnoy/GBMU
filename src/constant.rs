/// Name of the application
pub const APP_NAME: &str = "gbmu";
/// Name of the organization
// pub const ORG_NAME: &str = "";
// #[cfg(feature = "save_state")]
// /// File extension for a file that should contain a `save state` save
// pub const SAVE_STATE_EXT: &str = "savepack";
/// File extension for a file that should contain a `game save` save
pub const GAME_SAVE_EXT: &str = "gamepack";
/// List of preferred extensions for ROM file
pub const PREFERRED_ROM_EXTS: [&str; 3] = ["rom", "gb", "gbc"];
// #[cfg(feature = "save_state")]
// /// List of preferred extensions for `save state` file
// pub const PREFERRED_SAVE_STATE_EXT: [&str; 1] = [SAVE_STATE_EXT];

pub const MENU_BAR_SIZE: f32 = 30.;

pub const AUDIO_BUFFER_SIZE: usize = 2048;
