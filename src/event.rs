use crate::custom_event::CustomEvent;
use crate::{context::Context, settings};
#[cfg(feature = "debug_render")]
use sdl2::keyboard::Scancode;
use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub fn process_event<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    events: &mut EventPump,
) -> std::ops::ControlFlow<()> {
    for event in events.poll_iter() {
        context.joypad.borrow_mut().send_event(&event);
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                // here for debug, maybe remove later ?
                keycode: Some(Keycode::Escape),
                ..
            } => return std::ops::ControlFlow::Break(()),
            #[cfg(feature = "debug_render")]
            sdl2::event::Event::KeyDown {
                window_id,
                scancode,
                ..
            } => {
                if context.windows.main.sdl_window().id() == window_id
                    && scancode == Some(Scancode::Grave)
                {
                    context.debug_render = !context.debug_render;
                    log::debug!("toggle debug ({})", context.debug_render);
                    context.display.switch_draw_mode(context.debug_render);
                    context.windows.main.set_debug(context.debug_render);
                }
            }
            Event::Window {
                win_event,
                window_id,
                ..
            } => {
                let res = process_window_event(win_event, window_id, context);
                if res == std::ops::ControlFlow::Break(()) {
                    return res;
                }
            }
            Event::DropFile {
                filename,
                window_id,
                ..
            } if window_id == context.windows.main.sdl_window().id() => context
                .custom_events
                .push(CustomEvent::FileDropped(filename)),
            _ => {
                if !context.windows.main.send_event(&event, &context.sdl) {
                    if let Some(ref mut dbg_wind) = context.windows.debug {
                        dbg_wind.send_event(&event, &context.sdl);
                    }
                    if let Some(ref mut input_wind) = context.windows.input {
                        input_wind.send_event(&event, &context.sdl);
                    }
                    #[cfg(feature = "debug_render")]
                    if let Some((ref mut tilemap_wind, _, _)) = context.windows.tilemap {
                        tilemap_wind.send_event(&event, &context.sdl);
                    }
                    #[cfg(feature = "debug_render")]
                    if let Some((ref mut tilesheet_wind, _)) = context.windows.tilesheet {
                        tilesheet_wind.send_event(&event, &context.sdl);
                    }
                    #[cfg(feature = "debug_render")]
                    if let Some(ref mut cfg) = context.windows.oam {
                        cfg.window.send_event(&event, &context.sdl);
                    }
                }
            }
        }
    }
    std::ops::ControlFlow::Continue(())
}

fn process_window_event<const WIDTH: usize, const HEIGHT: usize>(
    event: sdl2::event::WindowEvent,
    window_id: u32,
    context: &mut Context<WIDTH, HEIGHT>,
) -> std::ops::ControlFlow<()> {
    match event {
        sdl2::event::WindowEvent::SizeChanged(_width, _height) => {
            if context.windows.main.sdl_window().id() == window_id {
                context
                    .windows
                    .main
                    .resize(&context.video)
                    .expect("Fail to resize GB window");
                context
                    .display
                    .resize(context.windows.main.sdl_window().size());
            }
            if let Some(ref mut dbg_wind) = context.windows.debug {
                if dbg_wind.sdl_window().id() == window_id {
                    dbg_wind
                        .resize(&context.video)
                        .expect("Fail to resize debug window");
                }
            }
            if let Some(ref mut input_wind) = context.windows.input {
                if input_wind.sdl_window().id() == window_id {
                    input_wind
                        .resize(&context.video)
                        .expect("Fail to resize input window");
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some((ref mut tilemap_wind, ref mut display, _)) = context.windows.tilemap {
                if tilemap_wind.sdl_window().id() == window_id {
                    tilemap_wind
                        .resize(&context.video)
                        .expect("Fail to resize tilemap window");
                    display.resize(tilemap_wind.sdl_window().size());
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some((ref mut tilesheet_wind, ref mut display)) = context.windows.tilesheet {
                if tilesheet_wind.sdl_window().id() == window_id {
                    tilesheet_wind
                        .resize(&context.video)
                        .expect("Fail to resize tilesheet window");
                    display.resize(tilesheet_wind.sdl_window().size());
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some(ref mut cfg) = context.windows.oam {
                if cfg.window.sdl_window().id() == window_id {
                    cfg.window
                        .resize(&context.video)
                        .expect("Fail to resize oam window");
                    cfg.viewport.resize(cfg.window.sdl_window().size());
                    cfg.list.resize(cfg.window.sdl_window().size());
                }
            }
        }
        sdl2::event::WindowEvent::Close => {
            if context.windows.main.sdl_window().id() == window_id {
                return std::ops::ControlFlow::Break(());
            }
            if let Some(ref mut dbg_wind) = context.windows.debug {
                if dbg_wind.sdl_window().id() == window_id {
                    context.windows.debug = None;
                }
            }
            if let Some(ref mut input_wind) = context.windows.input {
                if input_wind.sdl_window().id() == window_id {
                    settings::save(context.joypad.borrow().get_config());
                    context.windows.input = None;
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some((ref mut tilemap_wind, _, _)) = context.windows.tilemap {
                if tilemap_wind.sdl_window().id() == window_id {
                    context.windows.tilemap = None;
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some((ref mut tilesheet_wind, _)) = context.windows.tilesheet {
                if tilesheet_wind.sdl_window().id() == window_id {
                    context.windows.tilesheet = None;
                }
            }
            #[cfg(feature = "debug_render")]
            if let Some(ref mut cfg) = context.windows.oam {
                if cfg.window.sdl_window().id() == window_id {
                    context.windows.oam = None;
                }
            }
        }
        _ => {}
    }
    std::ops::ControlFlow::Continue(())
}
