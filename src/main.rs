mod config;
mod constant;
mod context;
mod custom_event;
mod image;
mod logger;
#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
mod time_frame;
mod ui;
mod windows;

use clap::StructOpt;
use config::Config;
use context::Context;
use custom_event::CustomEvent;
use gb_lcd::GBPixels;
use gb_ppu::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH};
use logger::init_logger;
use pixels::Error;
use windows::Windows;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = GB_SCREEN_WIDTH as u32;
const HEIGHT: u32 = GB_SCREEN_HEIGHT as u32;

fn main() -> Result<(), Error> {
    #[cfg(feature = "cgb")]
    let mut config: Config = Config::parse();
    #[cfg(not(feature = "cgb"))]
    let config: Config = Config::parse();
    // #[cfg(feature = "time_frame")]
    // let mut time_frame_stat = time_frame::TimeStat::default();
    // #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    // let mut render_time_frame = time_frame::TimeStat::default();
    // let frame_duration_target = Duration::from_nanos(10_000_000_000 / TARGET_FPS_X10);
    init_logger(config.log_level);

    let (event_loop, main_window) = init::<WIDTH, HEIGHT>(&config)?;
    let event_loop_proxy = event_loop.create_proxy();
    let windows = Windows::new(main_window);
    let mut context = Context::new(windows, config, event_loop_proxy);

    event_loop.run(move |event, _event_loop, control_flow| match event {
        Event::WindowEvent { window_id, event } => {
            context.process_window_event(window_id, event);
        }
        Event::UserEvent(event) => handle_custom_event(&mut context, event, control_flow),
        Event::RedrawRequested(window_id) => {
            if context
                .redraw(window_id)
                .map_err(|e| log::error!("fail to redraw window {window_id:?}: {e}"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
            }

            {
                let frame = context.windows.main.pixels.get_frame();
                const BOX_SIZE: usize = 32;
                const BOX_X: usize = (GB_SCREEN_WIDTH / 2) as usize - BOX_SIZE / 2;
                const BOX_Y: usize = (GB_SCREEN_HEIGHT / 2) as usize - BOX_SIZE / 2;

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = i % GB_SCREEN_WIDTH as usize;
                    let y = i / GB_SCREEN_HEIGHT as usize;

                    let inside_the_box = (x >= BOX_X && x < BOX_X + BOX_SIZE)
                        && (y >= BOX_Y && y < BOX_Y + BOX_SIZE);

                    let rgba = if inside_the_box {
                        [0x5e, 0x48, 0xe8, 0xff]
                    } else {
                        [0x48, 0xb2, 0xe8, 0xff]
                    };

                    pixel.copy_from_slice(&rgba);
                }
            }
        }
        Event::LoopDestroyed => {
            log::info!("bye bye");
        }
        Event::MainEventsCleared => {
            context.windows.main.window.request_redraw();
        }
        Event::NewEvents(_)
        | Event::Resumed
        | Event::Suspended
        | Event::RedrawEventsCleared
        | Event::DeviceEvent { .. } => {
            // log::debug!("ignore event {event:?}");
        }
    })
}

fn init<const WIDTH: u32, const HEIGHT: u32>(
    _config: &Config,
) -> Result<(EventLoop<CustomEvent>, GBPixels), Error> {
    let event_loop = EventLoop::with_user_event();
    let main_window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title(constant::APP_NAME)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("cannot build main window")
    };

    let main_window = GBPixels::new::<WIDTH, HEIGHT>(main_window)?;

    Ok((event_loop, main_window))
}

fn handle_custom_event(context: &mut Context, event: CustomEvent, control_flow: &mut ControlFlow) {
    match event {
        CustomEvent::Quit => *control_flow = ControlFlow::Exit,
        CustomEvent::LoadFile(file) => context.load(file),
        _ => todo!("unhandled custom event {event:?}"),
    }
}
