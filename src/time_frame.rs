use std::fmt::Display;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct TimeStat {
    min: Option<Duration>,
    max: Option<Duration>,
    sum: Duration,
    sample_count: u32,
}

impl TimeStat {
    pub fn add_sample(&mut self, sample: Duration) {
        let min = self.min.get_or_insert(sample);
        *min = (*min).min(sample);

        let max = self.max.get_or_insert(sample);
        *max = (*max).max(sample);

        self.sum += sample;
        self.sample_count += 1;
    }

    pub fn mean(&self) -> Duration {
        self.sum / self.sample_count
    }
}

impl Display for TimeStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ min: {}ms, mean: {}ms, max: {}ms }}",
            self.min.unwrap_or_default().as_millis(),
            self.mean().as_millis(),
            self.max.unwrap_or_default().as_millis()
        )
    }
}
