mod config;
mod constant;
mod custom_event;
mod logger;
#[cfg(any(feature = "time_frame", feature = "debug_fps"))]
mod time_frame;

use clap::StructOpt;
use config::Config;
use constant::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH, TARGET_FPS_X10};
use custom_event::CustomEvent;
use logger::init_logger;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Duration;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    #[cfg(feature = "cgb")]
    let mut opts: Config = Config::parse();
    #[cfg(not(feature = "cgb"))]
    let opts: Config = Config::parse();
    #[cfg(feature = "time_frame")]
    let mut time_frame_stat = time_frame::TimeStat::default();
    #[cfg(any(feature = "time_frame", feature = "debug_fps"))]
    let mut render_time_frame = time_frame::TimeStat::default();
    let frame_duration_target = Duration::from_nanos(10_000_000_000 / TARGET_FPS_X10);
    init_logger(opts.log_level);
    let mut input = WinitInputHelper::new();

    let (event_loop, main_window, pixels) = init::<GB_SCREEN_WIDTH, GB_SCREEN_HEIGHT>(&opts)?;

    event_loop.run(move |event, event_loop, control_flow| handle_input_event(&mut input, &event))
}

fn init<const WIDTH: u32, const HEIGHT: u32>(
    config: &Config,
) -> Result<(EventLoop<CustomEvent>, Window, Pixels), Error> {
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

    let (pixels) = {
        let window_size = main_window.inner_size();
        let scale_factor = main_window.scale_factor();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &main_window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

        (pixels)
    };

    Ok((event_loop, main_window, pixels))
}

fn handle_input_event(input: &mut WinitInputHelper, event: &Event<CustomEvent>) {
    if input.update(event) {}
}
