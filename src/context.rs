use gb_bus::{generic::SimpleRW, AddressBus, Bus, IORegBus, Lock, WorkingRam};
use gb_clock::{cycles, Clock};
use gb_cpu::cpu::Cpu;
use gb_dbg::dbg_interfaces::AudioRegs;
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
use std::{cell::RefCell, ops::DerefMut, rc::Rc};

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
    pub clock: Clock,
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
            let frame_not_finished = cycles!(
                self.clock,
                &mut self.addr_bus,
                self.cpu.borrow_mut().deref_mut(),
                &mut self.ppu,
                self.timer.borrow_mut().deref_mut(),
                self.joypad.borrow_mut().deref_mut(),
                self.dma.borrow_mut().deref_mut()
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

impl Drop for Game {
    fn drop(&mut self) {
        if self.auto_save == Some(AutoSave::Ram) || self.auto_save == Some(AutoSave::RamTimer) {
            use bincode::{config::Configuration, encode_into_std_write, serde::encode_to_vec};
            use core::ops::Deref;
            use std::fs::OpenOptions;

            let save_file = format!("/tmp/gbmu/saves/auto/{}/ram.bin", self.romname);
            let mut save_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(save_file)
                .expect("cannot open auto save file");
            let config = Configuration::standard()
                .with_big_endian()
                .with_fixed_int_encoding()
                .write_fixed_array_length();
            let data = encode_to_vec(self.mbc.borrow().deref(), config)
                .expect("cannot serialize mbc data");
            encode_into_std_write(data, &mut save_file, config)
                .expect("cannot serialize mbc data to file");
            todo!("auto save");
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
    fn cpu_get(&self, key: CpuRegs) -> RegisterValue {
        match key {
            CpuRegs::AF => self.cpu.borrow().registers.af.into(),
            CpuRegs::BC => self.cpu.borrow().registers.bc.into(),
            CpuRegs::DE => self.cpu.borrow().registers.de.into(),
            CpuRegs::HL => self.cpu.borrow().registers.hl.into(),
            CpuRegs::SP => self.cpu.borrow().registers.sp.into(),
            CpuRegs::PC => self.cpu.borrow().registers.pc.into(),
        }
    }

    fn ppu_get(&self, key: PpuRegs) -> RegisterValue {
        use gb_bus::io_reg_constant::{
            PPU_BGP, PPU_CONTROL, PPU_DMA, PPU_LY, PPU_LYC, PPU_OBP0, PPU_OBP1, PPU_SCX, PPU_SCY,
            PPU_STATUS, PPU_WX, PPU_WY,
        };

        match key {
            PpuRegs::Control => read_bus_reg!(self.addr_bus, PPU_CONTROL),
            PpuRegs::Status => read_bus_reg!(self.addr_bus, PPU_STATUS),
            PpuRegs::Scy => read_bus_reg!(self.addr_bus, PPU_SCY),
            PpuRegs::Scx => read_bus_reg!(self.addr_bus, PPU_SCX),
            PpuRegs::Ly => read_bus_reg!(self.addr_bus, PPU_LY),
            PpuRegs::Lyc => read_bus_reg!(self.addr_bus, PPU_LYC),
            PpuRegs::Dma => read_bus_reg!(self.addr_bus, PPU_DMA),
            PpuRegs::Bgp => read_bus_reg!(self.addr_bus, PPU_BGP),
            PpuRegs::Obp0 => read_bus_reg!(self.addr_bus, PPU_OBP0),
            PpuRegs::Obp1 => read_bus_reg!(self.addr_bus, PPU_OBP1),
            PpuRegs::Wy => read_bus_reg!(self.addr_bus, PPU_WY),
            PpuRegs::Wx => read_bus_reg!(self.addr_bus, PPU_WX),
        }
    }

    fn io_get(&self, key: IORegs) -> RegisterValue {
        use gb_bus::io_reg_constant::{
            IO_BOOTROM, IO_DIV, IO_IE, IO_IF, IO_JOY, IO_SERIALBYTE, IO_SERIALCTL, IO_TAC, IO_TIMA,
            IO_TMA,
        };

        match key {
            // joypad regs
            IORegs::Joy => read_bus_reg!(self.addr_bus, IO_JOY),
            // serial regs
            IORegs::SerialByte => read_bus_reg!(self.addr_bus, IO_SERIALBYTE),
            IORegs::SerialCtl => read_bus_reg!(self.addr_bus, IO_SERIALCTL),
            // Timer regs
            IORegs::Div => read_bus_reg!(self.addr_bus, IO_DIV),
            IORegs::Tima => read_bus_reg!(self.addr_bus, IO_TIMA),
            IORegs::Tma => read_bus_reg!(self.addr_bus, IO_TMA),
            IORegs::Tac => read_bus_reg!(self.addr_bus, IO_TAC),
            // cpu int regs
            IORegs::If => read_bus_reg!(self.addr_bus, IO_IF),
            IORegs::Ie => read_bus_reg!(self.addr_bus, IO_IE),
            // Boot ROM
            IORegs::BootRom => read_bus_reg!(self.addr_bus, IO_BOOTROM),
        }
    }

    fn audio_get(&self, key: AudioRegs) -> RegisterValue {
        use gb_bus::io_reg_constant::{
            AUD_A3TOGGLE, AUD_AF1, AUD_AF2, AUD_AF3, AUD_AF4, AUD_CHANNEL_CTL, AUD_CTL1, AUD_CTL2,
            AUD_CTL3, AUD_CTL4, AUD_ENV1, AUD_ENV2, AUD_FS1, AUD_MAP, AUD_OUTPUT_MAP, AUD_PWM1,
            AUD_PWM2, AUD_PWM3, AUD_PWM4, AUD_VOL3, AUD_VOL4, AUD_WAVE,
        };

        match key {
            AudioRegs::Fs1 => read_bus_reg!(self.addr_bus, AUD_FS1),
            AudioRegs::Pwm1 => read_bus_reg!(self.addr_bus, AUD_PWM1),
            AudioRegs::Env1 => read_bus_reg!(self.addr_bus, AUD_ENV1),
            AudioRegs::Af1 => read_bus_reg!(self.addr_bus, AUD_AF1),
            AudioRegs::Ctl1 => read_bus_reg!(self.addr_bus, AUD_CTL1),
            AudioRegs::Pwm2 => read_bus_reg!(self.addr_bus, AUD_PWM2),
            AudioRegs::Env2 => read_bus_reg!(self.addr_bus, AUD_ENV2),
            AudioRegs::Af2 => read_bus_reg!(self.addr_bus, AUD_AF2),
            AudioRegs::Ctl2 => read_bus_reg!(self.addr_bus, AUD_CTL2),
            AudioRegs::A3Toggle => read_bus_reg!(self.addr_bus, AUD_A3TOGGLE),
            AudioRegs::Pwm3 => read_bus_reg!(self.addr_bus, AUD_PWM3),
            AudioRegs::Vol3 => read_bus_reg!(self.addr_bus, AUD_VOL3),
            AudioRegs::Af3 => read_bus_reg!(self.addr_bus, AUD_AF3),
            AudioRegs::Ctl3 => read_bus_reg!(self.addr_bus, AUD_CTL3),
            AudioRegs::Pwm4 => read_bus_reg!(self.addr_bus, AUD_PWM4),
            AudioRegs::Vol4 => read_bus_reg!(self.addr_bus, AUD_VOL4),
            AudioRegs::Af4 => read_bus_reg!(self.addr_bus, AUD_AF4),
            AudioRegs::Ctl4 => read_bus_reg!(self.addr_bus, AUD_CTL4),
            AudioRegs::AudOutMap => read_bus_reg!(self.addr_bus, AUD_OUTPUT_MAP),
            AudioRegs::AudMap => read_bus_reg!(self.addr_bus, AUD_MAP),
            AudioRegs::AudChanCtl => read_bus_reg!(self.addr_bus, AUD_CHANNEL_CTL),
            AudioRegs::AudWave => read_bus_reg!(self.addr_bus, AUD_WAVE),
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
        use gb_bus::io_reg_constant::{
            PPU_BGP, PPU_CONTROL, PPU_DMA, PPU_LY, PPU_LYC, PPU_OBP0, PPU_OBP1, PPU_SCX, PPU_SCY,
            PPU_STATUS, PPU_WX, PPU_WY,
        };

        vec![
            // lcd regs
            read_bus_reg!(PpuRegs::Control, self.addr_bus, PPU_CONTROL),
            read_bus_reg!(PpuRegs::Status, self.addr_bus, PPU_STATUS),
            read_bus_reg!(PpuRegs::Scy, self.addr_bus, PPU_SCY),
            read_bus_reg!(PpuRegs::Scx, self.addr_bus, PPU_SCX),
            read_bus_reg!(PpuRegs::Ly, self.addr_bus, PPU_LY),
            read_bus_reg!(PpuRegs::Lyc, self.addr_bus, PPU_LYC),
            read_bus_reg!(PpuRegs::Dma, self.addr_bus, PPU_DMA),
            read_bus_reg!(PpuRegs::Bgp, self.addr_bus, PPU_BGP),
            read_bus_reg!(PpuRegs::Obp0, self.addr_bus, PPU_OBP0),
            read_bus_reg!(PpuRegs::Obp1, self.addr_bus, PPU_OBP1),
            read_bus_reg!(PpuRegs::Wy, self.addr_bus, PPU_WY),
            read_bus_reg!(PpuRegs::Wx, self.addr_bus, PPU_WX),
        ]
    }

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>> {
        use gb_bus::io_reg_constant::{
            IO_BOOTROM, IO_DIV, IO_IE, IO_IF, IO_JOY, IO_SERIALBYTE, IO_SERIALCTL, IO_TAC, IO_TIMA,
            IO_TMA,
        };

        vec![
            // joypad regs
            read_bus_reg!(IORegs::Joy, self.addr_bus, IO_JOY),
            // serial regs
            read_bus_reg!(IORegs::SerialByte, self.addr_bus, IO_SERIALBYTE),
            read_bus_reg!(IORegs::SerialCtl, self.addr_bus, IO_SERIALCTL),
            // Timer regs
            read_bus_reg!(IORegs::Div, self.addr_bus, IO_DIV),
            read_bus_reg!(IORegs::Tima, self.addr_bus, IO_TIMA),
            read_bus_reg!(IORegs::Tma, self.addr_bus, IO_TMA),
            read_bus_reg!(IORegs::Tac, self.addr_bus, IO_TAC),
            // cpu int regs
            read_bus_reg!(IORegs::If, self.addr_bus, IO_IF),
            read_bus_reg!(IORegs::Ie, self.addr_bus, IO_IE),
            // Boot ROM
            read_bus_reg!(IORegs::BootRom, self.addr_bus, IO_BOOTROM),
        ]
    }

    fn audio_registers(&self) -> Vec<RegisterMap<AudioRegs>> {
        use gb_bus::io_reg_constant::{
            AUD_A3TOGGLE, AUD_AF1, AUD_AF2, AUD_AF3, AUD_AF4, AUD_CHANNEL_CTL, AUD_CTL1, AUD_CTL2,
            AUD_CTL3, AUD_CTL4, AUD_ENV1, AUD_ENV2, AUD_FS1, AUD_MAP, AUD_OUTPUT_MAP, AUD_PWM1,
            AUD_PWM2, AUD_PWM3, AUD_PWM4, AUD_VOL3, AUD_VOL4, AUD_WAVE,
        };

        vec![
            read_bus_reg!(AudioRegs::Fs1, self.addr_bus, AUD_FS1),
            read_bus_reg!(AudioRegs::Pwm1, self.addr_bus, AUD_PWM1),
            read_bus_reg!(AudioRegs::Env1, self.addr_bus, AUD_ENV1),
            read_bus_reg!(AudioRegs::Af1, self.addr_bus, AUD_AF1),
            read_bus_reg!(AudioRegs::Ctl1, self.addr_bus, AUD_CTL1),
            read_bus_reg!(AudioRegs::Pwm2, self.addr_bus, AUD_PWM2),
            read_bus_reg!(AudioRegs::Env2, self.addr_bus, AUD_ENV2),
            read_bus_reg!(AudioRegs::Af2, self.addr_bus, AUD_AF2),
            read_bus_reg!(AudioRegs::Ctl2, self.addr_bus, AUD_CTL2),
            read_bus_reg!(AudioRegs::A3Toggle, self.addr_bus, AUD_A3TOGGLE),
            read_bus_reg!(AudioRegs::Pwm3, self.addr_bus, AUD_PWM3),
            read_bus_reg!(AudioRegs::Vol3, self.addr_bus, AUD_VOL3),
            read_bus_reg!(AudioRegs::Af3, self.addr_bus, AUD_AF3),
            read_bus_reg!(AudioRegs::Ctl3, self.addr_bus, AUD_CTL3),
            read_bus_reg!(AudioRegs::Pwm4, self.addr_bus, AUD_PWM4),
            read_bus_reg!(AudioRegs::Vol4, self.addr_bus, AUD_VOL4),
            read_bus_reg!(AudioRegs::Af4, self.addr_bus, AUD_AF4),
            read_bus_reg!(AudioRegs::Ctl4, self.addr_bus, AUD_CTL4),
            read_bus_reg!(AudioRegs::AudOutMap, self.addr_bus, AUD_OUTPUT_MAP),
            read_bus_reg!(AudioRegs::AudMap, self.addr_bus, AUD_MAP),
            read_bus_reg!(AudioRegs::AudChanCtl, self.addr_bus, AUD_CHANNEL_CTL),
            read_bus_reg!(AudioRegs::AudWave, self.addr_bus, AUD_WAVE),
        ]
    }
}
