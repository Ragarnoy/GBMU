use crate::InputType;
use sdl2::keyboard::Scancode;
use serde::{
    de::{Error, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

pub struct Config {
    mapping: HashMap<Scancode, InputType>,
}

impl Config {
    pub fn from_mapping(mapping: HashMap<Scancode, InputType>) -> Self {
        Config { mapping }
    }

    pub fn mapping(self) -> HashMap<Scancode, InputType> {
        self.mapping
    }
}

impl Serialize for Config {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.mapping.len()))?;
        for (k, v) in self.mapping.iter() {
            map.serialize_entry(v, k.name())?;
        }
        map.end()
    }
}

struct ConfigVisitor {
    marker: PhantomData<fn() -> Config>,
}

impl ConfigVisitor {
    fn new() -> Self {
        ConfigVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for ConfigVisitor {
    type Value = Config;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("inputs settings")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut conf = Config {
            mapping: HashMap::with_capacity(8),
        };
        while let Some((value, key)) = access.next_entry()? {
            conf.mapping.insert(
                Scancode::from_name(key)
                    .ok_or_else(|| M::Error::custom(format!("Unrecognised key name: {}", key)))?,
                value,
            );
        }

        Ok(conf)
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ConfigVisitor::new())
    }
}
