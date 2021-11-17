mod context;
mod event;
mod logger;
mod settings;
mod ui;

use clap::{AppSettings, Clap};
#[cfg(feature = "debug_render")]
use sdl2::keyboard::Scancode;

use context::{Context, Game, Windows};
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_lcd::{render, window::GBWindow};
use logger::init_logger;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clap, Debug)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short = 'l', long = "log", about = "change log level", possible_values = &["trace", "debug", "info", "warn", "error", "off"])]
    #[cfg_attr(not(debug_assertions), clap(default_value = "warn"))]
    #[cfg_attr(debug_assertions, clap(default_value = "debug"))]
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

    let (mut context, mut game, mut debugger, mut event_pump) = init_gbmu(&opts);

    'running: loop {
        context
            .windows
            .main
            .start_frame()
            .expect("Fail at the start for the main window");
        if let Some(ref mut game) = game {
            while game.cycle() {
                //            log::trace!("cycling the game");
                if let Some(flow) = debugger.updated_flow_status(game) {
                    game.update_scheduled_stop(flow);
                }
            }
            //       log::trace!("frame ready");
            game.draw(&mut context);
        }
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

        if let Some(ref mut game) = game {
            if let Some(ref mut dgb_wind) = context.windows.debug {
                dgb_wind
                    .start_frame()
                    .expect("Fail at the start for the debug window");
                debugger.draw(dgb_wind.egui_ctx(), game);
                dgb_wind
                    .end_frame()
                    .expect("Fail at the end for the debug window");
                if let Some(flow) = debugger.updated_flow_status(game) {
                    game.update_scheduled_stop(flow);
                }
            }
        }

        if let Some(ref mut input_wind) = context.windows.input {
            input_wind
                .start_frame()
                .expect("Fail at the start for the input window");
            context.joypad.borrow_mut().settings(input_wind.egui_ctx());
            input_wind
                .end_frame()
                .expect("Fail at the end for the input window");
        }

        if std::ops::ControlFlow::Break(()) == event::process_event(&mut context, &mut event_pump) {
            break 'running;
        }
    }
    log::info!("quitting");
}

fn init_gbmu<const WIDTH: usize, const HEIGHT: usize>(
    opts: &Opts,
) -> (
    Context<WIDTH, HEIGHT>,
    Option<Game>,
    Debugger<Game>,
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

    let joypad = Rc::new(RefCell::new(match settings::load() {
        Some(conf) => gb_joypad::Joypad::from_config(gb_window.sdl_window().id(), conf),
        None => {
            log::warn!("No settings found, using default input configuration");
            let tmp = gb_joypad::Joypad::new(gb_window.sdl_window().id());
            settings::save(tmp.get_config());
            tmp
        }
    }));

    let game_context: Option<Game> = opts.rom.as_ref().and_then(|romname| {
        Game::new(romname.clone(), joypad.clone(), opts.debug).map_or_else(
            |e| {
                log::error!("while creating game context for {}: {:?}", romname, e);
                None
            },
            Option::Some,
        )
    });

    let dbg = DebuggerBuilder::new().build();

    let windows = Windows {
        main: gb_window,
        debug: if opts.debug && game_context.is_some() {
            Some(ui::new_debug_window(&video_subsystem))
        } else {
            None
        },
        input: None,
    };

    (
        Context {
            sdl: sdl_context,
            video: video_subsystem,
            display,
            joypad,
            windows,
        },
        game_context,
        dbg,
        event_pump,
    )
}
