mod config;
mod constant;
mod context;
mod custom_event;
mod logger;
#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
mod time_frame;
mod windows;

use clap::StructOpt;
use config::Config;
use constant::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH, TARGET_FPS_X10};
use context::Context;
use custom_event::CustomEvent;
use egui::panel;
use gb_lcd::{EventProcessing, GBPixels, GBWindow, PseudoWindow};
use logger::init_logger;
use pixels::Error;
use std::time::Duration;
use windows::Windows;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    #[cfg(feature = "cgb")]
    let mut opts: Config = Config::parse();
    #[cfg(not(feature = "cgb"))]
    let opts: Config = Config::parse();
    // #[cfg(feature = "time_frame")]
    // let mut time_frame_stat = time_frame::TimeStat::default();
    // #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    // let mut render_time_frame = time_frame::TimeStat::default();
    // let frame_duration_target = Duration::from_nanos(10_000_000_000 / TARGET_FPS_X10);
    init_logger(opts.log_level);

    let (event_loop, main_window) = init::<GB_SCREEN_WIDTH, GB_SCREEN_HEIGHT>(&opts)?;
    let event_loop_proxy = event_loop.create_proxy();
    let windows = Windows::new(main_window);
    let mut context = Context::new(windows);

    event_loop.run(move |event, _event_loop, control_flow| match event {
        Event::WindowEvent { window_id, event } => {
            context.process_window_event(window_id, event, &event_loop_proxy);
        }
        Event::UserEvent(event) => handle_custom_event(event, control_flow),
        Event::RedrawRequested(window_id) => {
            if context
                .redraw(window_id)
                .map_err(|e| log::error!("fail to redraw window {window_id:?}: {e}"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
            }
        }
        Event::LoopDestroyed => {
            log::info!("bye bye");
        }
        Event::NewEvents(_)
        | Event::MainEventsCleared
        | Event::Resumed
        | Event::Suspended
        | Event::RedrawEventsCleared
        | Event::DeviceEvent {
            device_id: _,
            event: _,
        } => log::debug!("ignore event {event:?}"),
    })
}

fn init<const WIDTH: u32, const HEIGHT: u32>(
    config: &Config,
) -> Result<(EventLoop<CustomEvent>, GBPixels), Error> {
    let event_loop = EventLoop::with_user_event();
    let main_window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = WindowBuilder::new()
            .with_title(constant::APP_NAME)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("cannot build main window");
        GBWindow::new(window)
    };

    let main_window = GBPixels::from_window::<WIDTH, HEIGHT>(main_window)?;

    Ok((event_loop, main_window))
}

fn handle_custom_event(event: CustomEvent, control_flow: &mut ControlFlow) {
    match event {
        CustomEvent::Quit => *control_flow = ControlFlow::Exit,
        _ => todo!("unhandle custom event {event:?}"),
    }
}
