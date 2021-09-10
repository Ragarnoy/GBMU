use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use gb_bus::Bus;
use gb_clock::{Clock, Tick, Ticker};

use std::time::{Duration, Instant};

#[derive(Default)]
struct FakeBus {}

impl Bus<u8> for FakeBus {
    fn read(&self, _adr: u16) -> Result<u8, gb_bus::Error> {
        Ok(0xff)
    }
    fn write(&mut self, _adr: u16, _data: u8) -> Result<(), gb_bus::Error> {
        Ok(())
    }
}
impl Bus<u16> for FakeBus {
    fn read(&self, _adr: u16) -> Result<u16, gb_bus::Error> {
        Ok(0xffff)
    }
    fn write(&mut self, _adr: u16, _data: u16) -> Result<(), gb_bus::Error> {
        Ok(())
    }
}

struct FakeCPU {
    pub tick_count: usize,
}

impl Ticker<FakeBus> for FakeCPU {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, _adr_bus: &mut FakeBus) {
        self.tick_count += 1;
    }
}

struct FakePPU {
    pub tick_count: usize,
}

impl Ticker<FakeBus> for FakePPU {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, _adr_bus: &mut FakeBus) {
        self.tick_count += 1;
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("cannot setup terminal logger");

    let mut cpu = FakeCPU { tick_count: 0 };
    let mut ppu = FakePPU { tick_count: 0 };
    let mut bus = FakeBus::default();
    let mut clock = Clock::default();
    let one_sec = Duration::from_millis(1_000);
    let one_frame = Duration::from_nanos(16_750_419);

    log::info!("start 5s count example");

    for l in 0..5 {
        let t_stop = Instant::now() + one_sec;
        let mut frames = 0;
        cpu.tick_count = 0;
        ppu.tick_count = 0;
        let mut process_unit: Vec<&mut dyn Ticker<_>> = vec![&mut cpu, &mut ppu];
        while Instant::now() < t_stop {
            let t_stop_frame = Instant::now() + one_frame;
            clock.frame(&mut bus, &mut process_unit);
            frames += 1;
            while Instant::now() < t_stop_frame {}
        }
        log::info!(
            "loop {}:\t\t{} cpu ticks,\t\t{} ppu ticks",
            l,
            cpu.tick_count,
            ppu.tick_count
        );
        log::info!(
            "  {} frames\t{:.1} cpu ticks/frame,\t{:.1} ppu ticks/frame",
            frames,
            cpu.tick_count as f64 / frames as f64,
            ppu.tick_count as f64 / frames as f64
        );
        assert_eq!(cpu.tick_count * 4, ppu.tick_count);
    }
    log::info!("count example ended");
}
