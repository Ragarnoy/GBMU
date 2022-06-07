use crate::bios_configuration::BiosConfiguration;
use gb_joypad::Config;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{cell::RefCell, path::Path, rc::Rc};

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub bios: BiosConfiguration,
    #[serde(
        serialize_with = "serialize_joypad_config",
        deserialize_with = "deserialize_joypad_config"
    )]
    pub input: Rc<RefCell<Config>>,
}

fn serialize_joypad_config<S>(
    config: &Rc<RefCell<Config>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let config = config.borrow().clone();

    config.serialize(serializer)
}

fn deserialize_joypad_config<'de, D>(deserializer: D) -> Result<Rc<RefCell<Config>>, D::Error>
where
    D: Deserializer<'de>,
{
    let config = Config::deserialize(deserializer)?;

    Ok(Rc::new(RefCell::new(config)))
}

impl Configuration {
    pub fn load_from_default_config_file() -> Self {
        Self::load_form_config_file(crate::path::main_config_file())
    }
    pub fn load_form_config_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        match std::fs::File::open(path) {
            Ok(file) => serde_yaml::from_reader(file).unwrap_or_else(|e| {
                log::error!("failed to parse main config file: {e}");
                Configuration::default()
            }),
            Err(err) => {
                log::error!("cannot open main config file: {err}");
                Configuration::default()
            }
        }
    }
}
