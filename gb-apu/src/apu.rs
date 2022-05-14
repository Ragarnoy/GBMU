use std::sync::{Arc, Mutex};

use crate::{
    channel::sound_channel::SoundChannel, control::frame_sequencer::FrameSequencer, ChannelType,
    MASK_UNUSED_BITS_70,
};
use crate::{NB_CYCLES_512_HZ, T_CYCLE_FREQUENCY};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BuildStreamError, Device, SampleFormat, SampleRate, Stream, StreamConfig, StreamError,
    SupportedBufferSize,
};
use gb_bus::{Address, Bus, Error, FileOperation, IORegArea, Source};
use gb_clock::{Tick, Ticker};

pub struct Apu {
    cycle_counter: u32,
    nb_cycles_per_sample: u32,
    enabled: bool,
    buffer: Arc<Mutex<Vec<f32>>>,
    sound_channels: Vec<SoundChannel>,
    frame_sequencer: FrameSequencer,
    master: u8,
    panning: u8,
    stream: Option<Stream>,
}

impl Apu {
    pub fn new(
        input_buffer: Arc<Mutex<Vec<f32>>>,
        stream: Option<Stream>,
        sample_rate: SampleRate,
    ) -> Apu {
        // Channels order in vector is important !
        let sound_channels = vec![
            SoundChannel::new(ChannelType::SquareWave, true),
            SoundChannel::new(ChannelType::SquareWave, false),
            SoundChannel::new(ChannelType::WaveForm, false),
            SoundChannel::new(ChannelType::Noise, false),
        ];

        Self {
            cycle_counter: 0,
            nb_cycles_per_sample: T_CYCLE_FREQUENCY / sample_rate.0,
            enabled: false,
            buffer: input_buffer,
            sound_channels,
            frame_sequencer: FrameSequencer::default(),
            master: 0,
            panning: 0,
            stream,
        }
    }

    pub fn init_audio_output(input_buffer: Arc<Mutex<Vec<f32>>>) -> (Stream, SampleRate) {
        let required_buffer_size = input_buffer.lock().unwrap().capacity();

        let host = cpal::default_host();
        let mut device = host
            .default_output_device()
            .expect("no output device available");
        if device.supported_output_configs().is_err() {
            device = host.devices().unwrap().next().unwrap();
        }
        log::debug!("selected device: {:?}", device.name());

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        log::debug!("selected config: {:?}", supported_config);

        if let SupportedBufferSize::Range { min, max } = supported_config.buffer_size() {
            assert!(
                *min < required_buffer_size as u32,
                "required_buffer_size is smaller than the minimum supported buffer range"
            );
            assert!(
                (required_buffer_size as u32) < *max,
                "required_buffer_size is greater than the maximum supported buffer range"
            );
        } else {
            log::warn!("cannot check the supported buffer size");
        }
        let err_fn = |err| log::error!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_config.sample_format();
        let mut config: StreamConfig = supported_config.into();
        config.buffer_size = cpal::BufferSize::Fixed(required_buffer_size as u32);
        log::debug!("configured config: {:?}", config);

        let stream =
            Apu::build_output_stream(device, sample_format, &config, input_buffer, err_fn).unwrap();
        stream.play().unwrap();
        (stream, config.sample_rate)
    }

    fn build_output_stream<E>(
        device: Device,
        sample_format: SampleFormat,
        config: &StreamConfig,
        input_buffer: Arc<Mutex<Vec<f32>>>,
        error_callback: E,
    ) -> Result<Stream, BuildStreamError>
    where
        E: FnMut(StreamError),
        E: Send + 'static,
    {
        let channels = config.channels as usize;

        // callback used to get the next sample
        let mut next_value = move || {
            let mut buffer = input_buffer.lock().unwrap();
            if buffer.len() > 0 {
                buffer.remove(0)
            } else {
                0.0
            }
        };

        match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    Self::write_data(data, channels, &mut next_value)
                },
                error_callback,
            ),
            SampleFormat::I16 => device.build_output_stream(
                config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    Self::write_data(data, channels, &mut next_value)
                },
                error_callback,
            ),
            SampleFormat::U16 => device.build_output_stream(
                config,
                move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    Self::write_data(data, channels, &mut next_value)
                },
                error_callback,
            ),
        }
    }

    fn write_data<T, N>(output: &mut [T], channels: usize, next_value: &mut N)
    where
        T: cpal::Sample,
        N: FnMut() -> f32,
        N: Send + 'static,
    {
        for frame in output.chunks_mut(channels) {
            let value = next_value();
            let value: T = cpal::Sample::from::<f32>(&value);
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }

    pub fn is_buffer_full(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.len() == buffer.capacity()
    }

    fn add_sample(&mut self) {
        let sample = self.mix() * 0.3;
        self.buffer.lock().unwrap().push(sample);
    }

    fn mix(&self) -> f32 {
        let mut sample = 0.0;

        for i in 0..self.sound_channels.len() {
            sample += self.sound_channels[i].get_dac_output();
        }
        sample /= self.sound_channels.len() as f32;
        sample
    }

    fn get_power_channels_statuses_byte(&self) -> u8 {
        let mut res = 0;
        res |= if self.enabled { 0x80 } else { 0 };
        res |= if self.sound_channels[3].enabled {
            0x8
        } else {
            0
        };
        res |= if self.sound_channels[3].enabled {
            0x8
        } else {
            0
        };
        res |= if self.sound_channels[2].enabled {
            0x4
        } else {
            0
        };
        res |= if self.sound_channels[1].enabled {
            0x2
        } else {
            0
        };
        res |= if self.sound_channels[0].enabled {
            0x1
        } else {
            0
        };
        res
    }
}

