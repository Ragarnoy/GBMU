#[cfg(feature = "registers_logs")]
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, fs::File, ops::DerefMut, path::Path, rc::Rc};

use gb_apu::apu::Apu;
use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Bus, IORegArea, IORegBus, Source, WorkingRam,
};
use gb_clock::{counted_cycles, not_counted_cycles, Clock};
use gb_cpu::{cpu::Cpu, new_cpu, registers::Registers};
use gb_dbg::dbg_interfaces::{
    AudioRegs, CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs,
    RegisterDebugOperations, RegisterMap, RegisterValue,
};
use gb_dbg::until::Until;
use gb_dma::{dma::Dma, hdma::Hdma};
use gb_joypad::Joypad;
use gb_ppu::Ppu;
use gb_roms::controllers::bios::BiosType;
use gb_roms::controllers::Bios;
#[cfg(feature = "save_state")]
use gb_roms::controllers::Full;
use gb_roms::{controllers::Generic, header::AutoSave, Header};
use gb_timer::Timer;
#[cfg(feature = "save_state")]
use save_state::SaveState;
use utils::mbc_with_save_state;

use crate::{
    config::Mode, constant::AUDIO_BUFFER_SIZE, context::configuration::Configuration,
    path::game_save_path,
};

#[cfg(feature = "save_state")]
mod save_state;
mod utils;

