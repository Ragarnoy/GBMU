use crate::error::{Error, PPUResult};
use crate::object::Object;
use std::convert::TryInto;

/// Contains operations to collect objects from memory.
pub struct Oam {
    data: [u8; Oam::SIZE as usize],
}

impl Oam {
    pub const OBJECT_COUNT: usize = 40;
    pub const SIZE: usize = Self::OBJECT_COUNT * Object::SIZE;

    pub fn new() -> Self {
        Oam {
            data: [0x00; Self::SIZE as usize],
        }
    }

    fn read_object(&self, pos: usize) -> PPUResult<Object> {
        if pos > Self::OBJECT_COUNT - 1 {
            return Err(Error::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: Self::OBJECT_COUNT - 1,
            });
        }
        let index = pos * Object::SIZE;
        let bytes: [u8; Object::SIZE] = self.data[index..index + Object::SIZE]
            .try_into()
            .expect("failed to map object's bytes into array");
        Ok(Object::from(bytes))
    }

    pub fn collect_all_objects(&self) -> PPUResult<[Object; Self::OBJECT_COUNT]> {
        let mut objects = [Object::new(); Self::OBJECT_COUNT];
        for (i, object) in objects.iter_mut().enumerate() {
            *object = self.read_object(i)?;
        }
        Ok(objects)
    }

    pub fn overwrite(&mut self, data: &[u8; Self::SIZE]) {
        self.data = *data;
    }
}

impl Default for Oam {
    fn default() -> Oam {
        Oam::new()
    }
}
