#[cfg(feature = "cgb")]
use gb_bus::generic::CharDevice;
use gb_bus::{
    generic::SimpleRW, AddressBus, Bus, IORegArea, IORegBus, IORegBusBuilder, Lock, WorkingRam,
};
use gb_clock::{cycles, Clock};
use gb_cpu::{cpu::Cpu, new_cpu, registers::Registers};
use gb_dbg::{
    dbg_interfaces::{
        AudioRegs, CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs,
        RegisterDebugOperations, RegisterMap, RegisterValue,
    },
    until::Until,
};
use gb_dma::Dma;
use gb_joypad::Joypad;
use gb_lcd::render::{RenderImage, SCREEN_HEIGHT, SCREEN_WIDTH};
use gb_ppu::Ppu;
use gb_roms::{
    controllers::{bios, generate_rom_controller, BiosWrapper, Generic},
    header::AutoSave,
    Header,
};
use gb_timer::Timer;
use std::{cell::RefCell, collections::BTreeMap, ops::DerefMut, path::Path, rc::Rc};

pub struct Context<const WIDTH: usize, const HEIGHT: usize> {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub windows: crate::windows::Windows,
    pub display: RenderImage<WIDTH, HEIGHT>,
    pub joypad: Rc<RefCell<Joypad>>,
    #[cfg(feature = "debug_render")]
    pub debug_render: bool,
}

pub struct Game {
    pub romname: String,
    pub header: Header,
    pub auto_save: Option<AutoSave>,
    pub mbc: Rc<RefCell<Generic>>,
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
    /// Schedule a stop after `usize` cycle
    Cycle(usize),
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
        let (cpu, cpu_io_reg) = if cfg!(feature = "bios") {
            new_cpu()
        } else {
            let (mut cpu, cpu_io_reg) = new_cpu();
            cpu.set_registers(Registers::DMG);
            (cpu, cpu_io_reg)
        };
        let wram = Rc::new(RefCell::new(WorkingRam::new(false)));
        let timer = Rc::new(RefCell::new(Timer::default()));
        let bios_wrapper = {
            let bios = Rc::new(RefCell::new(if cfg!(feature = "cgb") {
                bios::cgb()
            } else {
                bios::dmg()
            }));
            let wrapper = if cfg!(feature = "bios") {
                BiosWrapper::new(bios, mbc.clone())
            } else {
                let mut wp = BiosWrapper::new(bios, mbc.clone());
                wp.bios_enabling_reg = 0xa;
                wp
            };
            Rc::new(RefCell::new(wrapper))
        };
        let dma = Rc::new(RefCell::new(Dma::new()));
        let serial = Rc::new(RefCell::new(gb_bus::Serial::default()));

