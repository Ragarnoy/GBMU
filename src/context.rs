use anyhow::Result;
use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Bus, IORegBus, Lock, WorkingRam,
};
use gb_clock::Clock;
use gb_cpu::cpu::Cpu;
use gb_dbg::{
    dbg_interfaces::{
        CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs, RegisterDebugOperations,
        RegisterMap, RegisterValue,
    },
    until::Until,
};
use gb_dma::Dma;
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
    pub joypad: Rc<RefCell<Joypad>>,
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
    pub dma: Rc<RefCell<Dma>>,
    pub joypad: Rc<RefCell<Joypad>>,
    pub addr_bus: AddressBus,
    scheduled_stop: Option<ScheduledStop>,
    emulation_stopped: bool,
}

#[derive(Debug)]
enum ScheduledStop {
    /// Schedule a stop after `usize` step
    Step(usize),
    /// Schedule a stop after `usize` frame
    Frame(usize),
    /// Schedule a stop after `time` delay
    Timeout(std::time::Instant, std::time::Duration),
}

impl Game {
    pub fn new(
        romname: String,
        joypad: Rc<RefCell<Joypad>>,
        stopped: bool,
    ) -> Result<Game, anyhow::Error> {
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
        let dma = Rc::new(RefCell::new(Dma::new()));

        let io_bus = Rc::new(RefCell::new(IORegBus {
            controller: joypad.clone(),
            communication: Rc::new(RefCell::new(SimpleRW::<2>::default())), // We don't handle communication
            div_timer: timer.clone(),
            tima: timer.clone(),
            tma: timer.clone(),
            tac: timer.clone(),
            sound: Rc::new(RefCell::new(SimpleRW::<0x17>::default())), // We don't handle sound
            waveform_ram: Rc::new(RefCell::new(SimpleRW::<0xF>::default())), // We don't handle sound
            lcd: ppu_reg.clone(),
            oam_dma: dma.clone(),
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
            dma,
            joypad,
            addr_bus: bus,
            scheduled_stop: None,
            emulation_stopped: stopped,
        })
    }

    pub fn cycle(&mut self) -> bool {
        if !self.emulation_stopped {
            let frame_not_finished = self.clock.cycle(
                &mut self.addr_bus,
                self.cpu.borrow_mut(),
                &mut self.ppu,
                self.timer.borrow_mut(),
            );
            self.check_scheduled_stop(!frame_not_finished);
            frame_not_finished
        } else {
            false
        }
    }

    fn check_scheduled_stop(&mut self, frame_ended: bool) {
        if let Some(ref mut scheduled) = self.scheduled_stop {
            log::trace!(
                "check for stop, scheduled={:?}, framed_ended={}",
                scheduled,
                frame_ended
            );
            match scheduled {
                ScheduledStop::Step(count) => {
                    if *count == 1 {
                        self.emulation_stopped = true;
                        self.scheduled_stop = None;
                    } else {
                        *count -= 1;
                    }
                }
                ScheduledStop::Frame(count) => {
                    if frame_ended {
                        if *count == 1 {
                            self.emulation_stopped = true;
                            self.scheduled_stop = None;
                        } else {
                            *count -= 1;
                        }
                    }
                }
                ScheduledStop::Timeout(instant, timeout) => {
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

    pub fn update_scheduled_stop(&mut self, flow: std::ops::ControlFlow<Until>) {
        use std::ops::ControlFlow::{Break, Continue};
        match flow {
            Continue(()) => {
                self.emulation_stopped = false;
                self.scheduled_stop = None;
            }
            Break(Until::Null | Until::Step(0) | Until::Frame(0) | Until::Second(0)) => {
                self.emulation_stopped = true;
                self.scheduled_stop = None;
            }
            Break(Until::Step(count)) => {
                self.emulation_stopped = false;
                self.scheduled_stop = Some(ScheduledStop::Step(count));
            }
            Break(Until::Frame(count)) => {
                self.emulation_stopped = false;
                self.scheduled_stop = Some(ScheduledStop::Frame(count));
            }
            Break(Until::Second(count)) => {
                self.emulation_stopped = false;
                self.scheduled_stop = Some(ScheduledStop::Timeout(
                    std::time::Instant::now(),
                    std::time::Duration::from_secs(count.try_into().unwrap_or_else(|e| {
                        log::error!("cannot convert {}_usize to u64: {:?}", count, e);
                        1_u64
                    })),
                ));
            }
        }
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

macro_rules! read_bus_reg {
    ($type:expr, $bus:expr, $addr:expr) => {
        RegisterMap($type, read_bus_reg!($bus, $addr))
    };

    ($bus:expr, $addr:expr) => {
        $bus.read($addr, Some(Lock::Debugger))
            .unwrap_or(0xffu8)
            .into()
    };
}

impl RegisterDebugOperations for Game {
    fn cpu_get(&self, key: CpuRegs) -> Result<RegisterValue> {
        match key {
            CpuRegs::AF => Ok(self.cpu.borrow().registers.af.into()),
            CpuRegs::BC => Ok(self.cpu.borrow().registers.bc.into()),
            CpuRegs::DE => Ok(self.cpu.borrow().registers.de.into()),
            CpuRegs::HL => Ok(self.cpu.borrow().registers.hl.into()),
            CpuRegs::SP => Ok(self.cpu.borrow().registers.sp.into()),
            CpuRegs::PC => Ok(self.cpu.borrow().registers.pc.into()),
        }
    }

    fn ppu_get(&self, key: PpuRegs) -> Result<RegisterValue> {
        match key {
            PpuRegs::Control => Ok(read_bus_reg!(self.addr_bus, 0xFF40)),
            PpuRegs::Status => Ok(read_bus_reg!(self.addr_bus, 0xFF41)),
            PpuRegs::Scy => Ok(read_bus_reg!(self.addr_bus, 0xFF42)),
            PpuRegs::Scx => Ok(read_bus_reg!(self.addr_bus, 0xFF43)),
            PpuRegs::Ly => Ok(read_bus_reg!(self.addr_bus, 0xFF44)),
            PpuRegs::Lyc => Ok(read_bus_reg!(self.addr_bus, 0xFF45)),
            PpuRegs::Dma => Ok(read_bus_reg!(self.addr_bus, 0xFF46)),
            PpuRegs::Bgp => Ok(read_bus_reg!(self.addr_bus, 0xFF47)),
            PpuRegs::Obp0 => Ok(read_bus_reg!(self.addr_bus, 0xFF48)),
            PpuRegs::Obp1 => Ok(read_bus_reg!(self.addr_bus, 0xFF49)),
            PpuRegs::Wy => Ok(read_bus_reg!(self.addr_bus, 0xFF4A)),
            PpuRegs::Wx => Ok(read_bus_reg!(self.addr_bus, 0xFF4B)),
        }
    }

    fn io_get(&self, key: IORegs) -> Result<RegisterValue> {
        match key {
            // joypad regs
            IORegs::Joy => Ok(read_bus_reg!(self.addr_bus, 0xFF00)),
            // serial regs
            IORegs::SerialByte => Ok(read_bus_reg!(self.addr_bus, 0xFF01)),
            IORegs::SerialControl => Ok(read_bus_reg!(self.addr_bus, 0xFF02)),
            // Timer regs
            IORegs::Div => Ok(read_bus_reg!(self.addr_bus, 0xFF04)),
            IORegs::Tima => Ok(read_bus_reg!(self.addr_bus, 0xFF05)),
            IORegs::Tma => Ok(read_bus_reg!(self.addr_bus, 0xFF06)),
            IORegs::Tac => Ok(read_bus_reg!(self.addr_bus, 0xFF07)),
            // cpu int regs
            IORegs::If => Ok(read_bus_reg!(self.addr_bus, 0xFF0F)),
            IORegs::Ie => Ok(read_bus_reg!(self.addr_bus, 0xFFFF)),
            // Boot ROM
            IORegs::BootRom => Ok(read_bus_reg!(self.addr_bus, 0xFF50)),
            // audio regs
            IORegs::Fs1 => Ok(read_bus_reg!(self.addr_bus, 0xFF10)),
            IORegs::Pwm1 => Ok(read_bus_reg!(self.addr_bus, 0xFF11)),
            IORegs::Env1 => Ok(read_bus_reg!(self.addr_bus, 0xFF12)),
            IORegs::Af1 => Ok(read_bus_reg!(self.addr_bus, 0xFF13)),
            IORegs::Ctl1 => Ok(read_bus_reg!(self.addr_bus, 0xFF14)),
            IORegs::Pwm2 => Ok(read_bus_reg!(self.addr_bus, 0xFF16)),
            IORegs::Env2 => Ok(read_bus_reg!(self.addr_bus, 0xFF17)),
            IORegs::Af2 => Ok(read_bus_reg!(self.addr_bus, 0xFF18)),
            IORegs::Ctl2 => Ok(read_bus_reg!(self.addr_bus, 0xFF19)),
            IORegs::A3Toggle => Ok(read_bus_reg!(self.addr_bus, 0xFF1A)),
            IORegs::Pwm3 => Ok(read_bus_reg!(self.addr_bus, 0xFF1B)),
            IORegs::Vol3 => Ok(read_bus_reg!(self.addr_bus, 0xFF1C)),
            IORegs::Af3 => Ok(read_bus_reg!(self.addr_bus, 0xFF1D)),
            IORegs::Ctl3 => Ok(read_bus_reg!(self.addr_bus, 0xFF1E)),
            IORegs::Pwm4 => Ok(read_bus_reg!(self.addr_bus, 0xFF20)),
            IORegs::Vol4 => Ok(read_bus_reg!(self.addr_bus, 0xFF21)),
            IORegs::Af4 => Ok(read_bus_reg!(self.addr_bus, 0xFF22)),
            IORegs::Ctl4 => Ok(read_bus_reg!(self.addr_bus, 0xFF23)),
            IORegs::AudOutMap => Ok(read_bus_reg!(self.addr_bus, 0xFF24)),
            IORegs::AudMap => Ok(read_bus_reg!(self.addr_bus, 0xFF25)),
            IORegs::AudChanCtl => Ok(read_bus_reg!(self.addr_bus, 0xFF26)),
            IORegs::AudWave => Ok(read_bus_reg!(self.addr_bus, 0xFF30)),
        }
    }

    fn cpu_registers(&self) -> Vec<RegisterMap<CpuRegs>> {
        vec![
            RegisterMap(CpuRegs::AF, self.cpu.borrow().registers.af.into()),
            RegisterMap(CpuRegs::BC, self.cpu.borrow().registers.bc.into()),
            RegisterMap(CpuRegs::DE, self.cpu.borrow().registers.de.into()),
            RegisterMap(CpuRegs::HL, self.cpu.borrow().registers.hl.into()),
            RegisterMap(CpuRegs::SP, self.cpu.borrow().registers.sp.into()),
            RegisterMap(CpuRegs::PC, self.cpu.borrow().registers.pc.into()),
        ]
    }

    fn ppu_registers(&self) -> Vec<RegisterMap<PpuRegs>> {
        vec![
            // lcd regs
            read_bus_reg!(PpuRegs::Control, self.addr_bus, 0xFF40),
            read_bus_reg!(PpuRegs::Status, self.addr_bus, 0xFF41),
            read_bus_reg!(PpuRegs::Scy, self.addr_bus, 0xFF42),
            read_bus_reg!(PpuRegs::Scx, self.addr_bus, 0xFF43),
            read_bus_reg!(PpuRegs::Ly, self.addr_bus, 0xFF44),
            read_bus_reg!(PpuRegs::Lyc, self.addr_bus, 0xFF45),
            read_bus_reg!(PpuRegs::Dma, self.addr_bus, 0xFF46),
            read_bus_reg!(PpuRegs::Bgp, self.addr_bus, 0xFF47),
            read_bus_reg!(PpuRegs::Obp0, self.addr_bus, 0xFF48),
            read_bus_reg!(PpuRegs::Obp1, self.addr_bus, 0xFF49),
            read_bus_reg!(PpuRegs::Wy, self.addr_bus, 0xFF4A),
            read_bus_reg!(PpuRegs::Wx, self.addr_bus, 0xFF4B),
        ]
    }

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>> {
        vec![
            // joypad regs
            read_bus_reg!(IORegs::Joy, self.addr_bus, 0xFF00),
            // serial regs
            read_bus_reg!(IORegs::SerialByte, self.addr_bus, 0xFF01),
            read_bus_reg!(IORegs::SerialControl, self.addr_bus, 0xFF02),
            // Timer regs
            read_bus_reg!(IORegs::Div, self.addr_bus, 0xFF04),
            read_bus_reg!(IORegs::Tima, self.addr_bus, 0xFF05),
            read_bus_reg!(IORegs::Tma, self.addr_bus, 0xFF06),
            read_bus_reg!(IORegs::Tac, self.addr_bus, 0xFF07),
            // cpu int regs
            read_bus_reg!(IORegs::If, self.addr_bus, 0xFF0F),
            read_bus_reg!(IORegs::Ie, self.addr_bus, 0xFFFF),
            // Boot ROM
            read_bus_reg!(IORegs::BootRom, self.addr_bus, 0xFF50),
            // audio regs
            read_bus_reg!(IORegs::Fs1, self.addr_bus, 0xFF10),
            read_bus_reg!(IORegs::Pwm1, self.addr_bus, 0xFF11),
            read_bus_reg!(IORegs::Env1, self.addr_bus, 0xFF12),
            read_bus_reg!(IORegs::Af1, self.addr_bus, 0xFF13),
            read_bus_reg!(IORegs::Ctl1, self.addr_bus, 0xFF14),
            read_bus_reg!(IORegs::Pwm2, self.addr_bus, 0xFF16),
            read_bus_reg!(IORegs::Env2, self.addr_bus, 0xFF17),
            read_bus_reg!(IORegs::Af2, self.addr_bus, 0xFF18),
            read_bus_reg!(IORegs::Ctl2, self.addr_bus, 0xFF19),
            read_bus_reg!(IORegs::A3Toggle, self.addr_bus, 0xFF1A),
            read_bus_reg!(IORegs::Pwm3, self.addr_bus, 0xFF1B),
            read_bus_reg!(IORegs::Vol3, self.addr_bus, 0xFF1C),
            read_bus_reg!(IORegs::Af3, self.addr_bus, 0xFF1D),
            read_bus_reg!(IORegs::Ctl3, self.addr_bus, 0xFF1E),
            read_bus_reg!(IORegs::Pwm4, self.addr_bus, 0xFF20),
            read_bus_reg!(IORegs::Vol4, self.addr_bus, 0xFF21),
            read_bus_reg!(IORegs::Af4, self.addr_bus, 0xFF22),
            read_bus_reg!(IORegs::Ctl4, self.addr_bus, 0xFF23),
            read_bus_reg!(IORegs::AudOutMap, self.addr_bus, 0xFF24),
            read_bus_reg!(IORegs::AudMap, self.addr_bus, 0xFF25),
            read_bus_reg!(IORegs::AudChanCtl, self.addr_bus, 0xFF26),
            read_bus_reg!(IORegs::AudWave, self.addr_bus, 0xFF30),
        ]
    }
}
