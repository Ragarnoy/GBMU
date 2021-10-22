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

impl Ticker for FakeCPU {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick<FakeBus>(&mut self, _adr_bus: &mut FakeBus) {
        self.tick_count += 1;
    }
}

struct FakePPU {
    pub tick_count: usize,
}

impl Ticker for FakePPU {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick<FakeBus>(&mut self, _adr_bus: &mut FakeBus) {
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
    let mut timer = FakeCPU { tick_count: 0 };
    let mut bus = FakeBus::default();
    let mut clock = Clock::default();
    let one_sec = Duration::from_secs(1);

    log::info!("start 5s count example");

    for l in 0..5 {
        let t_stop = Instant::now() + one_sec;
        let mut frames = 0;
        cpu.tick_count = 0;
        ppu.tick_count = 0;
        while Instant::now() < t_stop {
            clock.frame(&mut bus, None, &mut cpu, &mut ppu, &mut timer);
            frames += 1;
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
