use crate::Ticker;

pub struct Clock {}

impl Clock {
    pub fn cycle(&self, process_units: Vec<&mut dyn Ticker>) {
        for ticker in process_units {
            for _ in 0..ticker.cycle_count().into() {
                ticker.tick();
            }
        }
    }
}
