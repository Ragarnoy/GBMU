use super::Mode;
use crate::memory::Lock;
use crate::registers::LcdReg;
use gb_bus::Bus;
use std::cell::RefMut;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone)]
pub struct State {
    cgb_enabled: bool,
    mode: Mode,
    line: u8,
    step: u16,
    pixel_drawn: u8,
}

impl Default for State {
    fn default() -> Self {
        Self::new(false)
    }
}

const INTERRUPT_FLAG: u16 = 0xFF0F;
const INTERRUPT_STAT_BIT: u8 = 0b10;
const INTERRUPT_VBLANK_BIT: u8 = 0b01;

impl State {
    const LINE_COUNT: u8 = 154;
    pub const LAST_LINE: u8 = Self::LINE_COUNT - 1;
    const VBLANK_START: u8 = 144;

    const PIXEL_DRAWING_START: u16 = 80;
    const LAST_OAM_FETCH_STEP: u16 = Self::PIXEL_DRAWING_START - 1;
    const HBLANK_MIN_START: u16 = 248;
    const HBLANK_MAX_START: u16 = 371;
    const STEP_COUNT: u16 = 456;
    pub const LAST_STEP: u16 = Self::STEP_COUNT - 1;

    pub fn new(cgb_enabled: bool) -> Self {
        State {
            cgb_enabled,
            mode: Mode::HBlank,
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
    pub fn set_step(&mut self, v: u16) {
        self.step = v;
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
            (line, step)
                if line > Self::VBLANK_START || (line == Self::VBLANK_START && step > 0) =>
            {
                log::error!("HBlank reached on VBlank period")
            }
            (_, step) if step < Self::HBLANK_MIN_START && step > 0 => {
                log::error!("HBlank reached on OAMFetch/PixelDrawing period")
            }
            (line, 0) => {
                if line == Self::VBLANK_START {
                    self.mode = Mode::VBlank;
                    log::trace!(
                        "start VBlank (mode 1) at line {}, step {}, {} pixel drawn",
                        self.line,
                        self.step,
                        self.pixel_drawn()
                    );
                } else {
                    self.mode = Mode::OAMFetch;
                    log::trace!(
                        "start OAM fetch (mode 2) at line {}, step {}, {} pixel drawn",
                        self.line,
                        self.step,
                        self.pixel_drawn()
                    );
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
                log::trace!(
                    "start OAM fetch (mode 2) at line {}, step {}, {} pixel drawn",
                    self.line,
                    self.step,
                    self.pixel_drawn()
                );
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
                log::trace!(
                    "start pixel drawing (mode 3) at line {}, step {}, {} pixel drawn",
                    self.line,
                    self.step,
                    self.pixel_drawn()
                );
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
            (line, step) if step >= Self::HBLANK_MAX_START => {
                self.mode = Mode::HBlank;
                log::trace!(
                    "start HBlank (mode 0) at line {}, step {}, {} pixel drawn",
                    self.line,
                    self.step,
                    self.pixel_drawn()
                );
                log::error!(
                    "PixelDrawing reached on HBlank period at: l{}; s{}; p{}",
                    line,
                    step,
                    self.pixel_drawn()
                );
                return true;
            }
            (_, step) if step >= Self::HBLANK_MIN_START => {
                if self.pixel_drawn >= 160 {
                    self.mode = Mode::HBlank;
                    log::trace!(
                        "start HBlank (mode 0) at line {}, step {}, {} pixel drawn",
                        self.line,
                        self.step,
                        self.pixel_drawn()
                    );
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
        &mut self,
        mut lcd_reg: RefMut<LcdReg>,
        adr_bus: &mut dyn Bus<u8>,
        state_updated: bool,
        line_updated: bool,
    ) {
        if line_updated {
            lcd_reg.scrolling.ly = if !self.cgb_enabled && self.line == Self::LAST_LINE {
                0
            } else {
                self.line
            };
            let lyc_eq_ly = lcd_reg.scrolling.ly == lcd_reg.scrolling.lyc;
            lcd_reg.stat.set_lyc_eq_ly(lyc_eq_ly);
        }

        if state_updated {
            lcd_reg.stat.set_mode(self.mode);
        }

        let update_vblank = state_updated && self.mode == Mode::VBlank;
        let update_mode = state_updated
            && ((self.mode == Mode::OAMFetch && lcd_reg.stat.mode_2_interrupt())
                || (self.mode == Mode::VBlank && lcd_reg.stat.mode_1_interrupt())
                || (self.mode == Mode::HBlank && lcd_reg.stat.mode_0_interrupt()));
        let update_lyc_eq =
            line_updated && lcd_reg.stat.lyc_eq_ly_interrupt() && lcd_reg.stat.lyc_eq_ly();
        let update_stat = update_mode || update_lyc_eq;

        if update_vblank || update_stat {
            let mut interrupts_val = adr_bus
                .read(INTERRUPT_FLAG, Some(Lock::Ppu))
                .expect("Failed to read interrupt value for ppu interrupt");
            if update_vblank {
                interrupts_val |= INTERRUPT_VBLANK_BIT;
            }
            if update_stat {
                interrupts_val |= INTERRUPT_STAT_BIT;
            }
            if let Err(err) = adr_bus.write(INTERRUPT_FLAG, interrupts_val, Some(Lock::Ppu)) {
                log::error!(
                    "Failed to write interrupt value for ppu interrupt: {:?}",
                    err
                )
            }
        }
    }
}
