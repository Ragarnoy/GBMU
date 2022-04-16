#[cfg(feature = "time_stat_samples")]
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Duration;

#[derive(Default, Debug)]
pub struct TimeStat {
    min: Option<Duration>,
    max: Option<Duration>,
    last_value: Duration,
    sum: Duration,
    sample_count: u32,
    #[cfg(feature = "time_stat_samples")]
    samples: VecDeque<Duration>,
}

impl TimeStat {
    #[cfg(feature = "time_stat_samples")]
    const SAMPLES_MAX_COUNT: usize = 120;

    pub fn add_sample(&mut self, sample: Duration) {
        let min = self.min.get_or_insert(sample);
        *min = (*min).min(sample);

        let max = self.max.get_or_insert(sample);
        *max = (*max).max(sample);

        self.last_value = sample;

        self.sum += sample;
        self.sample_count += 1;
        #[cfg(feature = "time_stat_samples")]
        {
            self.samples.push_front(sample);
            if self.samples.len() > Self::SAMPLES_MAX_COUNT {
                self.samples.pop_back();
            }
        }
    }

    pub fn mean(&self) -> Duration {
        self.sum / self.sample_count
    }

    #[cfg(feature = "time_stat_samples")]
    pub fn fps(&self) -> f64 {
        let mean = self.samples.iter().fold(0.0, |acc, elt| {
            acc + elt.as_nanos() as f64 / self.samples.len() as f64
        });
        1_000_000_000.0 / mean
    }

    pub fn instant_fps(&self) -> f64 {
        if self.last_value != Duration::ZERO {
            (1_000_000_000.0 / self.last_value.as_nanos() as f64)
        } else {
            f64::NAN
        }
    }
}

impl Display for TimeStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "time_stat_samples"))]
        return write!(
            f,
            "{{ min: {}ms, mean: {}ms, max: {}ms}}",
            self.min.unwrap_or_default().as_millis(),
            self.mean().as_millis(),
            self.max.unwrap_or_default().as_millis()
        );
        #[cfg(feature = "time_stat_samples")]
        return write!(
            f,
            "{{ min: {}ms, mean: {}ms, max: {}ms, fps: {:>7.2}}}",
            self.min.unwrap_or_default().as_millis(),
            self.mean().as_millis(),
            self.max.unwrap_or_default().as_millis(),
            self.fps()
        );
    }
}
