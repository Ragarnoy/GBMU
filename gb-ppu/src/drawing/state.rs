use super::Mode;

pub struct State {
    mode: Mode,
    line: u32,
    step: u32,
}

impl State {
    const LINE_COUNT: u32 = 154;
    const STEP_COUNT: u32 = 456;

    const DRAW_END: u32 = 143;
    const HBLANK_MIN_START: u32 = 252;

    const VBLANK_START: u32 = 144;

    pub fn new() -> Self {
        State {
            mode: Mode::OAMFetch,
            line: 0,
            step: 0,
        }
    }

    pub fn update(&mut self) {
        match self {
            Mode::HBlank => self.update_hblank(),
            Mode::VBlank => self.update_vblank(),
            _ => log::error!("unimplemented"),
        }
        self.step = (self.step + 1) % STEP_COUNT;
        if self.step == 0 {
            self.line = (self.line + 1) % LINE_COUNT;
        }
    }

    fn update_hblank(&mut self) {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => {
                log::error!("HBlank reached on VBlank period")
            }
            (_, step) if step < Self::HBLANK_MIN_START => {
                log::error!("HBlank reached on OAMFetch/PixelDrawing period")
            }
            (line, STEP_COUNT - 1) => if line == Self::VBLANK_START - 1 {
                self.mode = Mode::VBlank;
            } else {
                self.mode = Mode::OAMFetch;
            },
            _ => {}
        }
    }

    fn update_vblank(&mut self) {
        match (self.line, self.step) {
            (line, _) if line < Self::VBLANK_START => log::error!("VBlank reached on draw line"),
            (Self::LINE_COUNT - 1, STEP_COUNT - 1) => {
                self::mode = Mode::OAMFetch
            },
            _ => {}
        }
    }
}

fn leave_hblank(line: u32) -> Mode {
    match line {
        0..=Mode::DRAW_END => Mode::OAMFetch,
        Mode::VBLANK_START => Mode::VBlank,
        l => {
            log::error!("HBlank ran on VBlank period");
            if l >= Mode::LINE_COUNT {
                Mode::OAMFetch
            } else {
                Mode::VBlank
            }
        }
    }
}
