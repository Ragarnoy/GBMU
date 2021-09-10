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
    let one_sec = Duration::from_millis(1000);

    log::info!("start 5s count example");

    for l in 0..5 {
        let t_stop = Instant::now() + one_sec;
        cpu.tick_count = 0;
        ppu.tick_count = 0;
        let mut process_unit: Vec<&mut dyn Ticker<_>> = vec![&mut cpu, &mut ppu];
        while Instant::now() < t_stop {
            clock.cycle(&mut bus, &mut process_unit);
        }
        log::info!(
            "loop {}:\t\t{} cpu ticks,\t\t{} ppu ticks",
            l,
            cpu.tick_count,
            ppu.tick_count
        );
        assert_eq!(cpu.tick_count * 4, ppu.tick_count);
    }
    log::info!("count example ended");
}
