use anyhow::Result;
use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Bus, IORegBus, Lock, WorkingRam,
};
use gb_clock::Clock;
use gb_cpu::cpu::Cpu;
use gb_dbg::dbg_interfaces::{
    DebugOperations, MemoryDebugOperations, RegisterDebugOperations, RegisterMap, RegisterValue,
};
use gb_joypad::Joypad;
use gb_lcd::{
    render::{RenderImage, SCREEN_HEIGHT, SCREEN_WIDTH},
    window::GBWindow,
};
use gb_ppu::Ppu;
use gb_roms::{
    controllers::{bios, generate_rom_controller, BiosWrapper, MbcController},
    header::AutoSave,
    Header,
};
use gb_timer::Timer;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

pub struct Context<const WIDTH: usize, const HEIGHT: usize> {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub windows: Windows,
    pub display: RenderImage<WIDTH, HEIGHT>,
    pub joypad: Joypad,
}

pub struct Windows {
    pub main: GBWindow,
    pub debug: Option<GBWindow>,
    pub input: Option<GBWindow>,
}

pub struct Game {
    pub romname: String,
    pub header: Header,
    pub auto_save: Option<AutoSave>,
    pub mbc: Rc<RefCell<MbcController>>,
    pub cpu: Rc<RefCell<Cpu>>,
    pub ppu: Ppu,
    pub clock: Clock<AddressBus>,
    pub io_bus: Rc<RefCell<IORegBus>>,
    pub timer: Rc<RefCell<Timer>>,
    pub addr_bus: AddressBus,
    scheduled_stop: Option<ScheduledStop>,
    emulation_stopped: bool,
}

enum ScheduledStop {
    /// Schedule a stop after `usize` step
    AfterStep(usize),
    /// Schedule a stop after `usize` frame
    AfterFrame(usize),
    /// Schedule a stop after `time` delay
    AfterTimeout(std::time::Instant, std::time::Duration),
}

impl Game {
    pub fn new(romname: String, stopped: bool) -> Result<Game, anyhow::Error> {
        use std::{fs::File, io::Seek};

        let mut file = File::open(romname.clone())?;
        let header = Header::from_file(&mut file)?;

        log::debug!("header: {:?}", header);

        file.rewind()?;
        let mbc = generate_rom_controller(file, header.clone())?;
        let mbc = Rc::new(RefCell::new(mbc));

        let ppu = Ppu::new();
        let ppu_mem = Rc::new(RefCell::new(ppu.memory()));
        let ppu_reg = Rc::new(RefCell::new(ppu.registers()));
        let cpu = Rc::new(RefCell::new(Cpu::default()));
        let wram = Rc::new(RefCell::new(WorkingRam::new(false)));
        let timer = Rc::new(RefCell::new(Timer::default()));
        let bios = Rc::new(RefCell::new(bios::dmg()));
        let bios_wrapper = Rc::new(RefCell::new(BiosWrapper::new(bios, mbc.clone())));

        let io_bus = Rc::new(RefCell::new(IORegBus {
            controller: Rc::new(RefCell::new(CharDevice::default())),
            communication: Rc::new(RefCell::new(SimpleRW::<2>::default())), // We don't handle communication
            div_timer: timer.clone(),
            tima: timer.clone(),
            tma: timer.clone(),
            tac: timer.clone(),
            sound: Rc::new(RefCell::new(SimpleRW::<0x17>::default())), // We don't handle sound
            waveform_ram: Rc::new(RefCell::new(SimpleRW::<0xF>::default())), // We don't handle sound
            lcd: ppu_reg.clone(),
            vram_bank: ppu_reg.clone(),
            boot_rom: bios_wrapper.clone(),
            vram_dma: Rc::new(RefCell::new(SimpleRW::<6>::default())), // TODO: link the part that handle the DMA
            bg_obj_palettes: ppu_reg,
            wram_bank: wram.clone(),
            interrupt_flag: cpu.clone(),
        }));

        let bus = AddressBus {
            rom: bios_wrapper,
            vram: ppu_mem.clone(),
            ext_ram: mbc.clone(),
            ram: wram.clone(),
            eram: wram,
            oam: ppu_mem,
            io_reg: io_bus.clone(),
            hram: Rc::new(RefCell::new(SimpleRW::<0x80>::default())),

            ie_reg: cpu.clone(),
            area_locks: HashMap::new(),
        };

        Ok(Self {
            romname,
            header: header.clone(),
            auto_save: header.cartridge_type.auto_save_type(),
            mbc,
            cpu,
            ppu,
            clock: Clock::default(),
            io_bus,
            timer,
            addr_bus: bus,
            scheduled_stop: None,
            emulation_stopped: stopped,
        })
    }

