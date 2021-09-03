use crate::{ReadRtcRegisters, WriteRtcRegisters, DAY, HOUR, MINUTE};
use std::{cell::RefCell, time::Instant};

pub struct Naive {
    timestamp: u32,
    clock: Option<RefCell<Instant>>,
}

impl Naive {
    pub fn new(timestamp: u32) -> Self {
        Self {
            timestamp,
            clock: None,
        }
    }

    pub fn from_days(days: u16) -> Self {
        Self::from_days_opt(days).unwrap()
    }

    pub fn from_days_opt(days: u16) -> Option<Self> {
        if days <= 0x1FF {
            Some(Self {
                timestamp: days as u32 * DAY,
                clock: None,
            })
        } else {
            None
        }
    }

    pub fn and_hms(&self, hours: u8, minutes: u8, seconds: u8) -> Self {
        self.and_hms_opt(hours, minutes, seconds).unwrap()
    }

    pub fn and_hms_opt(&self, hours: u8, minutes: u8, seconds: u8) -> Option<Self> {
        if hours > 23 || minutes > 59 || seconds > 59 {
            None
        } else {
            Some(Self::new(
                self.days() as u32 + hours as u32 * HOUR + minutes as u32 * MINUTE + seconds as u32,
            ))
        }
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new(0)
    }
}

impl ReadRtcRegisters for Naive {
    fn seconds(&self) -> u8 {
        (self.timestamp % MINUTE) as u8
    }

    fn minutes(&self) -> u8 {
        ((self.timestamp % HOUR) / MINUTE) as u8
    }

    fn hours(&self) -> u8 {
        ((self.timestamp % DAY) / HOUR) as u8
    }

    fn days(&self) -> u16 {
        (self.timestamp / DAY) as u16
    }

    fn lower_days(&self) -> u8 {
        (self.days() & 0xFF) as u8
    }

    fn upper_days(&self) -> bool {
        (self.days() & 0x100) == 0x100
    }

    fn halted(&self) -> bool {
        self.clock.is_none()
    }

    fn day_counter_carry(&self) -> bool {
        todo!()
    }

    fn control(&self) -> u8 {
        todo!()
    }
}
