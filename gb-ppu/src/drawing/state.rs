use super::Mode;
use crate::registers::LcdReg;
use gb_bus::Bus;
use std::cell::RefMut;

pub struct State {
    mode: Mode,
    line: u8,
    step: u16,
    pixel_drawn: u8,
}

impl State {
    const LINE_COUNT: u8 = 154;
    pub const LAST_LINE: u8 = Self::LINE_COUNT - 1;
    const VBLANK_START: u8 = 144;

    const PIXEL_DRAWING_START: u16 = 80;
    const LAST_OAM_FETCH_STEP: u16 = Self::PIXEL_DRAWING_START - 1;
    const HBLANK_MIN_START: u16 = 252;
    const HBLANK_MAX_START: u16 = 369;
    const STEP_COUNT: u16 = 456;
    pub const LAST_STEP: u16 = Self::STEP_COUNT - 1;

    pub fn new() -> Self {
        State {
            mode: Mode::OAMFetch,
            line: 0,
            step: 0,
            pixel_drawn: 0,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn line(&self) -> u8 {
        self.line
    }

    pub fn step(&self) -> u16 {
        self.step
    }

    pub fn pixel_drawn(&self) -> u8 {
        self.pixel_drawn
    }

    pub fn draw_pixel(&mut self) {
        self.pixel_drawn += 1;
    }

    pub fn clear_pixel_count(&mut self) {
        self.pixel_drawn = 0;
    }

    pub fn update(&mut self, lcd_reg: Option<RefMut<LcdReg>>, adr_bus: &mut dyn Bus<u8>) {
        let state_updated = match self.mode {
            Mode::HBlank => self.update_hblank(),
            Mode::VBlank => self.update_vblank(),
            Mode::OAMFetch => self.update_oam_fetch(),
            Mode::PixelDrawing => self.update_pixel_drawing(),
        };
        self.step = (self.step + 1) % Self::STEP_COUNT;
        if self.step == 0 {
            self.line = (self.line + 1) % Self::LINE_COUNT;
            self.pixel_drawn = 0;
        }
        if let Some(lcd) = lcd_reg {
            self.update_registers(lcd, adr_bus, state_updated, self.step == 0);
        } else {
            log::error!("PPU state failed to update registers");
        }
    }

    fn update_hblank(&mut self) -> bool {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => {
                log::error!("HBlank reached on VBlank period")
            }
            (_, step) if step < Self::HBLANK_MIN_START => {
                log::error!("HBlank reached on OAMFetch/PixelDrawing period")
            }
            (line, Self::LAST_STEP) => {
                if line == Self::VBLANK_START - 1 {
                    self.mode = Mode::VBlank;
                } else {
                    self.mode = Mode::OAMFetch;
                }
                return true;
            }
            _ => {}
        }
        false
    }

    fn update_vblank(&mut self) -> bool {
        match (self.line, self.step) {
            (line, _) if line < Self::VBLANK_START => log::error!("VBlank reached on draw line"),
            (Self::LAST_LINE, Self::LAST_STEP) => {
                self.mode = Mode::OAMFetch;
                return true;
            }
            _ => {}
        }
        false
    }

    fn update_oam_fetch(&mut self) -> bool {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => {
                log::error!("OAMFetch reached on VBlank period")
            }
            (_, step) if step >= Self::PIXEL_DRAWING_START => {
                log::error!("OAMFetch reached on PixelDrawing period")
            }
            (_, Self::LAST_OAM_FETCH_STEP) => {
                self.mode = Mode::PixelDrawing;
                return true;
            }
            _ => {}
        }
        false
    }

    fn update_pixel_drawing(&mut self) -> bool {
        match (self.line, self.step) {
            (line, _) if line >= Self::VBLANK_START => {
                log::error!("OAMFetch reached on VBlank period")
            }
            (_, step) if step < Self::PIXEL_DRAWING_START => {
                log::error!("PixelDrawing reached on OAMFetch period")
            }
            (_, step) if step >= Self::HBLANK_MAX_START => {
                self.mode = Mode::HBlank;
                log::error!("PixelDrawing reached on HBlank period");
                return true;
            }
            (_, step) if step >= Self::HBLANK_MIN_START => {
                if self.pixel_drawn >= 160 {
                    self.mode = Mode::HBlank;
                    if self.pixel_drawn > 160 {
                        log::error!("Too many pixel drawn before switching to HBlank");
                    }
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn update_registers(
        &self,
        mut lcd_reg: RefMut<LcdReg>,
        adr_bus: &mut dyn Bus<u8>,
        state_updated: bool,
        line_updated: bool,
    ) {
        if line_updated {
            lcd_reg.scrolling.ly = self.line;
            let lyc_eq_ly = self.line == lcd_reg.scrolling.lyc;
            lcd_reg.stat.set_lyc_eq_ly(lyc_eq_ly);
        }

        if state_updated {
            lcd_reg.stat.set_mode(self.mode);
        }

        if state_updated
            && ((self.mode == Mode::OAMFetch && lcd_reg.stat.mode_2_interrupt())
                || (self.mode == Mode::VBlank && lcd_reg.stat.mode_1_interrupt())
                || (self.mode == Mode::HBlank && lcd_reg.stat.mode_0_interrupt()))
            || line_updated && lcd_reg.stat.lyc_eq_ly_interrupt() && lcd_reg.stat.lyc_eq_ly()
        {
            match adr_bus.read(0xFF0F) {
                Ok(interrupts_val) => {
                    if let Err(err) = adr_bus.write(0xFF0F, interrupts_val | 0b0000_0010) {
                        log::error!("Failed to write interrupt value for lcd stat: {:?}", err)
                    }
                }
                Err(err) => {
                    log::error!("Failed to read interrupt value for lcd stat: {:?}", err)
                }
            }
        }
    }
}
