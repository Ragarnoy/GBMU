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
        let wram = WorkingRam::new(false);

        let io_bus = IORegBus {
            controller: Box::new(CharDevice::default()),
            communication: Box::new(SimpleRW::<2>::default()),
            div_timer: Box::new(SimpleRW::<3>::default()),
            sound: Box::new(SimpleRW::<0x16>::default()),
            waveform_ram: Box::new(SimpleRW::<0xF>::default()),
            lcd: Box::new(ppu_reg),
            vram_bank: Box::new(ppu_reg),
            boot_rom: Box::new(CharDevice::default()),
            vram_dma: Box::new(SimpleRW::<4>::default()),
            bg_obj_palettes: Box::new(ppu_reg),
            wram_bank: Box::new(wram),
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
