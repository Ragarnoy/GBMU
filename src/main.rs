mod constant;
mod context;
mod custom_event;
mod event;
mod logger;
mod settings;
#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
mod time_frame;
mod ui;
mod windows;

use clap::Parser;

use context::{Context, Game};
use gb_dbg::debugger::options::DebuggerOptions;
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_lcd::{render, window::GBWindow};
use logger::init_logger;
use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
    time::{Duration, Instant},
};
use windows::Windows;

// const TARGET_FPS_X10: u64 = 597;    // the true value
const TARGET_FPS_X10: u64 = 600;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Opts {
    #[clap(short = 'l', long = "log", help = "change log level", possible_values = &["trace", "debug", "info", "warn", "error", "off"])]
    #[cfg_attr(not(debug_assertions), clap(default_value = "warn"))]
    #[cfg_attr(debug_assertions, clap(default_value = "debug"))]
    log_level: log::LevelFilter,

    #[clap(help = "rom file to be loaded by the gameboy")]
    rom: Option<String>,

    #[clap(
        long = "breakpoint",
        short = 'b',
        help = "create and enable breakpoints at the start of the rom\n\
        breakpoints must be specified in the following format:\n\
        ./gbmu -b \"PC == 0050\" -b \"AF == 0010\" ...",
        multiple_occurrences = true,
        multiple_values = false,
        requires = "rom"
    )]
    breakpoints: Vec<String>,
    #[clap(
        long = "debug",
        short = 'd',
        help = "enable debug mode at the start of the rom",
        requires = "rom"
    )]
    debug: bool,

    #[cfg(feature = "cgb")]
    #[clap(
        arg_enum,
        short = 'm',
        long,
        help = "force gameboy mode between color and mono"
    )]
    mode: Option<Mode>,
}

#[derive(Debug, clap::ArgEnum, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Color,
    Classic,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Color => write!(f, "color"),
            Mode::Classic => write!(f, "classic"),
        }
    }
}

