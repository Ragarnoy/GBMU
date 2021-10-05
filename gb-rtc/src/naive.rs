use crate::{
    constant::{DAY, HOUR, MAX_DAYS, MAX_TIME, MINUTE},
    ReadRtcRegisters, WriteRtcRegisters,
};
use std::time::Instant;

#[derive(PartialEq, Eq, Debug)]
pub struct Naive {
    timestamp: u64,
    clock: Option<Instant>,

    day_carry: bool,
}

impl Naive {
    pub fn new(timestamp: u64) -> Self {
        Self {
            timestamp: timestamp % (MAX_TIME + 1),
            clock: None,

            day_carry: timestamp > MAX_TIME,
        }
    }

    pub fn from_days(days: u16) -> Self {
        Self::from_days_opt(days).unwrap()
    }

    pub fn from_days_opt(days: u16) -> Option<Self> {
        if days <= MAX_DAYS as u16 {
            Some(Self::new(days as u64 * DAY))
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
                self.days() as u64 + hours as u64 * HOUR + minutes as u64 * MINUTE + seconds as u64,
            ))
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
            + self
                .clock
                .map(|instant| instant.elapsed().as_secs())
                .unwrap_or(0)
    }

    fn may_reset_instant(&mut self) {
        if self.clock.is_some() {
            self.reset_instant()
        }
    }

    fn reset_instant(&mut self) {
        self.clock.replace(Instant::now());
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new(0)
    }
}

impl ReadRtcRegisters for Naive {
    fn seconds(&self) -> u8 {
        (self.timestamp() % MINUTE) as u8
    }

    fn minutes(&self) -> u8 {
        ((self.timestamp() % HOUR) / MINUTE) as u8
    }

    fn hours(&self) -> u8 {
        ((self.timestamp() % DAY) / HOUR) as u8
    }

    fn days(&self) -> u16 {
        (self.timestamp() / DAY) as u16 & 0x1FF
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
        self.day_carry
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
        self + rhs.as_secs() as u64
    }
}

impl std::ops::Add<u64> for Naive {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::Output::new(self.timestamp() + rhs)
    }
}

impl WriteRtcRegisters for Naive {
    fn set_seconds(&mut self, seconds: u8) {
        self.timestamp = self.timestamp() - self.seconds() as u64 + (seconds % 60) as u64;
        self.may_reset_instant();
    }

    fn set_minutes(&mut self, minutes: u8) {
        self.timestamp =
            self.timestamp() - self.minutes() as u64 * MINUTE + (minutes % 60) as u64 * MINUTE;
        self.may_reset_instant();
    }

    fn set_hours(&mut self, hours: u8) {
        self.timestamp = self.timestamp() - self.hours() as u64 * HOUR + (hours % 24) as u64 * HOUR;
        self.may_reset_instant();
    }

    fn set_lower_days(&mut self, ldays: u8) {
        self.timestamp = self.timestamp() - self.lower_days() as u64 * DAY + ldays as u64 * DAY;
        self.may_reset_instant();
    }

    fn set_upper_days(&mut self, udays: bool) {
        if udays && !self.upper_days() {
            self.timestamp = 0x100 * DAY + self.timestamp();
            self.may_reset_instant();
        } else if !udays && self.upper_days() {
            self.timestamp = self.timestamp() - 0x100 * DAY;
            self.may_reset_instant();
        }
    }

    fn set_halted(&mut self, halted: bool) {
        if halted != self.halted() {
            self.timestamp = self.timestamp();
            if halted {
                self.clock = None;
            } else {
                self.clock = Some(Instant::now());
            }
        }
    }

    fn set_day_counter_carry(&mut self, carry: bool) {
        self.day_carry = carry;
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
                clock: None,

                day_carry: false,
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
    use crate::{constant::DAY, ReadRtcRegisters};

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
        let mut date = date + (0x200 * DAY);
        assert_eq!(date.control(), 0b1100_0001);
        date.clock = Some(std::time::Instant::now());
        assert_eq!(date.control(), 0b1000_0001);
    }
}

#[cfg(test)]
mod test_ops {
    use super::Naive;
    use crate::constant::{DAY, HOUR};

    #[test]
    fn add_u64() {
        let date = Naive::default();

        assert_eq!(date.timestamp, 0);
        let date = date + 24 * DAY;
        assert_eq!(date.timestamp, 24 * DAY);
        let date = date + (DAY + 26 * HOUR);
        assert_eq!(date.timestamp, 26 * DAY + 2 * HOUR);
    }

    #[test]
    fn add_duration() {
        use std::time::Duration;

        let date = Naive::default();

        assert_eq!(date.timestamp, 0);
        let date = date + Duration::from_secs(24 * DAY as u64);
        assert_eq!(date.timestamp, 24 * DAY);
        let date = date + Duration::from_secs((DAY + 26 * HOUR) as u64);
        assert_eq!(date.timestamp, 26 * DAY + 2 * HOUR);
    }
}

#[cfg(test)]
mod test_write_regs {
    use super::Naive;
    use crate::{constant::DAY, ReadRtcRegisters, WriteRtcRegisters};

    #[test]
    fn seconds() {
        let mut date = Naive::default();

        assert_eq!(date.seconds(), 0);
        date.set_seconds(49);
        assert_eq!(date.seconds(), 49);
    }

    #[test]
    fn minutes() {
        let mut date = Naive::default();

        assert_eq!(date.minutes(), 0);
        date.set_minutes(32);
        assert_eq!(date.minutes(), 32);
    }

    #[test]
    fn hours() {
        let mut date = Naive::default();

        assert_eq!(date.hours(), 0);
        date.set_hours(18);
        assert_eq!(date.hours(), 18);
    }

    #[test]
    fn days() {
        let mut date = Naive::default();

        assert_eq!(date.days(), 0);
        assert_eq!(date.lower_days(), 0);
        assert_eq!(date.upper_days(), false);

        date.set_lower_days(0x2f);

        assert_eq!(date.days(), 0x2f);
        assert_eq!(date.lower_days(), 0x2f);
        assert_eq!(date.upper_days(), false);

        date.set_upper_days(true);

        assert_eq!(date.days(), 0x12f);
        assert_eq!(date.lower_days(), 0x2f);
        assert_eq!(date.upper_days(), true);

        date.set_upper_days(false);

        assert_eq!(date.days(), 0x2f);
        assert_eq!(date.lower_days(), 0x2f);
        assert_eq!(date.upper_days(), false);
    }

    #[test]
    fn halted() {
        let mut date = Naive::default();

        assert_eq!(date.halted(), true);
        date.set_halted(false);
        assert_eq!(date.halted(), false);
        date.set_halted(true);
        assert_eq!(date.halted(), true);
    }

    #[test]
    fn day_counter_carry() {
        let mut date = Naive::from_days(0x1FF);

        assert_eq!(date.days(), 0x1FF);
        assert_eq!(date.day_counter_carry(), false);

        date.set_day_counter_carry(true);
        assert_eq!(date.day_counter_carry(), true);

        date.set_day_counter_carry(false);
        assert_eq!(date.day_counter_carry(), false);

        let date = date + 1 * DAY;
        assert_eq!(date.day_counter_carry(), true);
        assert_eq!(date.days(), 0);
    }
}