impl Ticker for Apu {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, _addr_bus: &mut dyn Bus<u8>) {
        if !self.enabled || self.stream.is_none() {
            return;
        }

        self.cycle_counter += 1;
        for i in 0..self.sound_channels.len() {
            self.sound_channels[i].step();
        }

        // Frame sequencer is clocked at 512 Hz
        // 0x400_000 (TCycle freq.) / 0x2000 = 512 Hz
        if self.cycle_counter >= NB_CYCLES_512_HZ {
            self.cycle_counter %= NB_CYCLES_512_HZ;

            let step = self.frame_sequencer.step();
            for i in 0..self.sound_channels.len() {
                self.sound_channels[i].frame_sequencer(step);
            }
        }

        if self.cycle_counter % self.nb_cycles_per_sample == 0 {
            self.add_sample();
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Apu
where
    A: Address<IORegArea>,
    u16: From<A>,
{
    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52, WaveRam0, WaveRam1, WaveRam2, WaveRam3,
            WaveRam4, WaveRam5, WaveRam6, WaveRam7, WaveRam8, WaveRam9, WaveRamA, WaveRamB,
            WaveRamC, WaveRamD, WaveRamE, WaveRamF,
        };
        match addr.area_type() {
            Nr10 | Nr11 | Nr12 | Nr13 | Nr14 => self.sound_channels[0].read(addr, None),
            Nr21 | Nr22 | Nr23 | Nr24 => self.sound_channels[1].read(addr, None),
            Nr30 | Nr31 | Nr32 | Nr33 | Nr34 | WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3
            | WaveRam4 | WaveRam5 | WaveRam6 | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA
            | WaveRamB | WaveRamC | WaveRamD | WaveRamE | WaveRamF => {
                self.sound_channels[2].read(addr, None)
            }
            Nr41 | Nr42 | Nr43 | Nr44 => self.sound_channels[3].read(addr, None),
            Nr50 => Ok(self.master),
            Nr51 => Ok(self.panning),
            Nr52 => Ok(self.get_power_channels_statuses_byte() | MASK_UNUSED_BITS_70),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52, WaveRam0, WaveRam1, WaveRam2, WaveRam3,
            WaveRam4, WaveRam5, WaveRam6, WaveRam7, WaveRam8, WaveRam9, WaveRamA, WaveRamB,
            WaveRamC, WaveRamD, WaveRamE, WaveRamF,
        };
        match addr.area_type() {
            Nr10 | Nr11 | Nr12 | Nr13 | Nr14 => {
                if self.enabled {
                    return self.sound_channels[0].write(v, addr, None);
                } else {
                    return Ok(());
                }
            }
            Nr21 | Nr22 | Nr23 | Nr24 => {
                if self.enabled {
                    return self.sound_channels[1].write(v, addr, None);
                } else {
                    return Ok(());
                }
            }
            Nr30 | Nr31 | Nr32 | Nr33 | Nr34 => {
                if self.enabled {
                    return self.sound_channels[2].write(v, addr, None);
                } else {
                    return Ok(());
                }
            }
            WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3 | WaveRam4 | WaveRam5 | WaveRam6
            | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA | WaveRamB | WaveRamC | WaveRamD
            | WaveRamE | WaveRamF => return self.sound_channels[2].write(v, addr, None),
            Nr41 | Nr42 | Nr43 | Nr44 => {
                if self.enabled {
                    return self.sound_channels[3].write(v, addr, None);
                } else {
                    return Ok(());
                }
            }
            Nr50 => {
                if self.enabled {
                    self.master = v;
                }
            }
            Nr51 => {
                if self.enabled {
                    self.panning = v;
                }
            }
            Nr52 => {
                let was_enabled = self.enabled;
                let enabled = v & 0x80 != 0x00;
                if was_enabled && !enabled {
                    self.sound_channels = vec![
                        self.sound_channels[0].reset(),
                        self.sound_channels[1].reset(),
                        self.sound_channels[2].reset(),
                        self.sound_channels[3].reset(),
                    ];
                    self.master = 0;
                    self.panning = 0;
                }
                self.enabled = enabled;
            }
            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
