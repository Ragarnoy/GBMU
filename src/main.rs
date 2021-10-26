mod context;
mod event;
mod logger;
mod settings;
mod ui;

use clap::{AppSettings, Clap};
#[cfg(feature = "debug_render")]
use sdl2::keyboard::Scancode;
use std::{cell::RefCell, rc::Rc};

use context::{Context, Game, Windows};
use gb_dbg::*;
use gb_lcd::{render, window::GBWindow};
use gb_ppu::PPU;
use logger::init_logger;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}

impl dbg_interfaces::MemoryDebugOperations for Memory {
    fn read(&self, index: u16) -> u8 {
        *self.memory.get(index as usize).unwrap()
    }
}

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[cfg(not(debug_assertions))]
    #[clap(short = 'l', long = "log", default_value = "warn", about = "change log level", possible_values = &["trace", "debug", "info", "warn", "error", "off"])]
    log_level: log::LevelFilter,

    #[cfg(debug_assertions)]
    #[clap(short = 'l', long = "log", default_value = "debug", about = "change log level", possible_values = &["trace", "debug", "info", "warn", "error", "off"])]
    log_level: log::LevelFilter,

    #[clap(about = "rom file to be loaded by the gameboy")]
    rom: Option<String>,

    #[clap(
        long = "debug",
        about = "enable debug mode at the start of the rom",
        requires = "rom"
    )]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    init_logger(opts.log_level);

    let (mut context, mut game, mut ppu, mut event_pump) = init_gbmu(&opts);

    'running: loop {
        context
            .windows
            .main
            .start_frame()
            .expect("Fail at the start for the main window");

        // render is updated just before drawing for now but we might want to change that later
        ppu.borrow_mut().compute();
        context.display.update_render(ppu.borrow().pixels());
        // emulation render here
        context.display.draw();

        // set ui logic here
        ui::draw_egui(
            &mut context.windows.main,
            &mut context.windows.debug,
            &context.video,
            &mut context.windows.input,
        );
        context
            .windows
            .main
            .end_frame()
            .expect("Fail at the end for the main window");

        if let Some(ref mut dgb_wind) = context.windows.debug {
            dgb_wind
                .start_frame()
                .expect("Fail at the start for the debug window");
            // dbg_app.draw(dgb_wind.egui_ctx());
            // Regression for now
            dgb_wind
                .end_frame()
                .expect("Fail at the end for the debug window");
        }

        if let Some(ref mut input_wind) = context.windows.input {
            input_wind
                .start_frame()
                .expect("Fail at the start for the input window");
            context.joypad.settings(input_wind.egui_ctx());
            input_wind
                .end_frame()
                .expect("Fail at the end for the input window");
        }

        if std::ops::ControlFlow::Break(()) == event::process_event(&mut context, &mut event_pump) {
            break 'running;
        }
        // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    log::info!("quitting");
}

fn init_gbmu<const WIDTH: usize, const HEIGHT: usize>(
    opts: &Opts,
) -> (
    Context<WIDTH, HEIGHT>,
    Option<Game>,
    Rc<RefCell<PPU>>,
    sdl2::EventPump,
) {
    let (sdl_context, video_subsystem, event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let bar_pixels_size = GBWindow::dots_to_pixels(&video_subsystem, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut gb_window = GBWindow::new(
        "GBMU",
        (
            render::SCREEN_WIDTH as u32,
            render::SCREEN_HEIGHT as u32 + bar_pixels_size,
        ),
        true,
        &video_subsystem,
    )
    .expect("Error while building main window");
    let (width, height) = gb_window.sdl_window().size();

    gb_window
        .sdl_window_mut()
        .set_minimum_size(width, height)
        .expect("Failed to configure main window");

    let display = render::RenderImage::with_bar_size(bar_pixels_size as f32);

    let joypad = match settings::load() {
        Some(conf) => gb_joypad::Joypad::from_config(gb_window.sdl_window().id(), conf),
        None => {
            log::warn!("No settings found, using default input configuration");
            let tmp = gb_joypad::Joypad::new(gb_window.sdl_window().id());
            settings::save(tmp.get_config());
            tmp
        }
    };

    #[cfg(feature = "debug_render")]
    let mut debug = false;

    let windows = Windows {
        main: gb_window,
        debug: None,
        input: None,
    };

    let ppu = Rc::new(RefCell::new(PPU::new()));
    let game_context: Option<Game> = opts.rom.as_ref().and_then(|romname| {
        Game::new(romname.clone(), ppu.clone()).map_or_else(
            |e| {
                log::error!("while creating game context for {}: {:?}", romname, e);
                None
            },
            Option::Some,
        )
    });

    (
        Context {
            sdl: sdl_context,
            video: video_subsystem,
            display,
            joypad,
            windows,
        },
        game_context,
        ppu,
        event_pump,
    )
}