    pub fn cycle(&mut self) -> bool {
        if !self.emulation_stopped {
            let frame_ended = self.clock.cycle(
                &mut self.addr_bus,
                self.cpu.borrow_mut(),
                &mut self.ppu,
                self.timer.borrow_mut(),
            );
            self.check_scheduled_stop(frame_ended);
            frame_ended
        } else {
            false
        }
    }

    fn check_scheduled_stop(&mut self, frame_ended: bool) {
        if let Some(ref mut scheduled) = self.scheduled_stop {
            match scheduled {
                ScheduledStop::AfterStep(count) => {
                    if *count == 1 {
                        self.emulation_stopped = true;
                        self.scheduled_stop = None;
                    } else {
                        *count -= 1;
                    }
                }
                ScheduledStop::AfterFrame(count) => {
                    if frame_ended {
                        if *count == 1 {
                            self.emulation_stopped = true;
                            self.scheduled_stop = None;
                        } else {
                            *count -= 1;
                        }
                    }
                }
                ScheduledStop::AfterTimeout(instant, timeout) => {
                    if &instant.elapsed() > timeout {
                        self.emulation_stopped = true;
                        self.scheduled_stop = None;
                    }
                }
            }
        }
    }

    pub fn draw(&self, context: &mut Context<SCREEN_WIDTH, SCREEN_HEIGHT>) {
        context.display.update_render(self.ppu.pixels());
        context.display.draw();
    }
}

impl DebugOperations for Game {}

impl MemoryDebugOperations for Game {
    fn read(&self, index: u16) -> u8 {
        self.addr_bus
            .read(index, Some(Lock::Debugger))
            .unwrap_or_else(|err| {
                log::error!("[DBG-OPS] bus read error at {}: {:?}", index, err);
                0xff
            })
    }
}

impl RegisterDebugOperations for Game {
    fn cpu_get(&self, key: &str) -> Result<RegisterValue> {
        match key {
            "AF" => Ok(self.cpu.borrow().registers.af.into()),
            "BC" => Ok(self.cpu.borrow().registers.bc.into()),
            "DE" => Ok(self.cpu.borrow().registers.de.into()),
            "HL" => Ok(self.cpu.borrow().registers.hl.into()),
            "SP" => Ok(self.cpu.borrow().registers.sp.into()),
            "PC" => Ok(self.cpu.borrow().registers.pc.into()),
            _ => Ok(0xffu8.into()),
        }
    }

    fn ppu_get(&self, _key: &str) -> Result<RegisterValue> {
        Ok(RegisterValue::U8(0xff))
    }

    fn io_get(&self, _key: &str) -> Result<RegisterValue> {
        Ok(RegisterValue::U8(0xff))
    }

    fn cpu_registers(&self) -> Vec<RegisterMap> {
        vec![
            ("AF".to_string(), self.cpu.borrow().registers.af.into()),
            ("BC".to_string(), self.cpu.borrow().registers.bc.into()),
            ("DE".to_string(), self.cpu.borrow().registers.de.into()),
            ("HL".to_string(), self.cpu.borrow().registers.hl.into()),
            ("SP".to_string(), self.cpu.borrow().registers.sp.into()),
            ("PC".to_string(), self.cpu.borrow().registers.pc.into()),
        ]
    }

    fn ppu_registers(&self) -> Vec<RegisterMap> {
        Vec::new()
    }

    fn io_registers(&self) -> Vec<RegisterMap> {
        Vec::new()
    }
}
