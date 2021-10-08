mod context;
mod event;
mod settings;
mod ui;

#[cfg(feature = "debug_render")]
use sdl2::keyboard::Scancode;

use context::{Context, Windows};
use gb_dbg::*;
use gb_lcd::{render, window::GBWindow};
use gb_ppu::PPU;

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

fn main() {
    init_logger();

    let (sdl_context, video_subsystem, mut event_pump) =
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
    let mut ppu = PPU::new();

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
    let mut context = Context {
        sdl: sdl_context,
        video: video_subsystem,
        display: display,
        joypad,
        windows,
    };
    'running: loop {
        context
            .windows
            .main
            .start_frame()
            .expect("Fail at the start for the main window");

        // render is updated just before drawing for now but we might want to change that later
        ppu.compute();
        context.display.update_render(ppu.pixels());
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

        if std::ops::ControlFlow::Break(())
            == event::process_event(
                &mut context.windows.main,
                &mut context.windows.debug,
                &context.video,
                &mut context.display,
                &mut context.windows.input,
                &mut event_pump,
                &mut context.joypad,
            )
        {
            break 'running;
        }
        // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    log::info!("quitting");
}

#[cfg(debug_assertions)]
fn init_logger() {
    use log::LevelFilter;
    use simplelog::Config;

    setup_terminal_logger(LevelFilter::Debug, Config::default());
}

#[cfg(not(debug_assertions))]
fn init_logger() {
    use log::LevelFilter;
    use simplelog::{Config, WriteLogger};
    use std::fs::File;

    const LEVEL_FILTER: LevelFilter = LevelFilter::Warn;
    const LOG_FILE: &'static str = "/tmp/gbmu.log";
    let config: Config = Config::default();
    let file_res = File::create(LOG_FILE);

    if let Ok(file) = file_res {
        let write_logger_res = WriteLogger::init(LEVEL_FILTER, config.clone(), file);
        if write_logger_res.is_ok() {
            return;
        } else {
            setup_terminal_logger(LEVEL_FILTER, config);
            log::warn!(
                "cannot setup write logger (because: {})",
                write_logger_res.unwrap_err()
            );
        }
    } else {
        setup_terminal_logger(LEVEL_FILTER, config);
        log::warn!(
            "cannot setup logging to file {} (because: {})",
            LOG_FILE,
            file_res.unwrap_err()
        );
    }
    log::warn!("fallback to terminal logger");
}

fn setup_terminal_logger(level: log::LevelFilter, config: simplelog::Config) {
    use simplelog::{ColorChoice, TermLogger, TerminalMode};

    TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto)
        .expect("cannot setup terminal logger")
}
