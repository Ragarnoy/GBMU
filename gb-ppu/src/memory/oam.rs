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

    pub fn scan_line_object(&self, line: u8, size_16: bool) -> PPUResult<Vec<Object>> {
        let mut selected_obj = Vec::with_capacity(10);
        let all_obj = self.collect_all_objects()?;
        let scanline = line + 16;
        for obj in all_obj {
            let top = obj.y_pos();
            let bot = top + if size_16 { 15 } else { 7 };
            if scanline >= top && scanline <= bot {
                selected_obj.push(obj);
                if selected_obj.len() == 10 {
                    return Ok(selected_obj);
                }
            }
        }
        Ok(selected_obj)
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

impl From<&[u8; Oam::SIZE]> for Oam {
    fn from(bytes: &[u8; Oam::SIZE]) -> Oam {
        Oam { data: *bytes }
    }
}

impl From<&Oam> for [u8; Oam::SIZE] {
    fn from(mem: &Oam) -> [u8; Oam::SIZE] {
        mem.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_with_obj() {
        let oam: Oam =
            include_bytes!("../../examples/memory dumps/oam/Legend_of_Zelda_link_Awaking.dmp")
                .into();
        let line = 32;
        let scanned_line = oam
            .scan_line_object(line, true)
            .expect("Should not contains objects out of memory");
        assert_eq!(scanned_line.len(), 4);
    }

    #[test]
    fn line_with_more_than_10() {
        let oam: Oam = include_bytes!(
            "../../examples/memory dumps/oam/[MODDED]-Legend_of_Zelda_link_Awaking.dmp"
        )
        .into();
        let line = 30;
        let scanned_line = oam
            .scan_line_object(line, true)
            .expect("Should not contains objects out of memory");
        assert_eq!(scanned_line.len(), 10);
    }
}
