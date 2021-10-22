use gb_bus::{Area, FileOperation};
use gb_joypad::Joypad;
use gb_lcd::{render::RenderImage, window::GBWindow};
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
}

impl GameContext {
    pub fn new(romname: String) -> Result<GameContext, anyhow::Error> {
        use std::{fs::File, io::Seek};

        let mut file = File::open(romname)?;
        let header = Header::from_file(&mut file)?;

        log::debug!("header: {:?}", header);

        file.rewind()?;
        let mbc = generate_rom_controller(file, header)?;

        todo!("store cpu");
        todo!("store clock");
        todo!("store timer");
        todo!("store address bus");
        Ok(Self {
            romname,
            header,
            auto_save: header.cartridge_type.auto_save_type(),
            mbc,
        })
    }
}
