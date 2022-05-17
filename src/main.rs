mod config;
mod constant;
mod context;
mod custom_event;
mod game;
mod image;
mod logger;
mod path;
#[cfg(feature = "fps")]
mod time_frame;
mod ui;
mod windows;

use clap::StructOpt;
use config::Config;

use crate::constant::MENU_BAR_SIZE;
use context::Context;
use custom_event::CustomEvent;
use gb_lcd::{GBPixels, PseudoWindow};
use gb_ppu::{GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH};
use logger::init_logger;
use pixels::Error;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};

const WIDTH: u32 = GB_SCREEN_WIDTH as u32;
const HEIGHT: u32 = GB_SCREEN_HEIGHT as u32;
const MENU_BAR: u32 = MENU_BAR_SIZE as u32;
const MAIN_WINDOW_SCALE_FACTOR: u32 = 4;

fn main() -> Result<(), Error> {
    let config: Config = Config::parse();
    init_logger(config.log_level);

    let (event_loop, main_window) = init::<WIDTH, HEIGHT, MENU_BAR, MAIN_WINDOW_SCALE_FACTOR>()?;
    let event_loop_proxy = event_loop.create_proxy();
    let mut context = Context::new(main_window, event_loop_proxy);
    context.load_config(config);
    let mut render_time = std::time::Instant::now();

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::NewEvents(_) => {
            render_time = std::time::Instant::now();
        }
        Event::WindowEvent { window_id, event } => {
            context.process_window_event(window_id, event);
        }
        Event::UserEvent(event) => {
            handle_custom_event(&mut context, event, event_loop, control_flow)
        }
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
            drop(context.game.take());
            log::info!("bye bye");
        }
        Event::MainEventsCleared => {
            if let Some(ref mut game) = context.game {
                while game.cycle() {
                    if let Some(status) = context
                        .debugger_ctx
                        .as_mut()
                        .and_then(|ctx| ctx.debugger.updated_flow_status(game))
                    {
                        game.update_scheduled_stop(status);
                    }
                }
                if let Some(ref mut ctx) = context.debugger_ctx {
                    ctx.window.request_redraw();
                    if let Some(status) = ctx.debugger.updated_flow_status(game) {
                        game.update_scheduled_stop(status);
                    }
                }
                if let Some(ref mut ctx) = context.tilesheet_ctx {
                    ctx.window.request_redraw();
                }
                if let Some(ref mut ctx) = context.tilemap_ctx {
                    ctx.window.request_redraw();
                }
                if let Some(ref mut ctx) = context.spritesheet_ctx {
                    ctx.window.request_redraw();
                }
            }
            context.main_window.window.request_redraw();
            if let Some(ref keybindings) = context.keybindings_ctx {
                keybindings.window.request_redraw();
            }
        }
        Event::RedrawEventsCleared => {
            let elapsed = render_time.elapsed();
            if control_flow != &ControlFlow::Exit {
                if elapsed < constant::TARGET_FRAME_DURATION {
                    let time_to_sleep = constant::TARGET_FRAME_DURATION - elapsed;
                    *control_flow =
                        ControlFlow::WaitUntil(std::time::Instant::now() + time_to_sleep);
                    std::thread::sleep(time_to_sleep);
                } else {
                    *control_flow = ControlFlow::Poll;
                }
            }
        }
        Event::Resumed | Event::Suspended | Event::DeviceEvent { .. } => {
            // log::debug!("ignore event {event:?}");
        }
    })
}

fn init<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32, const SCALE_FACTOR: u32>(
) -> Result<
    (
        EventLoop<CustomEvent>,
        GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>,
    ),
    Error,
> {
    let event_loop = EventLoop::with_user_event();
    let main_window = {
        let size = LogicalSize::new(
            (WIDTH * SCALE_FACTOR) as f64,
            (HEIGHT * SCALE_FACTOR) as f64 + MENU_BAR_SIZE as f64,
        );
        WindowBuilder::new()
            .with_title(constant::APP_NAME)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("cannot build main window")
    };

    let main_window = GBPixels::new(main_window)?;

    Ok((event_loop, main_window))
}

fn handle_custom_event(
    context: &mut Context,
    event: CustomEvent,
    event_loop: &EventLoopWindowTarget<CustomEvent>,
    control_flow: &mut ControlFlow,
) {
    match event {
        CustomEvent::Quit => *control_flow = ControlFlow::Exit,
        CustomEvent::LoadFile(file) => context.load(file, context.debugger_ctx.is_some()),
        CustomEvent::OpenWindow(window_type) => context
            .open_window(window_type, event_loop)
            .unwrap_or_else(|ref err| log::error!("Failed to open new window: {:?}", err)),
        CustomEvent::CloseWindow(window_type) => context.close_window(window_type),
        CustomEvent::ChangedMode(mode) => context.reset_game(mode),
        CustomEvent::ResetGame => context.reset_game(None),
    }
}
