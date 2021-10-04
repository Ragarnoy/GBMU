use crate::error::{PPUError, PPUResult};
use std::convert::TryFrom;

pub enum Mode {
    HBlank,
    VBlank,
    OAMFetch,
    PixelDrawing,
}

impl Mode {
    const SCANLINE_CYCLE_COUNT: u32 = 456;
    const SCANLINE_COUNT: u8 = 154;
    const DRAW_END: u8 = 143;
    const VBLANK_START: u8 = 144;

    pub fn update(&mut self, line: u8, line_cycle: u32) {
        match self {
            Mode::HBlank => self.update_hblank(line, line_cycle),
            Mode::VBlank => self.update_vblank(line, line_cycle),
            _ => log::error!("unimplemented"),
        }
    }

    fn update_hblank(&mut self, line: u8, line_cycle: u32) {
        match line_cycle {
            0..=251 => log::error!("HBlank reached too early"),
            c @ 252..=Self::SCANLINE_CYCLE_COUNT => {
                if c == Self::SCANLINE_CYCLE_COUNT {
                    *self = leave_hblank(line);
                }
            }
            _ => {
                log::error!("HBlank reached too late");
                *self = leave_hblank(line);
            }
        }
    }

    fn update_vblank(&mut self, line: u8, line_cycle: u32) {
        match line {
            l @ Self::VBLANK_START..=Self::SCANLINE_COUNT => {
                if l == Self::SCANLINE_COUNT && line_cycle >= Self::SCANLINE_CYCLE_COUNT {
                    *self = Mode::OAMFetch;
                    if line_cycle > Self::SCANLINE_CYCLE_COUNT {
                        log::error!("VBlank reached too late on scanline");
                    }
                }
            }
            0..=Self::DRAW_END => log::error!("VBlank reached too early"),
            _ => {
                log::error!("VBlank reached too late");
                *self = Mode::OAMFetch;
            }
        }
    }
}

fn leave_hblank(line: u8) -> Mode {
    match line {
        0..=Mode::DRAW_END => Mode::OAMFetch,
        Mode::VBLANK_START => Mode::VBlank,
        l => {
            log::error!("HBlank ran on VBlank period");
            if l >= Mode::SCANLINE_COUNT {
                Mode::OAMFetch
            } else {
                Mode::VBlank
            }
        }
    }
}

impl TryFrom<u8> for Mode {
    type Error = PPUError;

    fn try_from(byte: u8) -> PPUResult<Mode> {
        match byte {
            0 => Ok(Mode::HBlank),
            1 => Ok(Mode::VBlank),
            2 => Ok(Mode::OAMFetch),
            3 => Ok(Mode::PixelDrawing),
            _ => Err(PPUError::OutOfBound {
                value: byte as usize,
                min_bound: 0,
                max_bound: 4,
            }),
        }
    }
}

impl From<Mode> for u8 {
    fn from(mode: Mode) -> u8 {
        match mode {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAMFetch => 2,
            Mode::PixelDrawing => 3,
        }
    }
}
