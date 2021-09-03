use crate::{ReadRtcRegisters, WriteRtcRegisters, DAY, HOUR, MINUTE};
use std::{cell::RefCell, time::Instant};

#[derive(PartialEq, Eq, Debug)]
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
        (self.timestamp / DAY) as u16 & 0x1FF
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
        self.timestamp > (0x1FF * DAY)
    }

    fn control(&self) -> u8 {
        todo!()
    }
}

impl std::ops::Add<std::time::Duration> for Naive {
    type Output = Self;

    fn add(self, rhs: std::time::Duration) -> Self::Output {
        Self::new(self.timestamp + rhs.as_secs() as u32)
    }
}

#[cfg(test)]
mod test_naive {
    use super::Naive;
    use crate::{constant::DAY, ReadRtcRegisters, WriteRtcRegisters};

    #[test]
    fn default_init() {
        assert_eq!(
            Naive::default(),
            Naive {
                timestamp: 0,
                clock: None
            }
        );
    }

    #[test]
    fn days_init() {
        assert!(Naive::from_days_opt(511).is_some());
        let date = Naive::from_days(25);
        assert_eq!(date.days(), 25);
        assert_eq!(date.lower_days(), 25);
        assert_eq!(date.timestamp, 25 * DAY);
    }

    #[test]
    #[should_panic]
    fn overflow_days_init() {
        Naive::from_days(512);
    }

    #[test]
    fn hours_minutes_seconds() {
        let date = Naive::default().and_hms(5, 3, 2);

        assert_eq!(date.seconds(), 2);
        assert_eq!(date.minutes(), 3);
        assert_eq!(date.hours(), 5);
    }

    #[test]
    fn days() {
        let date = Naive::from_days(356);

        assert!(date.upper_days());
        assert_eq!(date.lower_days(), (356 % 0x100) as u8);
        assert_eq!(date.days(), 356);
    }

    #[test]
    fn halted() {
        let mut date = Naive::default();

        assert!(date.halted());
        date.clock = Some(std::cell::RefCell::new(std::time::Instant::now()));
        assert!(!date.halted());
    }

    #[test]
    fn day_counter_carry() {
        let date = Naive::from_days(511);

        assert!(!date.day_counter_carry());

        let date = date + std::time::Duration::from_secs((2 * DAY) as u64);
        assert!(date.day_counter_carry());
    }
}
