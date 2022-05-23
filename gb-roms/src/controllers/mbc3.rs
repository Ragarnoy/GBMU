use gb_rtc::{Naive, ReadRtcRegisters};

use crate::controllers::RAM_BANK_SIZE;
use crate::Header;

use super::save::{Full as Complete, Partial as Incomplete, SaveState, StateError};
use super::{Controller, ROM_BANK_SIZE};

pub fn new_controller(header: Header) -> Box<Mbc3> {
    use crate::header::CartridgeType::{Mbc3TimerBattery, Mbc3TimerRamBattery2};

    Box::new(Mbc3 {
        rom_banks: header.rom_size.get_bank_amounts(),
        ram_banks: header.ram_size.get_bank_amounts(),
        clock: if matches!(
            header.cartridge_type,
            Mbc3TimerBattery | Mbc3TimerRamBattery2
        ) {
            Some(Naive::default())
        } else {
            None
        },
        ..Default::default()
    })
}

pub struct Mbc3 {
    rom_banks: usize,
    ram_banks: usize,
    clock: Option<Naive>,
    external_gate: bool,
    rom_bank: u8,
    external_selector: u8,

    rtc_regs: RTCRegs,
    last_written_byte: Option<u8>,
}

impl Default for Mbc3 {
    fn default() -> Self {
        Self {
            rom_banks: 0,
            ram_banks: 0,
            clock: None,
            external_gate: false,
            rom_bank: 1,
            external_selector: 0,
            last_written_byte: None,
            rtc_regs: RTCRegs::default(),
        }
    }
}

impl Mbc3 {
    fn may_latch_clock_data(&mut self, v: u8) {
        if self.last_written_byte == Some(0) && v == 1 {
            self.latch_clock_data();
        } else {
            self.last_written_byte = Some(v);
        }
    }

    fn latch_clock_data(&mut self) {
        if let Some(ref clock) = self.clock {
            self.rtc_regs = clock.into();
        }
    }

    fn should_map_rtc_regs(&self) -> bool {
        self.external_selector >= 0x8 && self.external_selector <= 0xC
    }

    fn read_rtc_regs(&self) -> u8 {
        match self.external_selector {
            0x8 => self.rtc_regs.seconds,
            0x9 => self.rtc_regs.minutes,
            0xa => self.rtc_regs.hours,
            0xb => self.rtc_regs.lower_day_counter,
            0xc => self.rtc_regs.upper_day_counter,
            _ => 0xff,
        }
    }

    fn write_rtc_regs(&mut self, v: u8) {
        match self.external_selector {
            0x8 => self.rtc_regs.seconds = v,
            0x9 => self.rtc_regs.minutes = v,
            0xa => self.rtc_regs.hours = v,
            0xb => self.rtc_regs.lower_day_counter = v,
            0xc => self.rtc_regs.upper_day_counter = v,
            _ => {}
        }
    }
}

impl Controller for Mbc3 {
    fn sizes(&self) -> (usize, Option<usize>) {
        (
            self.rom_banks * ROM_BANK_SIZE,
            if self.ram_banks > 0 {
                Some(self.ram_banks * RAM_BANK_SIZE)
            } else {
                None
            },
        )
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        match (addr >> 8) & 0xff {
            0x00..=0x1f => {
                self.external_gate = v & 0xf == 0xa;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("external_gate={}", self.external_gate);
            }
            0x20..=0x3f => {
                self.rom_bank = v & 0x7f;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("rom_bank={}", self.rom_bank);
            }
            0x40..=0x5f => {
                self.external_selector = v & 0xf;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("external_selector={}", self.external_selector);
            }
            0x60..=0x7f => {
                self.may_latch_clock_data(v);
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("update rtc_regs: {:x?}", self.rtc_regs);
            }
            _ => {}
        }
    }

    fn ram_enabled(&self) -> bool {
        self.ram_banks > 0 && self.external_gate
    }

    fn override_read_ram(&self, _addr: u16) -> Option<u8> {
        if self.external_gate && self.should_map_rtc_regs() {
            Some(self.read_rtc_regs())
        } else {
            None
        }
    }

    fn override_write_ram(&mut self, v: u8, _addr: u16) -> Option<()> {
        if self.external_gate && self.should_map_rtc_regs() {
            self.write_rtc_regs(v);
            Some(())
        } else {
            None
        }
    }

    fn offset_ram_addr(&self, addr: u16) -> usize {
        let bank = (self.external_selector & 3) as usize;
        ((bank % self.ram_banks) * RAM_BANK_SIZE) | (addr & 0x1fff) as usize
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        let bank = if addr <= 0x3fff {
            0
        } else {
            self.rom_bank as usize
        };
        ((bank % self.rom_banks) * ROM_BANK_SIZE) | (addr & 0x3fff) as usize
    }
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct RTCRegs {
    seconds: u8,
    minutes: u8,
    hours: u8,
    lower_day_counter: u8,
    upper_day_counter: u8,
}

impl<RTC: ReadRtcRegisters> From<&RTC> for RTCRegs {
    fn from(rtc: &RTC) -> Self {
        Self {
            seconds: rtc.seconds(),
            minutes: rtc.minutes(),
            hours: rtc.hours(),
            lower_day_counter: rtc.lower_days(),
            upper_day_counter: rtc.control(),
        }
    }
}

impl SaveState for Mbc3 {
    fn serialize(&self) -> Complete {
        Complete::Mbc3(Full::from(self))
    }

    fn load(&mut self, state: Complete) -> Result<(), StateError> {
        if let Complete::Mbc3(state) = state {
            self.rtc_regs = state.rtc_regs;
            self.external_gate = state.external_gate;
            self.rom_bank = state.rom_bank;
            self.external_selector = state.external_selector;
            self.last_written_byte = state.last_written_byte;

            self.load_partial(Incomplete::Mbc3(state.partial))
        } else {
            Err(StateError::WrongType {
                expected: "mbc3",
                got: state.id(),
            })
        }
    }

    fn serialize_partial(&self) -> Incomplete {
        Incomplete::Mbc3(Partial {
            clock: self.clock.clone(),
        })
    }

    fn load_partial(&mut self, state: Incomplete) -> Result<(), StateError> {
        if let Incomplete::Mbc3(state) = state {
            self.clock = state.clock;
            Ok(())
        } else {
            Err(StateError::WrongType {
                expected: "mbc3",
                got: state.id(),
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Full {
    partial: Partial,
    rtc_regs: RTCRegs,
    external_gate: bool,
    rom_bank: u8,
    external_selector: u8,
    last_written_byte: Option<u8>,
}

impl From<&Mbc3> for Full {
    fn from(ctl: &Mbc3) -> Self {
        Self {
            partial: Partial {
                clock: ctl.clock.clone(),
            },
            rtc_regs: ctl.rtc_regs,
            external_gate: ctl.external_gate,
            rom_bank: ctl.rom_bank,
            external_selector: ctl.external_selector,
            last_written_byte: ctl.last_written_byte,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Partial {
    clock: Option<Naive>,
}
