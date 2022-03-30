use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use gb_bus::{Area, Bus, MemoryLock, Source};
use gb_clock::{cycles, Clock, Tick, Ticker};

use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Default)]
struct FakeBus {}

impl Bus<u8> for FakeBus {
    fn read(&self, _adr: u16, _lock_key: Option<Source>) -> Result<u8, gb_bus::Error> {
        Ok(0xff)
    }
    fn write(
        &mut self,
        _adr: u16,
        _data: u8,
        _lock_key: Option<Source>,
    ) -> Result<(), gb_bus::Error> {
        Ok(())
    }
}

impl MemoryLock for FakeBus {
    fn lock(&mut self, _area: Area, _lock: Source) {}

    fn unlock(&mut self, _area: Area) {}

    fn is_available(&self, _area: Area, _lock_key: Option<Source>) -> bool {
        true
    }
}

struct FakeCPU {
    pub tick_count: usize,
}

impl Ticker for FakeCPU {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, _adr_bus: &mut dyn Bus<u8>) {
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

    fn tick(&mut self, _adr_bus: &mut dyn Bus<u8>) {
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

    let cpu = Rc::new(RefCell::new(FakeCPU { tick_count: 0 }));
    let mut ppu = FakePPU { tick_count: 0 };
    let timer = Rc::new(RefCell::new(FakeCPU { tick_count: 0 }));
    let mut bus = FakeBus::default();
    let mut clock = Clock::default();
    let one_sec = Duration::from_secs(1);
    log::info!("start 5s count example");
    let mut frames = 0;
    for l in 0..5 {
        let mut curr_frames = 0;
        let mut cycle = 0;
        let t_stop = Instant::now() + one_sec;
        let mut tmp_cpu = cpu.borrow_mut();
        let ref_cpu = tmp_cpu.deref_mut();
        let ref_ppu = &mut ppu;
        let mut tmp_timer = timer.borrow_mut();
        let ref_timer = tmp_timer.deref_mut();
        while Instant::now() < t_stop {
            if !cycles!(clock, &mut bus, ref_cpu, ref_ppu, ref_timer) {
                curr_frames += 1;
            }
            cycle += 1;
        }
        log::info!(
            "Sec {}:\t\t{} cycles,\t\t\t{} frames",
            l,
            cycle,
            curr_frames
        );
        frames += curr_frames;
    }
    log::info!(
        "Total:\t\t{} cpu ticks,\t\t{} ppu ticks",
        cpu.borrow().tick_count,
        ppu.tick_count
    );
    log::info!(
        "{} frames:\t{:.1} cpu ticks/frame,\t{:.1} ppu ticks/frame",
        frames,
        cpu.borrow().tick_count as f64 / frames as f64,
        ppu.tick_count as f64 / frames as f64
    );
    log::info!("{:.1} frames per seconds.", frames as f32 / 5.0);
    assert_eq!(cpu.borrow().tick_count * 4, ppu.tick_count);
    log::info!("count example ended");
}
