use super::{Lock, Lockable};
use crate::error::{PPUError, PPUResult};
use crate::Sprite;
use std::collections::BTreeMap;
use std::convert::TryInto;

/// Contains operations to collect sprites from memory.
pub struct Oam {
    data: [u8; Oam::SIZE as usize],
    lock: Option<Lock>,
}

impl Oam {
    pub const SPRITE_COUNT: usize = 40;
    pub const SIZE: usize = Self::SPRITE_COUNT * Sprite::SIZE;

    pub fn new() -> Self {
        Oam {
            data: [0x00; Self::SIZE as usize],
            lock: None,
        }
    }

    pub fn read(&self, addr: usize) -> PPUResult<u8> {
        if addr < Self::SIZE {
            Ok(self.data[addr])
        } else {
            Err(PPUError::OutOfBound {
                value: addr,
                min_bound: 0,
                max_bound: Self::SIZE,
            })
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) -> PPUResult<()> {
        if addr < Self::SIZE {
            self.data[addr] = value;
            Ok(())
        } else {
            Err(PPUError::OutOfBound {
                value: addr,
                min_bound: 0,
                max_bound: Self::SIZE,
            })
        }
    }

    fn read_sprite(&self, pos: usize) -> PPUResult<Sprite> {
        if pos > Self::SPRITE_COUNT - 1 {
            return Err(PPUError::OutOfBound {
                value: pos,
                min_bound: 0,
                max_bound: Self::SPRITE_COUNT - 1,
            });
        }
        let index = pos * Sprite::SIZE;
        let bytes: [u8; Sprite::SIZE] = self.data[index..index + Sprite::SIZE]
            .try_into()
            .expect("failed to map sprite's bytes into array");
        Ok(Sprite::from(bytes))
    }

    /// Return all the sprites contained in memory.
    pub fn collect_all_sprites(&self) -> PPUResult<[Sprite; Self::SPRITE_COUNT]> {
        let mut sprites = [Sprite::new(); Self::SPRITE_COUNT];
        for (i, sprite) in sprites.iter_mut().enumerate() {
            *sprite = self.read_sprite(i)?;
        }
        Ok(sprites)
    }

    /// Return the first 10 obj on the x axis that overlap a line of the viewport.
    ///
    /// ### Parameters
    ///  - **line**: the y coordinate of the line. 0 is the top of the viewport.
    ///  - **size_16**: the bit 2 flag from Control indicating if sprites are 8(*false*) or 16(*true*) pixels high.
    pub fn scan_line_sprite(&self, line: u8, size_16: bool) -> PPUResult<Vec<Sprite>> {
        let mut selected_obj = BTreeMap::new();
        let all_obj = self.collect_all_sprites()?;
        let scanline = line + 16;
        for obj in all_obj {
            let top = obj.y_pos();
            let bot = top + if size_16 { 15 } else { 7 };
            if scanline >= top && scanline <= bot {
                selected_obj.insert(obj.x_pos(), obj);
            }
        }
        Ok(selected_obj.values().take(10).copied().collect())
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

impl From<[u8; Oam::SIZE]> for Oam {
    fn from(bytes: [u8; Oam::SIZE]) -> Oam {
        Oam {
            data: bytes,
            lock: None,
        }
    }
}

impl From<Oam> for [u8; Oam::SIZE] {
    fn from(mem: Oam) -> [u8; Oam::SIZE] {
        mem.data
    }
}

impl Lockable for Oam {
    fn lock(&mut self, owner: Lock) {
        self.lock = Some(owner);
    }

    fn unlock(&mut self) {
        self.lock = None;
    }

    fn get_lock(&self) -> Option<Lock> {
        self.lock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_with_obj() {
        let oam: Oam =
            (*include_bytes!("../../examples/memory dumps/oam/Legend_of_Zelda_link_Awaking.dmp"))
                .into();
        let line = 32;
        let scanned_line = oam
            .scan_line_sprite(line, true)
            .expect("Should not contains sprites out of memory");
        assert_eq!(scanned_line.len(), 4);
        assert!(
            scanned_line[0].x_pos() <= scanned_line[1].x_pos(),
            "not ordered"
        );
        assert!(
            scanned_line[1].x_pos() <= scanned_line[2].x_pos(),
            "not ordered"
        );
        assert!(
            scanned_line[2].x_pos() <= scanned_line[3].x_pos(),
            "not ordered"
        );
    }

    #[test]
    fn line_with_more_than_10() {
        let oam: Oam = (*include_bytes!(
            "../../examples/memory dumps/oam/[MODDED]-Legend_of_Zelda_link_Awaking.dmp"
        ))
        .into();
        let line = 30;
        let scanned_line = oam
            .scan_line_sprite(line, true)
            .expect("Should not contains sprites out of memory");
        assert_eq!(scanned_line.len(), 10);
        let mut previous = 0x00;
        for obj in scanned_line {
            assert!(obj.x_pos() >= previous, "not ordered");
            assert!(obj.x_pos() <= 0x0A, "did not removed last obj of line");
            previous = obj.x_pos();
        }
    }
}