macro_rules! cell {
    ($e:expr) => {
        Rc::new(RefCell::new($e))
    };
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
    pub hdma: Rc<RefCell<Hdma>>,
    pub dma: Rc<RefCell<Dma>>,
    pub joypad: Rc<RefCell<Joypad>>,

    pub apu: Rc<RefCell<Apu>>,
    pub addr_bus: AddressBus,
    scheduled_stop: Option<ScheduledStop>,
    emulation_stopped: bool,
    cycle_count: usize,
    #[cfg(feature = "save_state")]
    hram: Rc<RefCell<SimpleRW<0x80>>>,
    #[cfg(feature = "save_state")]
    wram: Rc<RefCell<WorkingRam>>,
    #[cfg(feature = "registers_logs")]
    logs_file: BufWriter<File>,
    pub cgb_mode: bool,
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
        rom_path: &P,
        stopped: bool,
        forced_mode: Option<Mode>,
        configuration: &Configuration,
    ) -> Result<Game, anyhow::Error> {
        use std::io::Seek;

        let romname = rom_path.as_ref().to_string_lossy().to_string();
        let mut file = File::open(rom_path)?;
        let header = Header::from_file(&mut file)?;

        let cgb_mode = if let Some(forced_mode) = forced_mode {
            forced_mode == Mode::Color
        } else {
            header.title.is_cgb_cartridge()
        };

        log::debug!("header: {:?}", header);

        file.rewind()?;

        let bios: Option<Bios> = {
            use gb_roms::controllers::bios::{CGB_BIOS_SIZE, DMG_BIOS_SIZE};
            use std::io::Read;

            if cgb_mode && configuration.bios.enable_cbg {
                configuration.bios.cgb_bios_file.as_ref().and_then(|path| {
                    File::open(path)
                        .map_err(|e| {
                            log::error!("cannot open bios file: {e}");
                        })
                        .and_then(|mut reader| {
                            let mut buf = Box::new([0; CGB_BIOS_SIZE]);
                            if let Err(e) = reader.read_exact(buf.as_mut()) {
                                log::error!("cannot read bios file: {e}");
                                Err(())
                            } else {
                                Ok(Bios::from_bytes(BiosType::Cgb, buf.as_ref()))
                            }
                        })
                        .ok()
                })
            } else if !cgb_mode && configuration.bios.enable_dmg {
                configuration.bios.dmg_bios_file.as_ref().and_then(|path| {
                    File::open(path)
                        .map_err(|e| {
                            log::error!("cannot open bios file: {e}");
                        })
                        .and_then(|mut reader| {
                            let mut buf = [0; DMG_BIOS_SIZE];
                            if let Err(e) = reader.read_exact(&mut buf) {
                                log::error!("cannot read bios file: {e}");
                                Err(())
                            } else {
                                Ok(Bios::from_bytes(BiosType::Dmg, &buf))
                            }
                        })
                        .ok()
                })
            } else {
                None
            }
        };

        let mut io_bus = IORegBus::default();
        let mut bus = AddressBus::default();

        let mbc = mbc_with_save_state(&romname, &header, file)?;
        let mbc = cell!(mbc);
        bus.with_ext_ram(mbc.clone());

        let ppu = Ppu::new(cgb_mode);
        let ppu_mem = cell!(ppu.memory());
        bus.with_vram(ppu_mem.clone());
        bus.with_oam(ppu_mem);

        let ppu_reg = cell!(ppu.registers());
        if bios.is_none() {
            ppu_reg.borrow_mut().overwrite_lcd_control(0x91_u8);
        }
        io_bus.with_ppu(ppu_reg.clone());

        let (cpu, cpu_io_reg) = if bios.is_some() {
            new_cpu(cgb_mode)
        } else {
            let (mut cpu, cpu_io_reg) = new_cpu(cgb_mode);
            cpu.set_registers(if cgb_mode {
                Registers::CGB
            } else {
                Registers::DMG
            });
            (cpu, cpu_io_reg)
        };
        io_bus.with_area(IORegArea::IF, cpu_io_reg.clone());
        bus.with_ie_reg(cpu_io_reg.clone());

        let timer = if bios.is_none() {
            let mut timer = Timer::default();
            timer.system_clock = 0xAC00;
            timer
        } else {
            Timer::default()
        };
        let timer = cell!(timer);
        io_bus.with_timer(timer.clone());

        if let Some(bios) = bios {
            let wrapper =
                gb_roms::controllers::BiosWrapper::new(cell!(bios), mbc.clone(), cgb_mode);
            let wrapper = cell!(wrapper);
            io_bus.with_area(IORegArea::BootRom, wrapper.clone());
            bus.with_rom(wrapper);
        } else {
            io_bus.with_area(
                IORegArea::BootRom,
                cell!(gb_bus::generic::PanicDevice::default()),
            );
            bus.with_rom(mbc.clone());
        }

        let dma = cell!(Dma::new(ppu.memory()));
        io_bus.with_area(IORegArea::Dma, dma.clone());

        let hdma = cell!(Hdma::default());
        io_bus.with_hdma(hdma.clone());

        let serial = cell!(gb_bus::Serial::new(cgb_mode));
        io_bus.with_serial(serial);

        let buffer: Arc<Mutex<Vec<f32>>> =
            Arc::new(Mutex::new(Vec::with_capacity(AUDIO_BUFFER_SIZE)));
        let (stream, sample_rate) = Apu::init_audio_output(buffer.clone());
        let apu = cell!(Apu::new(buffer, Some(stream), sample_rate));
        io_bus.with_sound(apu.clone());

        let joypad = cell!(Joypad::from_config(configuration.input.clone(),));
        io_bus.with_area(IORegArea::Joy, joypad.clone());

        let wram = cell!(WorkingRam::new(cgb_mode));
        bus.with_ram(wram.clone());

        if cgb_mode {
            io_bus
                .with_ppu_cgb(ppu_reg)
                .with_area(IORegArea::Key1, cpu_io_reg)
                .with_area(IORegArea::RP, cell!(CharDevice(0)))
                .with_area(IORegArea::Svbk, wram);
        }

        let hram = cell!(SimpleRW::<0x80>::default());
        bus.with_hram(hram);

        let io_bus = cell!(io_bus);
        bus.with_io_reg(io_bus.clone());

        #[cfg(feature = "registers_logs")]
        let logs_file = Game::create_new_file().unwrap();

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
            hdma,
            joypad,

            apu,
            addr_bus: bus,
            scheduled_stop: None,
            emulation_stopped: stopped,
            cycle_count: 0,
            #[cfg(feature = "save_state")]
            hram,
            #[cfg(feature = "save_state")]
            wram,
            #[cfg(feature = "registers_logs")]
            logs_file,
            cgb_mode,
        })
    }

    pub fn cycle(&mut self) -> bool {
        if !self.emulation_stopped {
            #[cfg(feature = "registers_logs")]
            if self.cpu.controller.is_instruction_finished {
                self.log_registers_to_file().unwrap_or_default();
            }
            self.hdma
                .borrow_mut()
                .check_hdma_state(&mut self.cpu, &self.ppu);

            let frame_not_finished = counted_cycles!(
                self.clock,
                &mut self.addr_bus,
                self.timer.borrow_mut().deref_mut(),
                &mut self.ppu,
                self.joypad.borrow_mut().deref_mut(),
                self.dma.borrow_mut().deref_mut(),
                &mut self.cpu,
                self.hdma.borrow_mut().deref_mut(),
                self.apu.borrow_mut().deref_mut()
            );

            self.check_scheduled_stop(!frame_not_finished);
            if self.cpu.io_regs.borrow().fast_mode() {
                not_counted_cycles!(
                    self.clock,
                    &mut self.addr_bus,
                    &mut self.cpu,
                    self.timer.borrow_mut().deref_mut(),
                    self.dma.borrow_mut().deref_mut()
                );
                self.check_scheduled_stop(!frame_not_finished);
            }

            self.cycle_count += 1;
            frame_not_finished
        } else {
            false
        }
    }
    pub fn is_audio_buffer_full(&self) -> bool {
        (*self.apu.borrow()).is_buffer_full()
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

        let minimal_state = SaveState::from(self);
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
                "successfully save the current game state of {}",
                self.romname
            );
        }
    }

    #[cfg(feature = "save_state")]
    /// Load a game state from a file
    pub fn load_save_file(&mut self, filename: &Path) {
        use anyhow::Error;
        use rmp_serde::decode::from_read;

        match File::open(&filename)
            .map_err(Error::from)
            .and_then(|file| Ok(from_read::<File, SaveState>(file)?))
        {
            Ok(minimal_state) => {
                if let Err(e) = self.load_state(minimal_state) {
                    log::error!("failed to load save state: {}", e)
                }
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

    #[cfg(feature = "save_state")]
    fn load_state(&mut self, state: SaveState) -> anyhow::Result<()> {
        self.load_cpu_state(state.cpu_regs, state.cpu_io_regs)?;
        self.load_wram(state.working_ram)?;
        self.load_timer(state.timer)?;
        self.load_hram(state.hram)?;
        self.load_dma(state.dma)?;
        self.load_hdma(state.hdma)?;

        self.mbc.borrow_mut().load(state.mbcs)?;
        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_hdma(&mut self, hdma: Hdma) -> anyhow::Result<()> {
        self.hdma = Rc::new(RefCell::new(hdma));
        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_hram(&mut self, hram: Vec<u8>) -> anyhow::Result<()> {
        let hram = SimpleRW::try_from(hram)
            .map_err(|size| anyhow::anyhow!("Failed to load HRAM, invalid size {:x}", size))?;
        let hram = Rc::new(RefCell::new(hram));
        self.addr_bus.hram = hram.clone();
        self.hram = hram;

        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_dma(&mut self, state: gb_dma::dma::State) -> anyhow::Result<()> {
        self.dma = Rc::new(RefCell::new(Dma::with_state(state, self.ppu.memory())));
        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_cpu_state(
        &mut self,
        registers: Registers,
        io_regs: gb_cpu::io_registers::IORegisters,
    ) -> anyhow::Result<()> {
        let cpu_io = Rc::new(RefCell::new(io_regs));
        self.cpu.io_regs = cpu_io.clone();
        let mut io_bus = self.io_bus.borrow_mut();
        io_bus.with_area(IORegArea::Key1, cpu_io.clone());
        io_bus.with_area(IORegArea::IF, cpu_io.clone());

        self.cpu.registers = registers;
        self.addr_bus.ie_reg = cpu_io;
        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_wram(&mut self, state: WorkingRam) -> anyhow::Result<()> {
        let wram = Rc::new(RefCell::new(state));
        self.addr_bus.ram = wram.clone();
        self.io_bus
            .borrow_mut()
            .with_area(IORegArea::Svbk, wram.clone());
        self.wram = wram;
        Ok(())
    }

    #[cfg(feature = "save_state")]
    fn load_timer(&mut self, state: Timer) -> anyhow::Result<()> {
        let timer = Rc::new(RefCell::new(state));
        self.io_bus.borrow_mut().with_timer(timer.clone());
        self.timer = timer;
        Ok(())
    }

    #[cfg(feature = "registers_logs")]
    fn log_registers_to_file(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        let file = &mut self.logs_file;
        let timer_borrow = self.timer.borrow();

        if let Err(e) = writeln!(
            file,
            "{} ({:02X} {:02X} {:02X} {:02X}) TIMA: {:02X} TAC: {:02X} CLK: {:04X}",
            self.cpu.registers,
            <AddressBus as Bus<u8>>::read(&self.addr_bus, self.cpu.registers.pc, None)
                .unwrap_or(0xff),
            <AddressBus as Bus<u8>>::read(&self.addr_bus, self.cpu.registers.pc + 1, None)
                .unwrap_or(0xff),
            <AddressBus as Bus<u8>>::read(&self.addr_bus, self.cpu.registers.pc + 2, None)
                .unwrap_or(0xff),
            <AddressBus as Bus<u8>>::read(&self.addr_bus, self.cpu.registers.pc + 3, None)
                .unwrap_or(0xff),
            timer_borrow.tima,
            <AddressBus as Bus<u8>>::read(&self.addr_bus, 0xff07, None).unwrap_or(0xff),
            timer_borrow.system_clock
        ) {
            log::error!("Couldn't write to file: {}", e);
        }
        Ok(())
    }

    #[cfg(feature = "registers_logs")]
    fn create_new_file() -> std::io::Result<BufWriter<File>> {
        use std::{env, fs::OpenOptions};

        let registers_logs = {
            use env::{temp_dir, var};
            let mut project_path =
                var("LOG_DIR").map_or_else(|_| temp_dir(), std::path::PathBuf::from);
            project_path.push("registers.log");
            project_path
        };

        log::info!("opening registers log at {}", registers_logs.display());
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(registers_logs)?;
        Ok(BufWriter::new(file))
    }
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
                    write_named(&mut file, &self.mbc.borrow().save_partial()).map_err(Error::from)
                }) {
                Ok(_) => log::info!(
                    "successfully save mbc data to {}",
                    filename.to_string_lossy()
                ),
                Err(e) => {
                    log::error!(
                        "failed to save mbc data to {}, got error: {}",
                        filename.to_string_lossy(),
                        e
                    )
                }
            }
        }
    }
}

impl DebugOperations for Game {
    fn cycle(&self) -> usize {
        self.cycle_count
    }
}

impl MemoryDebugOperations for Game {
    fn read(&self, index: u16) -> u8 {
        self.addr_bus
            .read(index, Some(Source::Debugger))
            .unwrap_or_else(|err| {
                log::trace!("[DBG-OPS] bus read error at {}: {:?}", index, err);
                0xff
            })
    }
}

macro_rules! read_bus_reg {
    ($type:expr, $bus:expr, $addr:expr) => {
        RegisterMap($type, read_bus_reg!($bus, u16::from($addr)))
    };

    ($bus:expr, $addr:expr) => {
        $bus.read(u16::from($addr), Some(Source::Debugger))
            .unwrap_or(0xff_u8)
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
        use gb_bus::io_reg_area::IORegArea::{
            BootRom, Div, Hdma1, Hdma2, Hdma3, Hdma4, Hdma5, Joy, Key1, Svbk, Tac, Tima, Tma, Vbk,
            IF, SB, SC,
        };

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
            IORegs::Key1 => read_bus_reg!(self.addr_bus, Key1),
            IORegs::VramBank => read_bus_reg!(self.addr_bus, Vbk),
            IORegs::WRamBank => read_bus_reg!(self.addr_bus, Svbk),
            IORegs::Hdma1 => read_bus_reg!(self.addr_bus, Hdma1),
            IORegs::Hdma2 => read_bus_reg!(self.addr_bus, Hdma2),
            IORegs::Hdma3 => read_bus_reg!(self.addr_bus, Hdma3),
            IORegs::Hdma4 => read_bus_reg!(self.addr_bus, Hdma4),
            IORegs::Hdma5 => read_bus_reg!(self.addr_bus, Hdma5),
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
        use gb_bus::io_reg_area::IORegArea::{
            BootRom, Div, Hdma1, Hdma2, Hdma3, Hdma4, Hdma5, Joy, Key1, Svbk, Tac, Tima, Tma, Vbk,
            IF, SB, SC,
        };

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
            read_bus_reg!(IORegs::VramBank, self.addr_bus, Vbk),
            read_bus_reg!(IORegs::Key1, self.addr_bus, Key1),
            read_bus_reg!(IORegs::WRamBank, self.addr_bus, Svbk),
            read_bus_reg!(IORegs::Hdma1, self.addr_bus, Hdma1),
            read_bus_reg!(IORegs::Hdma2, self.addr_bus, Hdma2),
            read_bus_reg!(IORegs::Hdma3, self.addr_bus, Hdma3),
            read_bus_reg!(IORegs::Hdma4, self.addr_bus, Hdma4),
            read_bus_reg!(IORegs::Hdma5, self.addr_bus, Hdma5),
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
            read_bus_reg!(AudioRegs::AudMap, self.addr_bus, Nr50),
            read_bus_reg!(AudioRegs::AudChanCtl, self.addr_bus, Nr51),
            read_bus_reg!(AudioRegs::AudWave, self.addr_bus, Nr52),
        ]
    }
}
