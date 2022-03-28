use std::{cell::RefCell, rc::Rc};

use gb_bus::Bus;
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;

pub struct Apu {
    audio_queue: Rc<RefCell<AudioQueue<i16>>>,
    step: usize,
}

impl Apu {
    pub fn new(audio_queue: Rc<RefCell<AudioQueue<i16>>>) -> Apu {
        Self {
            audio_queue,
            step: 0,
        }
    }
    fn gen_wave(bytes_to_write: i32) -> Vec<i16> {
        // Generate a square wave
        let tone_volume = 1_000i16;
        let period = 41_000 / 440;
        let sample_count = bytes_to_write;
        let mut result = Vec::new();

        for x in 0..sample_count {
            result.push(if (x / period) % 2 == 0 {
                tone_volume
            } else {
                -tone_volume
            });
        }
        result
    }
}

impl Ticker for Apu {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, _addr_bus: &mut dyn Bus<u8>) {
        if self.step % 1000000 == 0 {
            self.audio_queue
                .borrow()
                .queue_audio(&Apu::gen_wave(41_000))
                .expect("failed to queue audio");
        }
        self.step += 1;
    }
}
