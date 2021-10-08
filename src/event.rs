use crate::{context::Context, settings};
use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub fn process_event<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    events: &mut EventPump,
) -> std::ops::ControlFlow<()> {
    for event in events.poll_iter() {
        context.joypad.send_event(&event);
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
                if window.sdl_window().id() == window_id && scancode == Some(Scancode::Grave) {
                    debug = !debug;
                    log::debug!("toggle debug ({})", debug);
                    display.switch_draw_mode(debug);
                    window.set_debug(debug);
                }
            }
            Event::Window {
                win_event,
                window_id,
                ..
            } => match win_event {
                sdl2::event::WindowEvent::SizeChanged(width, height) => {
                    if context.windows.main.sdl_window().id() == window_id {
                        context
                            .windows
                            .main
                            .resize((width as u32, height as u32), &context.video)
                            .expect("Fail to resize GB window");
                        context
                            .display
                            .resize(context.windows.main.sdl_window().size());
                    } else if let Some(ref mut dbg_wind) = context.windows.debug {
                        if dbg_wind.sdl_window().id() == window_id {
                            dbg_wind
                                .resize((width as u32, height as u32), &context.video)
                                .expect("Fail to resize debug window");
                        }
                    } else if let Some(ref mut input_wind) = context.windows.input {
                        if input_wind.sdl_window().id() == window_id {
                            input_wind
                                .resize((width as u32, height as u32), &context.video)
                                .expect("Fail to resize input window");
                        }
                    }
                }
                sdl2::event::WindowEvent::Close => {
                    if context.windows.main.sdl_window().id() == window_id {
                        return std::ops::ControlFlow::Break(());
                    } else if let Some(ref mut dbg_wind) = context.windows.debug {
                        if dbg_wind.sdl_window().id() == window_id {
                            context.windows.debug = None;
                        }
                    } else if let Some(ref mut input_wind) = context.windows.input {
                        if input_wind.sdl_window().id() == window_id {
                            settings::save(context.joypad.get_config());
                            context.windows.input = None;
                        }
                    }
                }
                _ => {}
            },
            _ => {
                if !context.windows.main.send_event(&event, &context.sdl) {
                    if let Some(ref mut dbg_wind) = context.windows.debug {
                        dbg_wind.send_event(&event, &context.sdl);
                    }
                    if let Some(ref mut input_wind) = context.windows.input {
                        input_wind.send_event(&event, &context.sdl);
                    }
                }
            }
        }
    }
    std::ops::ControlFlow::Continue(())
}
