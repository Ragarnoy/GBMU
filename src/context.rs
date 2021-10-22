use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Area, FileOperation, IORegBus, WorkingRam,
};
use gb_clock::Clock;
use gb_cpu::cpu::Cpu;
use gb_joypad::Joypad;
use gb_lcd::{render::RenderImage, window::GBWindow};
use gb_ppu::PPU;
use gb_roms::{
    controllers::{generate_rom_controller, MbcController},
    header::AutoSave,
    Header,
};
use gb_timer::Timer;
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

pub struct GameContext {
    romname: String,
    header: Header,
    auto_save: Option<AutoSave>,
    mbc: MbcController,
    cpu: Rc<RefCell<Cpu>>,
    clock: Clock<AddressBus>,
    ppu: Rc<RefCell<PPU>>,
    io_bus: Rc<RefCell<IORegBus>>,
    timer: Rc<RefCell<Timer>>,
    addr_bus: AddressBus,
}

impl GameContext {
    pub fn new(romname: String) -> Result<GameContext, anyhow::Error> {
        use std::{fs::File, io::Seek};

        let mut file = File::open(romname)?;
        let header = Header::from_file(&mut file)?;

        log::debug!("header: {:?}", header);

        file.rewind()?;
        let mbc = generate_rom_controller(file, header)?;

        let ppu = PPU::new();
        let ppu_mem = Rc::new(RefCell::new(ppu.memory()));
        let ppu_reg = Rc::new(RefCell::new(ppu.registers()));
        let ppu = Rc::new(RefCell::new(ppu));
        let cpu = Rc::new(RefCell::new(Cpu::default()));
        let wram = Rc::new(RefCell::new(WorkingRam::new(false)));
        let timer = Rc::new(RefCell::new(Timer::default()));

        let io_bus = Rc::new(RefCell::new(IORegBus {
            controller: Rc::new(RefCell::new(CharDevice::default())),
            communication: Rc::new(RefCell::new(SimpleRW::<2>::default())), // We don't handle communication
            div_timer: timer,
            sound: Rc::new(RefCell::new(SimpleRW::<0x16>::default())), // We don't handle sound
            waveform_ram: Rc::new(RefCell::new(SimpleRW::<0xF>::default())), // We don't handle sound
            lcd: ppu_reg.clone(),
            vram_bank: ppu_reg.clone(),
            boot_rom: Rc::new(RefCell::new(CharDevice::default())), // TODO: togle between bios and ROM
            vram_dma: Rc::new(RefCell::new(SimpleRW::<4>::default())), // TODO: link the part that handle the DMA
            bg_obj_palettes: ppu_reg,
            wram_bank: wram.clone(),
        }));

        let bus = AddressBus {
            bios_enabling_reg: 0,
            bios: _TODO_,
            rom: mbc,
            vram: ppu_mem,
            ext_ram: mbc,
            ram: wram.clone(),
            eram: wram,
            oam: ppu_mem,
            io_reg: io_bus.clone(),
            hram: Rc::new(RefCell::new(SimpleRW::<0x80>::default())),
            ie_reg: Rc::new(RefCell::new(CharDevice::default()), // TODO: link the part that handle the IE
        };

        todo!("store address bus");
        Ok(Self {
            romname,
            header,
            auto_save: header.cartridge_type.auto_save_type(),
            mbc,
            cpu,
            clock: Clock::default(),
            ppu,
            io_bus,
            timer,
            bus
        })
    }
}
