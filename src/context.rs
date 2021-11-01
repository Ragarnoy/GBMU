use anyhow::Result;
use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Bus, IORegBus, Lock, WorkingRam,
};
use gb_clock::Clock;
use gb_cpu::cpu::Cpu;
use gb_dbg::dbg_interfaces::{
    CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs, RegisterDebugOperations,
    RegisterMap, RegisterValue,
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
}

impl Game {
    pub fn new(romname: String) -> Result<Game, anyhow::Error> {
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
            vram_dma: Rc::new(RefCell::new(SimpleRW::<4>::default())), // TODO: link the part that handle the DMA
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
        })
    }

    pub fn cycle(&mut self) -> bool {
        self.clock.cycle(
            &mut self.addr_bus,
            self.cpu.borrow_mut(),
            &mut self.ppu,
            self.timer.borrow_mut(),
        )
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

macro_rules! read_bus_reg {
    ($type:expr, $bus:expr, $addr:expr) => {
        RegisterMap(
            $type,
            $bus.read($addr, Some(Lock::Debugger))
                .unwrap_or(0xffu8)
                .into(),
        )
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

    fn ppu_get(&self, _key: PpuRegs) -> Result<RegisterValue> {
        Ok(RegisterValue::U8(0xff))
    }

    fn io_get(&self, _key: IORegs) -> Result<RegisterValue> {
        Ok(RegisterValue::U8(0xff))
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
            read_bus_reg!(IORegs::Joy, self.addr_bus, 0xff00),
            // serial regs
            read_bus_reg!(IORegs::SerialByte, self.addr_bus, 0xff01),
            read_bus_reg!(IORegs::SerialControl, self.addr_bus, 0xff02),
            // Timer regs
            read_bus_reg!(IORegs::Div, self.addr_bus, 0xff04),
            read_bus_reg!(IORegs::Tima, self.addr_bus, 0xff05),
            read_bus_reg!(IORegs::Tma, self.addr_bus, 0xff06),
            read_bus_reg!(IORegs::Tac, self.addr_bus, 0xff07),
            // cpu int regs
            read_bus_reg!(IORegs::If, self.addr_bus, 0xff0f),
            read_bus_reg!(IORegs::Ie, self.addr_bus, 0xffff),
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
