use gb_bus::{Area, FileOperation};
use gb_joypad::Joypad;
use gb_lcd::{render::RenderImage, window::GBWindow};
use gb_roms::Header;

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
    pub romname: String,
}

impl GameContext {
    pub fn new(romname: String) -> Result<GameContext, std::io::Error> {
        use std::fs::File;

        let file = File::open(romname)?;
        let header = Header::from_file(file);

        log::debug!("header: {:?}", header);
        todo!("store header");
        todo!("indicate if the ext ram need to be saved");
        todo!("store mbc controller");
        todo!("store cpu");
        todo!("store clock");
        todo!("store timer");
        todo!("store address bus");
        Ok(Self { romname })
    }
}
