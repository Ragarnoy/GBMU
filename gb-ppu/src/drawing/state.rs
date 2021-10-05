use super::Mode;

pub struct State {
    mode: Mode,
    line: u32,
    step: u32,
    pixel_drawn: u32,
}

impl State {
    const LINE_COUNT: u32 = 154;
    const STEP_COUNT: u32 = 456;

    const HBLANK_MIN_START: u32 = 252;
    const HBLANK_MAX_START: u32 = 369;
    const VBLANK_START: u32 = 144;
    const PIXEL_DRAWING_START: u32 = 80;

    pub fn new() -> Self {
        State {
            mode: Mode::OAMFetch,
            line: 0,
            step: 0,
            pixel_drawn: 0,
        }
    }

    pub fn update(&mut self) {
        match self {
            Mode::HBlank => self.update_hblank(),
            Mode::VBlank => self.update_vblank(),
            Mode::OAMFetch => self.update_oam_fetch(),
            Mode::PixelDrawing => self.update_pixel_drawing(),
        }
        self.step = (self.step + 1) % STEP_COUNT;
        if self.step == 0 {
            self.line = (self.line + 1) % LINE_COUNT;
            self.pixel_drawn = 0;
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
            (Self::LINE_COUNT - 1, Self::STEP_COUNT - 1) => {
                self::mode = Mode::OAMFetch
            },
            _ => {}
        }
    }

    fn update_oam_fetch(&mut self) {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => log::error!("OAMFetch reached on VBlank period"),
            (_, step) if step >= Self::PIXEL_DRAWING_START => log::error!("OAMFetch reached on PixelDrawing period"),
            (_, Self::PIXEL_DRAWING_START - 1) => {
                self::mode = Mode::PixelDrawing
            },
            _ => {}
        }
    }

    fn update_pixel_drawing(&mut self) {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => log::error!("OAMFetch reached on VBlank period"),
            (_, step) if step < Self::PIXEL_DRAWING_START => log::error!("PixelDrawing reached on OAMFetch period"),
            (_, step) if step >= Self::HBLANK_MAX_START => log::error!("PixelDrawing reached on HBlank period"),
            (_, step) if step >= Self::HBLANK_MIN_START => {
                if self.pixel_drawn >= 160 {
                    self.mode = Mode::HBlank;
                    if self.pixel_drawn > 160 {
                        log::error!("Too many pixel drawn before switching to HBlank");
                    }
                }
            },
            _ => {}
        }
    }
}
