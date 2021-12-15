use gb_bus::{generic::SimpleRW, AddressBus, Bus, IORegBus, Lock, WorkingRam};
use gb_clock::{cycles, Clock};
use gb_cpu::{cpu::Cpu, new_cpu};
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
#[cfg(feature = "debug_render")]
use gb_ppu::{
    SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH, SPRITE_RENDER_HEIGHT, SPRITE_RENDER_WIDTH,
    TILEMAP_DIM, TILESHEET_HEIGHT, TILESHEET_WIDTH,
};
use gb_roms::{
    controllers::{bios, generate_rom_controller, BiosWrapper, MbcController},
    header::AutoSave,
    Header,
};
use gb_timer::Timer;
use std::{cell::RefCell, collections::BTreeMap, ops::DerefMut, path::Path, rc::Rc};

pub struct Context<const WIDTH: usize, const HEIGHT: usize> {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub windows: Windows,
    pub display: RenderImage<WIDTH, HEIGHT>,
    pub joypad: Rc<RefCell<Joypad>>,
    #[cfg(feature = "debug_render")]
    pub debug_render: bool,
}

pub struct Windows {
    pub main: GBWindow,
    pub debug: Option<GBWindow>,
    pub input: Option<GBWindow>,
    #[cfg(feature = "debug_render")]
    pub tilemap: Option<(GBWindow, RenderImage<TILEMAP_DIM, TILEMAP_DIM>, bool)>,
    #[cfg(feature = "debug_render")]
    pub tilesheet: Option<(GBWindow, RenderImage<TILESHEET_WIDTH, TILESHEET_HEIGHT>)>,
    #[cfg(feature = "debug_render")]
    pub oam: Option<(
        GBWindow,
        RenderImage<SPRITE_RENDER_WIDTH, SPRITE_RENDER_HEIGHT>,
        RenderImage<SPRITE_LIST_RENDER_WIDTH, SPRITE_LIST_RENDER_HEIGHT>,
        bool,
    )>,
}

