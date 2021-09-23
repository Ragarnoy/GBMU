use sdl2::filesystem::pref_path;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

const SETTINGS_FILE: &str = "settings";
const SETTINGS_FORMAT: &str = "yaml";

fn settings_path() -> PathBuf {
    let mut settings_path = PathBuf::new();
    if let Ok(path) = pref_path("", "GBMU") {
        settings_path.push(path)
    } else {
        log::warn!("No settings folder found, using app folder");
    };
    settings_path.push(SETTINGS_FILE);
    settings_path.set_extension(SETTINGS_FORMAT);
    settings_path
}

pub fn load() -> Option<gb_joypad::Config> {
    let settings_path = settings_path();
    let mut content = String::new();
    match OpenOptions::new().read(true).open(&settings_path) {
        Ok(mut file) => match file.read_to_string(&mut content) {
            Ok(_) => match serde_yaml::from_str::<gb_joypad::Config>(&content) {
                Ok(input_conf) => Some(input_conf),
                Err(err) => {
                    log::error!("failed to parse config at {:?}: {}", settings_path, err);
                    None
                }
            },
            Err(err) => {
                log::error!("failed to read config at {:?}: {}", settings_path, err);
                None
            }
        },
        Err(err) => {
            log::error!("failed to open config at {:?}: {}", settings_path, err);
            None
        }
    }
}

pub fn save(conf: gb_joypad::Config) {
    let settings_path = settings_path();
    match std::fs::File::create(&settings_path) {
        Ok(mut setting_file) => match serde_yaml::to_string(&conf) {
            Ok(conf_value) => {
                if let Err(err) = setting_file.write_all(conf_value.as_bytes()) {
                    log::error!(
                        "Failed to save inputs settings at {:?}: {}",
                        settings_path,
                        err
                    );
                }
            }
            Err(err) => log::error!(
                "Failed to save inputs settings at {:?}: {}",
                settings_path,
                err
            ),
        },
        Err(err) => log::error!("Failed to save settings at {:?}: {}", settings_path, err),
    }
}
