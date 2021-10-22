use gb_bus::{
    generic::{CharDevice, SimpleRW},
    AddressBus, Area, FileOperation, IORegBus,
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
    cpu: Cpu,
    clock: Clock<AddressBus>,
    ppu: PPU,
    io_bus: IORegBus,
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
        let ppu_mem = ppu.memory();
        let ppu_reg = ppu.registers();
        let cpu = Cpu::default();

        let io_bus = IORegBus {
            controller: Box::new(CharDevice::default()),
            // communication: Box<dyn FileOperation<IORegArea>>,
            div_timer: Box::new(SimpleRW::<3>::default()),
            // sound: Box<dyn FileOperation<IORegArea>>,
            // waveform_ram: Box<dyn FileOperation<IORegArea>>,
            lcd: Box::new(ppu_reg),
            vram_bank: Box::new(ppu_reg),
            // boot_rom: Box<dyn FileOperation<IORegArea>>,
            // vram_dma: Box<dyn FileOperation<IORegArea>>,
            bg_obj_palettes: Box::new(ppu_reg),
            // wram_bank: Box<dyn FileOperation<IORegArea>>,
        };

        // TODO: store timer
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
        })
    }
}