pub struct Game {
    pub romname: String,
    pub header: Header,
    pub auto_save: Option<AutoSave>,
    pub mbc: Rc<RefCell<MbcController>>,
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub clock: Clock,
    pub io_bus: Rc<RefCell<IORegBus>>,
    pub timer: Rc<RefCell<Timer>>,
    pub dma: Rc<RefCell<Dma>>,
    pub joypad: Rc<RefCell<Joypad>>,
    pub addr_bus: AddressBus,
    scheduled_stop: Option<ScheduledStop>,
    emulation_stopped: bool,
    cycle_count: usize,
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
    pub fn new<P: AsRef<Path>>(
        rompath: &P,
        joypad: Rc<RefCell<Joypad>>,
        stopped: bool,
    ) -> Result<Game, anyhow::Error> {
        use std::{fs::File, io::Seek};

        let romname = rompath.as_ref().to_string_lossy().to_string();
        let mut file = File::open(rompath)?;
        let header = Header::from_file(&mut file)?;

        log::debug!("header: {:?}", header);

        file.rewind()?;
        let mbc = mbc_with_save_state(&romname, &header, file)?;
        let mbc = Rc::new(RefCell::new(mbc));

        let ppu = Ppu::new();
        let ppu_mem = Rc::new(RefCell::new(ppu.memory()));
        let ppu_reg = Rc::new(RefCell::new(ppu.registers()));
        let (cpu, cpu_io_reg) = new_cpu();
        let wram = Rc::new(RefCell::new(WorkingRam::new(false)));
        let timer = Rc::new(RefCell::new(Timer::default()));
        let bios_wrapper = if cfg!(feature = "bios") {
            let bios = Rc::new(RefCell::new(if cfg!(feature = "cgb") {
                bios::cgb()
            } else {
                bios::dmg()
            }));
            Rc::new(RefCell::new(BiosWrapper::new(bios, mbc.clone())))
        } else {
            todo!("allow to run without the bios")
        };
        let dma = Rc::new(RefCell::new(Dma::new()));

        let io_bus = Rc::new(RefCell::new(IORegBus {
            controller: joypad.clone(),
            communication: Rc::new(RefCell::new(SimpleRW::<2>::default())), // We don't handle communication
            div_timer: timer.clone(),
            tima: timer.clone(),
            tma: timer.clone(),
            tac: timer.clone(),
            sound: Rc::new(RefCell::new(SimpleRW::<0x17>::default())), // We don't handle sound
            waveform_ram: Rc::new(RefCell::new(SimpleRW::<0x10>::default())), // We don't handle sound
            lcd: ppu_reg.clone(),
            oam_dma: dma.clone(),
            #[cfg(feature = "cgb")]
            vram_bank: ppu_reg.clone(),
            boot_rom: bios_wrapper.clone(),
            #[cfg(feature = "cgb")]
            vram_dma: Rc::new(RefCell::new(SimpleRW::<6>::default())), // TODO: link the part that handle the DMA
            bg_obj_palettes: ppu_reg,
            #[cfg(feature = "cgb")]
            wram_bank: wram.clone(),
            interrupt_flag: cpu_io_reg.clone(),
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

            ie_reg: cpu_io_reg,
            area_locks: BTreeMap::new(),
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
            cycle_count: 0,
        })
    }

    pub fn cycle(&mut self) -> bool {
        if !self.emulation_stopped {
            let frame_not_finished = cycles!(
                self.clock,
                &mut self.addr_bus,
                &mut self.cpu,
                &mut self.ppu,
                self.timer.borrow_mut().deref_mut(),
                self.joypad.borrow_mut().deref_mut(),
                self.dma.borrow_mut().deref_mut()
            );
            self.cycle_count += 1;
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

/// Return an initalised MBCs with it auto game save if possible
fn mbc_with_save_state(
    romname: &str,
    header: &Header,
    file: std::fs::File,
) -> anyhow::Result<MbcController> {
    let mut mbc = generate_rom_controller(file, header.clone())?;

    {
        use gb_roms::controllers::MbcStates;
        use rmp_serde::decode::from_read;
        use std::fs::File;

        let filename = game_save_path(romname);
        if let Ok(file) = File::open(&filename) {
            log::info!("found auto save file at {}", filename);
            if let Err(e) = from_read(file).map(|state: MbcStates| mbc.with_state(state)) {
                log::error!(
                    "while loading data into mbc, got the following error: {}",
                    e
                )
            } else {
                log::info!("successfuly load mbc data from {}", filename);
            }
        }
    }

    Ok(mbc)
}

impl Drop for Game {
    fn drop(&mut self) {
        if self.auto_save == Some(AutoSave::Ram) || self.auto_save == Some(AutoSave::RamTimer) {
            use anyhow::Error;
            use core::ops::Deref;
            use rmp_serde::encode::write_named;
            use std::fs::OpenOptions;

            let filename = game_save_path(&self.romname);
            match OpenOptions::new()
                .create(true)
                .write(true)
                .open(&filename)
                .map_err(Error::from)
                .and_then(|mut file| {
                    write_named(&mut file, self.mbc.borrow().deref()).map_err(Error::from)
                }) {
                Ok(_) => log::info!("successfuly save mbc data to {}", filename),
                Err(e) => {
                    log::error!("failed to save mbc data to {}, got error: {}", filename, e)
                }
            }
        }
    }
}

/// Return the path where the game save file will be located
fn game_save_path(rom_filename: &str) -> String {
    use sdl2::filesystem::pref_path;

    let rom_id = game_id(rom_filename);
    let root =
        pref_path(crate::constant::ORG_NAME, crate::constant::APP_NAME).expect("a prefered config");
    std::path::Path::new(&root)
        .join(format!("{}-game-save.msgpack", rom_id))
        .to_string_lossy()
        .to_string()
}

/// Create a standardize rom name id
fn game_id(rom_filename: &str) -> String {
    let rom_path = Path::new(rom_filename);
    rom_path
        .file_stem()
        .map_or_else(
            || rom_filename.to_string(),
            |filename| filename.to_string_lossy().to_string(),
        )
        .replace(" ", "-")
        .to_lowercase()
}

impl DebugOperations for Game {
    fn cycle(&self) -> usize {
        self.cycle_count
    }
}

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
            CpuRegs::AF => self.cpu.registers.af.into(),
            CpuRegs::BC => self.cpu.registers.bc.into(),
            CpuRegs::DE => self.cpu.registers.de.into(),
            CpuRegs::HL => self.cpu.registers.hl.into(),
            CpuRegs::SP => self.cpu.registers.sp.into(),
            CpuRegs::PC => self.cpu.registers.pc.into(),
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
            RegisterMap(CpuRegs::AF, self.cpu.registers.af.into()),
            RegisterMap(CpuRegs::BC, self.cpu.registers.bc.into()),
            RegisterMap(CpuRegs::DE, self.cpu.registers.de.into()),
            RegisterMap(CpuRegs::HL, self.cpu.registers.hl.into()),
            RegisterMap(CpuRegs::SP, self.cpu.registers.sp.into()),
            RegisterMap(CpuRegs::PC, self.cpu.registers.pc.into()),
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
