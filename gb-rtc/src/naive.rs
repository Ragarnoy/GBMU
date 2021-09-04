use crate::{ReadRtcRegisters, WriteRtcRegisters, DAY, HOUR, MINUTE};
use std::time::Instant;

#[derive(PartialEq, Eq, Debug)]
pub struct Naive {
    timestamp: u32,
    clock: Option<Instant>,
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
        (self.upper_days() as u8)
            | ((self.halted() as u8) << 6)
            | ((self.day_counter_carry() as u8) << 7)
    }
}

impl std::ops::Add<std::time::Duration> for Naive {
    type Output = Self;

    fn add(self, rhs: std::time::Duration) -> Self::Output {
        self + rhs.as_secs() as u32
    }
}

impl std::ops::Add<u32> for Naive {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self::Output::new(self.timestamp + rhs)
    }
}

impl std::ops::Sub<u32> for &Naive {
    type Output = Naive;

    fn sub(self, rhs: u32) -> Self::Output {
        Self::Output::new(self.timestamp - rhs)
    }
}

impl WriteRtcRegisters for Naive {
    fn set_seconds(&mut self, seconds: u8) {
        self.timestamp = self.timestamp - self.seconds() as u32 + (seconds % 60) as u32;
    }

    fn set_minutes(&mut self, minutes: u8) {
        self.timestamp =
            self.timestamp - self.minutes() as u32 * MINUTE + (minutes % 60) as u32 * MINUTE;
    }

    fn set_hours(&mut self, hours: u8) {
        self.timestamp = self.timestamp - self.hours() as u32 * HOUR + (hours % 24) as u32 * HOUR;
    }

    fn set_lower_days(&mut self, ldays: u8) {
        self.timestamp = self.timestamp - self.lower_days() as u32 * DAY + ldays as u32 * DAY;
    }

    fn set_upper_days(&mut self, udays: bool) {
        if udays {
            self.timestamp |= 0x100 * DAY;
        }
    }

    fn set_halted(&mut self, halted: bool) {
        if halted != self.halted() {
            if halted {
                self.clock = None;
            } else {
                self.clock = Some(Instant::now());
            }
        }
    }

    fn set_day_counter_carry(&mut self, carry: bool) {
        if carry {
            self.timestamp |= 0x200 * DAY;
        }
    }

    fn set_control(&mut self, control: u8) {
        self.set_upper_days((control & 1) == 1);
        self.set_halted((control & 0b100_0000) == 0b100_0000);
        self.set_day_counter_carry((control & 0b1000_0000) == 0b1000_0000);
    }
}

#[cfg(test)]
mod test_contructor {
    use super::Naive;
    use crate::{constant::DAY, ReadRtcRegisters};

    #[test]
    fn default() {
        assert_eq!(
            Naive::default(),
            Naive {
                timestamp: 0,
                clock: None
            }
        );
    }

    #[test]
    fn days() {
        assert!(Naive::from_days_opt(511).is_some());
        let date = Naive::from_days(25);
        assert_eq!(date.days(), 25);
        assert_eq!(date.lower_days(), 25);
        assert_eq!(date.timestamp, 25 * DAY);
    }

    #[test]
    #[should_panic]
    fn overflow_days() {
        Naive::from_days(512);
    }
}

#[cfg(test)]
mod test_read_regs {
    use super::Naive;
    use crate::{constant::DAY, ReadRtcRegisters, WriteRtcRegisters};

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
        date.clock = Some(std::time::Instant::now());
        assert!(!date.halted());
    }

    #[test]
    fn day_counter_carry() {
        let date = Naive::from_days(511);

        assert!(!date.day_counter_carry());

        let date = date + std::time::Duration::from_secs((2 * DAY) as u64);
        assert!(date.day_counter_carry());
    }

    #[test]
    fn control() {
        let mut date = Naive::from_days(0xFF);

        assert_eq!(date.control(), 0b100_0000);
        date.clock = Some(std::time::Instant::now());
        assert_eq!(date.control(), 0);

        let date = date + std::time::Duration::from_secs(DAY as u64);
        assert_eq!(date.control(), 0b100_0001);
        let mut date = date + std::time::Duration::from_secs((0x200 * DAY) as u64);
        assert_eq!(date.control(), 0b1100_0001);
        date.clock = Some(std::time::Instant::now());
        assert_eq!(date.control(), 0b1000_0001);
    }
}
