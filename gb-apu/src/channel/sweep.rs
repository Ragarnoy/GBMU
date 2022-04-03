use super::volume_envelope::Direction;

#[derive(Debug)]
pub struct Sweep {
    pub enabled: bool,
    pub period: u8,
    pub direction: Direction,
    pub shift_nb: u8,
    pub shadow_frequency: u16,
    counter: u8,
}
impl Default for Sweep {
    fn default() -> Self {
        Self {
            enabled: false,
            period: 0,
            direction: Direction::Inc,
            shift_nb: 0,
            shadow_frequency: 0,
            counter: 0,
        }
    }
}

impl Sweep {
    pub fn step(&mut self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.counter > 0 {
            self.counter -= 1;
        }
        if self.counter == 0 {
            self.reload_counter();
            return true;
        }
        return false;
    }

    fn reload_counter(&mut self) {
        self.counter = if self.period > 0 { self.period } else { 8 }
    }

    pub fn reload(&mut self, frequency: u16) -> bool {
        self.enabled = self.period > 0 || self.shift_nb > 0;
        self.shadow_frequency = frequency;
        self.reload_counter();
        if self.shift_nb > 0 {
            let new_frequency = self.calculate_frequency();
            self.is_overflowing(new_frequency)
        } else {
            false
        }
    }
    pub fn calculate_frequency(&self) -> u16 {
        let mut new_frequency = self.shadow_frequency >> self.shift_nb;
        match self.direction {
            Direction::Dec => new_frequency = self.shadow_frequency.wrapping_sub(new_frequency),
            Direction::Inc => new_frequency = self.shadow_frequency.wrapping_add(new_frequency),
        };
        new_frequency
    }
    pub fn is_overflowing(&self, frequency: u16) -> bool {
        frequency >= 2048
    }
}
