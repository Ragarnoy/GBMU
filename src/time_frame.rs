use std::time::Duration;

#[derive(Default, Debug)]
pub struct TimeStat<const SAMPLE_SIZE: usize = 60> {
    sample: CyclicBuffer<Duration, SAMPLE_SIZE>,
}

impl<const SAMPLE_SIZE: usize> TimeStat<SAMPLE_SIZE> {
    pub fn add_sample(&mut self, sample: Duration) {
        self.sample.push(sample);
    }

    pub fn iter(&self) -> std::slice::Iter<Duration> {
        self.sample.iter()
    }

    #[cfg(feature = "fps_stat")]
    pub fn last(&self) -> Duration {
        *self.sample.last()
    }
}

#[derive(Debug)]
struct CyclicBuffer<T, const SIZE: usize> {
    buffer: [T; SIZE],
    index: usize,
}

impl<T, const SIZE: usize> Default for CyclicBuffer<T, SIZE>
where
    T: Default + Copy,
{
    fn default() -> Self {
        assert!(SIZE > 0, "SIZE should be greater than 0");
        Self {
            buffer: [T::default(); SIZE],
            index: 0,
        }
    }
}

impl<T, const SIZE: usize> CyclicBuffer<T, SIZE> {
    pub fn push(&mut self, value: T) {
        self.buffer[self.index] = value;
        self.index = (self.index.wrapping_add(1)) % SIZE;
    }

    #[cfg(feature = "fps_stat")]
    pub fn last(&self) -> &T {
        &self.buffer[self.index.wrapping_sub(1) % SIZE]
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.buffer.as_slice().iter()
    }
}