        let io_bus = {
            let mut bus_builder = IORegBusBuilder::default();
            bus_builder
                .with_area(IORegArea::Joy, joypad.clone())
                .with_area(IORegArea::Div, timer.clone())
                .with_area(IORegArea::Tima, timer.clone())
                .with_area(IORegArea::Tma, timer.clone())
                .with_area(IORegArea::Tac, timer.clone())
                .with_area(IORegArea::IF, cpu_io_reg.clone())
                .with_area(IORegArea::LcdControl, ppu_reg.clone())
                .with_area(IORegArea::LcdStat, ppu_reg.clone())
                .with_area(IORegArea::Scy, ppu_reg.clone())
                .with_area(IORegArea::Scx, ppu_reg.clone())
                .with_area(IORegArea::Ly, ppu_reg.clone())
                .with_area(IORegArea::Lyc, ppu_reg.clone())
                .with_area(IORegArea::Dma, dma.clone())
                .with_area(IORegArea::Bgp, ppu_reg.clone())
                .with_area(IORegArea::Obp0, ppu_reg.clone())
                .with_area(IORegArea::Obp1, ppu_reg.clone())
                .with_area(IORegArea::Wy, ppu_reg.clone())
                .with_area(IORegArea::Wx, ppu_reg)
                .with_area(IORegArea::BootRom, bios_wrapper.clone())
                .with_area(IORegArea::SC, serial.clone())
                .with_area(IORegArea::SB, serial)
                .with_default_sound()
                .with_default_waveform_ram();

            #[cfg(feature = "cgb")]
            {
                bus_builder
                    .with_area(IORegArea::Vbk, ppu_reg.clone())
                    .with_area(IORegArea::Hdma1, Rc::new(RefCell::new(CharDevice(0))))
                    .with_area(IORegArea::Hdma2, Rc::new(RefCell::new(CharDevice(0))))
                    .with_area(IORegArea::Hdma3, Rc::new(RefCell::new(CharDevice(0))))
                    .with_area(IORegArea::Hdma4, Rc::new(RefCell::new(CharDevice(0))))
                    .with_area(IORegArea::Hdma5, Rc::new(RefCell::new(CharDevice(0))))
                    .with_area(IORegArea::Key1, cpu_io_reg.clone())
                    .with_area(IORegArea::Svbk, wram.clone());
            }
            bus_builder.build()
        };
        let io_bus = Rc::new(RefCell::new(io_bus));

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
                ScheduledStop::Cycle(count) => {
                    if *count == 1 {
                        self.emulation_stopped = true;
                        self.scheduled_stop = None;
                    } else {
                        *count -= 1;
                    }
                }
                ScheduledStop::Step(count) => {
                    if self.cpu.controller.is_instruction_finished {
                        if *count == 1 {
                            self.emulation_stopped = true;
                            self.scheduled_stop = None;
                        } else {
                            *count -= 1;
                        }
                    }
                }
                ScheduledStop::Frame(count) => {
                    if frame_ended {
                        if *count == 1 {
                            self.finish_instruction();
                        } else {
                            *count -= 1;
                        }
                    }
                }
                ScheduledStop::Timeout(instant, timeout) => {
                    if &instant.elapsed() > timeout {
                        self.finish_instruction();
                    }
                }
            }
        }
    }

    fn finish_instruction(&mut self) {
        if !self.cpu.controller.is_instruction_finished {
            self.scheduled_stop = Some(ScheduledStop::Step(1));
        } else {
            self.emulation_stopped = true;
            self.scheduled_stop = None;
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
            Break(Until::Null | Until::Cycle(0) | Until::Frame(0) | Until::Second(0)) => {
                self.emulation_stopped = true;
                self.scheduled_stop = None;
            }
            Break(Until::Cycle(count)) => {
                self.emulation_stopped = false;
                self.scheduled_stop = Some(ScheduledStop::Cycle(count));
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

    #[cfg(feature = "save_state")]
    /// Save the current game state to a file
    pub fn save_state(&self, filename: &Path) {
        use anyhow::Error;
        use rmp_serde::encode::write_named;
        use std::fs::OpenOptions;

        let minimal_state = MinimalState::from(self);
        if let Err(e) = OpenOptions::new()
            .create(true)
            .write(true)
            .open(filename)
            .map_err(Error::from)
            .and_then(|mut writer| Ok(write_named(&mut writer, &minimal_state)?))
        {
            log::error!(
                "failed to save the game context to {}: {}",
                filename.to_string_lossy(),
                e
            );
        } else {
            log::info!(
                "successfuly save the current game state of {}",
                self.romname
            );
        }
    }

    #[cfg(feature = "save_state")]
    /// Load a game state from a file
    pub fn load_state(&mut self, filename: &Path) {
        use anyhow::Error;
        use rmp_serde::decode::from_read;
        use std::fs::File;

        match File::open(&filename)
            .map_err(Error::from)
            .and_then(|file| Ok(from_read::<File, MinimalState>(file)?))
        {
            Ok(minimal_state) => {
                todo!("load minimal state {:?}", minimal_state);
            }
            Err(e) => {
                log::error!(
                    "failed to load game state from {}: {}",
                    filename.to_string_lossy(),
                    e
                );
            }
        }
    }
}

/// Return an initalised MBCs with it auto game save if possible
fn mbc_with_save_state(
    romname: &str,
    header: &Header,
    file: std::fs::File,
) -> anyhow::Result<Generic> {
    let mut mbc = generate_rom_controller(file, header.clone())?;

    {
        use gb_roms::controllers::GenericState;
        use rmp_serde::decode::from_read;
        use std::fs::File;

        let filename = game_save_path(romname);
        if let Ok(file) = File::open(&filename) {
            log::info!("found auto save file at {}", filename);
            if let Err(e) = from_read(file).map(|state: GenericState| mbc.load_state(state)) {
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
            use rmp_serde::encode::write_named;
            use std::fs::OpenOptions;

            let filename = game_save_path(&self.romname);
            match OpenOptions::new()
                .create(true)
                .write(true)
                .open(&filename)
                .map_err(Error::from)
                .and_then(|mut file| {
                    write_named(&mut file, &self.mbc.borrow().save_state()).map_err(Error::from)
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
pub fn game_save_path(rom_filename: &str) -> String {
    let rom_id = game_id(rom_filename);
    let root = game_root_config_path();

    std::path::Path::new(&root)
        .join(format!("{}.{}", rom_id, crate::constant::GAME_SAVE_EXT))
        .to_string_lossy()
        .to_string()
}

/// Return the root path of the config folder
pub fn game_root_config_path() -> String {
    let path = sdl2::filesystem::pref_path(crate::constant::ORG_NAME, crate::constant::APP_NAME)
        .expect("a prefered config");
    if cfg!(target_os = "macos") {
        path.replace(' ', "%20")
    } else {
        path
    }
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
        RegisterMap($type, read_bus_reg!($bus, u16::from($addr)))
    };

    ($bus:expr, $addr:expr) => {
        $bus.read(u16::from($addr), Some(Lock::Debugger))
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
        use gb_bus::io_reg_area::IORegArea::{
            Bgp, Dma, LcdControl, LcdStat, Ly, Lyc, Obp0, Obp1, Scx, Scy, Wx, Wy,
        };

        match key {
            PpuRegs::Control => read_bus_reg!(self.addr_bus, LcdControl),
            PpuRegs::Status => read_bus_reg!(self.addr_bus, LcdStat),
            PpuRegs::Scy => read_bus_reg!(self.addr_bus, Scy),
            PpuRegs::Scx => read_bus_reg!(self.addr_bus, Scx),
            PpuRegs::Ly => read_bus_reg!(self.addr_bus, Ly),
            PpuRegs::Lyc => read_bus_reg!(self.addr_bus, Lyc),
            PpuRegs::Dma => read_bus_reg!(self.addr_bus, Dma),
            PpuRegs::Bgp => read_bus_reg!(self.addr_bus, Bgp),
            PpuRegs::Obp0 => read_bus_reg!(self.addr_bus, Obp0),
            PpuRegs::Obp1 => read_bus_reg!(self.addr_bus, Obp1),
            PpuRegs::Wy => read_bus_reg!(self.addr_bus, Wy),
            PpuRegs::Wx => read_bus_reg!(self.addr_bus, Wx),
        }
    }

    fn io_get(&self, key: IORegs) -> RegisterValue {
        use gb_bus::constant::IE_REG;
        use gb_bus::io_reg_area::IORegArea::{BootRom, Div, Joy, Tac, Tima, Tma, IF, SB, SC};
        #[cfg(feature = "cgb")]
        use gb_bus::io_reg_area::IORegArea::{Hdma1, Hdma2, Hdma3, Hdma4, Hdma5, Key1, Svbk, Vbk};

        match key {
            // joypad regs
            IORegs::Joy => read_bus_reg!(self.addr_bus, Joy),
            // serial regs
            IORegs::SerialByte => read_bus_reg!(self.addr_bus, SB),
            IORegs::SerialCtl => read_bus_reg!(self.addr_bus, SC),
            // Timer regs
            IORegs::Div => read_bus_reg!(self.addr_bus, Div),
            IORegs::Tima => read_bus_reg!(self.addr_bus, Tima),
            IORegs::Tma => read_bus_reg!(self.addr_bus, Tma),
            IORegs::Tac => read_bus_reg!(self.addr_bus, Tac),
            // cpu int regs
            IORegs::If => read_bus_reg!(self.addr_bus, IF),
            IORegs::Ie => read_bus_reg!(self.addr_bus, IE_REG),
            // Boot ROM
            IORegs::BootRom => read_bus_reg!(self.addr_bus, BootRom),
            #[cfg(feature = "cgb")]
            IORegs::Key1 => read_bus_reg!(self.addr_bus, Key1),
            #[cfg(feature = "cgb")]
            IORegs::VramBank => read_bus_reg!(self.addr_bus, Vbk),
            #[cfg(feature = "cgb")]
            IORegs::WRamBank => read_bus_reg!(self.addr_bus, Svbk),
            #[cfg(feature = "cgb")]
            IORegs::VramDma => read_bus_reg!(self.addr_bus, Hdma1),
            #[cfg(feature = "cgb")]
            IORegs::VramDma => read_bus_reg!(self.addr_bus, Hdma2),
            #[cfg(feature = "cgb")]
            IORegs::VramDma => read_bus_reg!(self.addr_bus, Hdma3),
            #[cfg(feature = "cgb")]
            IORegs::VramDma => read_bus_reg!(self.addr_bus, Hdma4),
            #[cfg(feature = "cgb")]
            IORegs::VramDma => read_bus_reg!(self.addr_bus, Hdma5),
        }
    }

    fn audio_get(&self, key: AudioRegs) -> RegisterValue {
        use gb_bus::io_reg_area::IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52,
        };

        match key {
            AudioRegs::Fs1 => read_bus_reg!(self.addr_bus, Nr10),
            AudioRegs::Pwm1 => read_bus_reg!(self.addr_bus, Nr11),
            AudioRegs::Env1 => read_bus_reg!(self.addr_bus, Nr12),
            AudioRegs::Af1 => read_bus_reg!(self.addr_bus, Nr13),
            AudioRegs::Ctl1 => read_bus_reg!(self.addr_bus, Nr14),
            AudioRegs::Pwm2 => read_bus_reg!(self.addr_bus, Nr21),
            AudioRegs::Env2 => read_bus_reg!(self.addr_bus, Nr22),
            AudioRegs::Af2 => read_bus_reg!(self.addr_bus, Nr23),
            AudioRegs::Ctl2 => read_bus_reg!(self.addr_bus, Nr24),
            AudioRegs::A3Toggle => read_bus_reg!(self.addr_bus, Nr30),
            AudioRegs::Pwm3 => read_bus_reg!(self.addr_bus, Nr31),
            AudioRegs::Vol3 => read_bus_reg!(self.addr_bus, Nr32),
            AudioRegs::Af3 => read_bus_reg!(self.addr_bus, Nr33),
            AudioRegs::Ctl3 => read_bus_reg!(self.addr_bus, Nr34),
            AudioRegs::Pwm4 => read_bus_reg!(self.addr_bus, Nr41),
            AudioRegs::Vol4 => read_bus_reg!(self.addr_bus, Nr42),
            AudioRegs::Af4 => read_bus_reg!(self.addr_bus, Nr43),
            AudioRegs::Ctl4 => read_bus_reg!(self.addr_bus, Nr44),
            AudioRegs::AudOutMap => read_bus_reg!(self.addr_bus, Nr44),
            AudioRegs::AudMap => read_bus_reg!(self.addr_bus, Nr50),
            AudioRegs::AudChanCtl => read_bus_reg!(self.addr_bus, Nr51),
            AudioRegs::AudWave => read_bus_reg!(self.addr_bus, Nr52),
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
        use gb_bus::io_reg_area::IORegArea::{
            Bgp, Dma, LcdControl, LcdStat, Ly, Lyc, Obp0, Obp1, Scx, Scy, Wx, Wy,
        };

        vec![
            read_bus_reg!(PpuRegs::Control, self.addr_bus, LcdControl),
            read_bus_reg!(PpuRegs::Status, self.addr_bus, LcdStat),
            read_bus_reg!(PpuRegs::Scy, self.addr_bus, Scy),
            read_bus_reg!(PpuRegs::Scx, self.addr_bus, Scx),
            read_bus_reg!(PpuRegs::Ly, self.addr_bus, Ly),
            read_bus_reg!(PpuRegs::Lyc, self.addr_bus, Lyc),
            read_bus_reg!(PpuRegs::Dma, self.addr_bus, Dma),
            read_bus_reg!(PpuRegs::Bgp, self.addr_bus, Bgp),
            read_bus_reg!(PpuRegs::Obp0, self.addr_bus, Obp0),
            read_bus_reg!(PpuRegs::Obp1, self.addr_bus, Obp1),
            read_bus_reg!(PpuRegs::Wy, self.addr_bus, Wy),
            read_bus_reg!(PpuRegs::Wx, self.addr_bus, Wx),
        ]
    }

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>> {
        use gb_bus::constant::IE_REG;
        use gb_bus::io_reg_area::IORegArea::{BootRom, Div, Joy, Tac, Tima, Tma, IF, SB, SC};
        #[cfg(feature = "cgb")]
        use gb_bus::io_reg_area::IORegArea::{Hdma1, Hdma2, Hdma3, Hdma4, Hdma5, Key1, Svbk, Vbk};

        vec![
            // joypad regs
            read_bus_reg!(IORegs::Joy, self.addr_bus, Joy),
            // serial regs
            read_bus_reg!(IORegs::SerialByte, self.addr_bus, SB),
            read_bus_reg!(IORegs::SerialCtl, self.addr_bus, SC),
            // Timer regs
            read_bus_reg!(IORegs::Div, self.addr_bus, Div),
            read_bus_reg!(IORegs::Tima, self.addr_bus, Tima),
            read_bus_reg!(IORegs::Tma, self.addr_bus, Tma),
            read_bus_reg!(IORegs::Tac, self.addr_bus, Tac),
            // cpu int regs
            read_bus_reg!(IORegs::If, self.addr_bus, IF),
            read_bus_reg!(IORegs::Ie, self.addr_bus, IE_REG),
            // Boot ROM
            read_bus_reg!(IORegs::BootRom, self.addr_bus, BootRom),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramBank, self.addr_bus, Vbk),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::Key1, self.addr_bus, Key1),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::WRamBank, self.addr_bus, Svbk),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramDma, self.addr_bus, Hdma1),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramDma, self.addr_bus, Hdma2),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramDma, self.addr_bus, Hdma3),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramDma, self.addr_bus, Hdma4),
            #[cfg(feature = "cgb")]
            read_bus_reg!(IORegs::VramDma, self.addr_bus, Hdma5),
        ]
    }

    fn audio_registers(&self) -> Vec<RegisterMap<AudioRegs>> {
        use gb_bus::io_reg_area::IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52,
        };

        vec![
            read_bus_reg!(AudioRegs::Fs1, self.addr_bus, Nr10),
            read_bus_reg!(AudioRegs::Pwm1, self.addr_bus, Nr11),
            read_bus_reg!(AudioRegs::Env1, self.addr_bus, Nr12),
            read_bus_reg!(AudioRegs::Af1, self.addr_bus, Nr13),
            read_bus_reg!(AudioRegs::Ctl1, self.addr_bus, Nr14),
            read_bus_reg!(AudioRegs::Pwm2, self.addr_bus, Nr21),
            read_bus_reg!(AudioRegs::Env2, self.addr_bus, Nr22),
            read_bus_reg!(AudioRegs::Af2, self.addr_bus, Nr23),
            read_bus_reg!(AudioRegs::Ctl2, self.addr_bus, Nr24),
            read_bus_reg!(AudioRegs::A3Toggle, self.addr_bus, Nr30),
            read_bus_reg!(AudioRegs::Pwm3, self.addr_bus, Nr31),
            read_bus_reg!(AudioRegs::Vol3, self.addr_bus, Nr32),
            read_bus_reg!(AudioRegs::Af3, self.addr_bus, Nr33),
            read_bus_reg!(AudioRegs::Ctl3, self.addr_bus, Nr34),
            read_bus_reg!(AudioRegs::Pwm4, self.addr_bus, Nr41),
            read_bus_reg!(AudioRegs::Vol4, self.addr_bus, Nr42),
            read_bus_reg!(AudioRegs::Af4, self.addr_bus, Nr43),
            read_bus_reg!(AudioRegs::Ctl4, self.addr_bus, Nr44),
            read_bus_reg!(AudioRegs::AudOutMap, self.addr_bus, Nr44),
            read_bus_reg!(AudioRegs::AudMap, self.addr_bus, Nr50),
            read_bus_reg!(AudioRegs::AudChanCtl, self.addr_bus, Nr51),
            read_bus_reg!(AudioRegs::AudWave, self.addr_bus, Nr52),
        ]
    }
}

#[cfg(feature = "save_state")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MinimalState {
    pub romname: String,
}

#[cfg(feature = "save_state")]
impl From<&Game> for MinimalState {
    fn from(context: &Game) -> Self {
        Self {
            romname: context.romname.clone(),
        }
    }
}