fn main() {
    #[cfg(feature = "cgb")]
    let mut opts: Opts = Opts::parse();
    #[cfg(not(feature = "cgb"))]
    let opts: Opts = Opts::parse();
    #[cfg(feature = "time_frame")]
    let mut time_frame_stat = time_frame::TimeStat::default();
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    let mut render_time_frame = time_frame::TimeStat::default();
    let frame_duration_target = Duration::from_nanos(10_000_000_000 / TARGET_FPS_X10);
    init_logger(opts.log_level);

    let (mut context, mut game, mut debugger, mut event_pump) = init_gbmu(&opts);

    'running: loop {
        let now_render = Instant::now();
        context
            .windows
            .main
            .start_frame()
            .expect("Fail at the start for the main window");
        if let Some(ref mut game) = game {
            #[cfg(feature = "time_frame")]
            let now = Instant::now();
            while game.cycle() {
                if let Some(flow) = debugger.updated_flow_status(game) {
                    game.update_scheduled_stop(flow);
                }
            }
            #[cfg(feature = "time_frame")]
            {
                let time = now.elapsed();
                time_frame_stat.add_sample(time);
                log::info!(
                    "frame computed: current={}ms stat={}",
                    time.as_millis(),
                    time_frame_stat
                );
            }
            game.draw(&mut context);
        }
        ui::draw_egui(
            &mut context,
            #[cfg(feature = "cgb")]
            &mut opts,
            #[cfg(feature = "debug_fps")]
            render_time_frame.fps(),
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
                #[cfg(not(feature = "time_frame"))]
                debugger.draw(dgb_wind.egui_ctx(), game, None);
                #[cfg(feature = "time_frame")]
                debugger.draw(
                    dgb_wind.egui_ctx(),
                    game,
                    Some((&"time frame", &time_frame_stat)),
                );
                dgb_wind
                    .end_frame()
                    .expect("Fail at the end for the debug window");
                if let Some(flow) = debugger.updated_flow_status(game) {
                    game.update_scheduled_stop(flow);
                }
            }
        }
        if debugger.reset_triggered {
            game = opts.rom.as_ref().and_then(|romname| {
                load_game(
                    romname,
                    context.joypad.clone(),
                    true,
                    #[cfg(feature = "cgb")]
                    opts.mode,
                )
            });
            debugger.reset();
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

        #[cfg(feature = "debug_render")]
        ui::draw_ppu_debug_ui(&mut context, &mut game);

        for event in &context.custom_events {
            use custom_event::CustomEvent;

            match event {
                CustomEvent::FileDropped(filename) => {
                    game = load_game(
                        filename,
                        context.joypad.clone(),
                        opts.debug,
                        #[cfg(feature = "cgb")]
                        opts.mode,
                    )
                }
                CustomEvent::LoadFile(file) => {
                    game = load_game(
                        file,
                        context.joypad.clone(),
                        opts.debug,
                        #[cfg(feature = "cgb")]
                        opts.mode,
                    )
                }
                #[cfg(feature = "save_state")]
                CustomEvent::SaveState(file) => {
                    if let Some(ref game) = game {
                        game.save_state(&file);
                    } else {
                        log::warn!("no game context present, retry after loading a ROM");
                    }
                }
                #[cfg(feature = "save_state")]
                CustomEvent::LoadState(file) => {
                    if let Some(ref mut game) = game {
                        game.load_save_file(&file);
                    } else {
                        log::warn!("no game context to load the state into");
                    }
                }
                #[cfg(feature = "cgb")]
                CustomEvent::ChangedMode(wanted_mode) =>
                {
                    #[cfg(feature = "cgb")]
                    if let Some(ref game_ctx) = game {
                        if (wanted_mode == &Mode::Color) != game_ctx.cgb_mode {
                            game = load_game(
                                game_ctx.romname.clone(),
                                context.joypad.clone(),
                                opts.debug,
                                Some(*wanted_mode),
                            )
                        }
                    }
                }
            }
        }
        context.custom_events.clear();

        if std::ops::ControlFlow::Break(()) == event::process_event(&mut context, &mut event_pump) {
            break 'running;
        }

        let mut time = now_render.elapsed();
        while time < frame_duration_target {
            let diff = frame_duration_target - time;
            if diff.as_micros() > 500 {
                std::thread::sleep(diff - Duration::from_micros(100));
            }
            time = now_render.elapsed();
        }

        #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
        {
            render_time_frame.add_sample(time);
            #[cfg(feature = "time_frame")]
            log::info!(
                "frame rendered: current={}ms stat={}",
                time.as_millis(),
                render_time_frame
            );
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
        load_game(
            &romname,
            joypad.clone(),
            opts.debug,
            #[cfg(feature = "cgb")]
            opts.mode,
        )
    });

    let dbg_options = DebuggerOptions {
        breakpoints: opts.breakpoints.clone(),
        ..Default::default()
    };
    let dbg = DebuggerBuilder::new().with_options(dbg_options).build();

    let windows = Windows {
        main: gb_window,
        debug: if opts.debug && game_context.is_some() {
            Some(ui::new_debug_window(&video_subsystem))
        } else {
            None
        },
        input: None,
        #[cfg(feature = "debug_render")]
        tilemap: None,
        #[cfg(feature = "debug_render")]
        tilesheet: None,
        #[cfg(feature = "debug_render")]
        oam: None,
    };

    (
        Context {
            sdl: sdl_context,
            video: video_subsystem,
            display,
            joypad,
            windows,
            #[cfg(feature = "debug_render")]
            debug_render: false,
            custom_events: Vec::with_capacity(5),
        },
        game_context,
        dbg,
        event_pump,
    )
}

fn load_game<P: AsRef<std::path::Path>>(
    rompath: P,
    joypad: std::rc::Rc<std::cell::RefCell<gb_joypad::Joypad>>,
    stopped: bool,
    #[cfg(feature = "cgb")] forced_mode: Option<Mode>,
) -> Option<Game> {
    Game::new(
        &rompath,
        joypad,
        stopped,
        #[cfg(feature = "cgb")]
        forced_mode,
    )
    .map_or_else(
        |e| {
            log::error!(
                "while creating game context for {:?}: {:?}",
                rompath.as_ref(),
                e
            );
            None
        },
        Option::Some,
    )
}
