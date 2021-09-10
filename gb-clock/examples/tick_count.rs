use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use gb_clock::{Clock, Tick, Ticker};

use std::time::{Duration, Instant};

struct FakeCPU {
    pub tick_count: usize,
}

impl Ticker for FakeCPU {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self) {
        self.tick_count += 1;
    }
}

struct FakePPU {
    pub tick_count: usize,
}

impl Ticker for FakePPU {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self) {
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
    let clock = Clock {};
    let one_sec = Duration::from_millis(1000);

    log::info!("start 5s count example");

    for l in 0..5 {
        let t_stop = Instant::now() + one_sec;
        cpu.tick_count = 0;
        ppu.tick_count = 0;
        let mut process_unit: Vec<&mut dyn Ticker> = vec![&mut cpu, &mut ppu];
        while Instant::now() < t_stop {
            clock.cycle(&mut process_unit);
        }
        log::info!(
            "loop {}:\t{} cpu ticks,\t\t{} ppu ticks",
            l,
            cpu.tick_count,
            ppu.tick_count
        );
        log::info!(
            "loop {}:\t{:.1} cpu ticks/frame,\t{:.1} ppu ticks/frame",
            l,
            cpu.tick_count as f64 / 59.7,
            ppu.tick_count as f64 / 59.7
        );
        assert_eq!(cpu.tick_count * 4, ppu.tick_count);
    }
    log::info!("count example ended");
}
