use crate::settings;
use gb_joypad::Joypad;
use gb_lcd::{render::RenderImage, window::GBWindow};
use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub fn process_event<const WIDTH: usize, const HEIGHT: usize>(
    sdl_context: &sdl2::Sdl,
    window: &mut GBWindow,
    debug_window: &mut Option<GBWindow>,
    video: &sdl2::VideoSubsystem,
    display: &mut RenderImage<WIDTH, HEIGHT>,
    input: &mut Option<GBWindow>,
    events: &mut EventPump,
    joypad: &mut Joypad,
) -> std::ops::ControlFlow<()> {
    for event in events.poll_iter() {
        joypad.send_event(&event);
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
                    if window.sdl_window().id() == window_id {
                        window
                            .resize((width as u32, height as u32), video)
                            .expect("Fail to resize GB window");
                        display.resize(window.sdl_window().size());
                    } else if let Some(ref mut dbg_wind) = debug_window {
                        if dbg_wind.sdl_window().id() == window_id {
                            dbg_wind
                                .resize((width as u32, height as u32), video)
                                .expect("Fail to resize debug window");
                        }
                    } else if let Some(ref mut input_wind) = input {
                        if input_wind.sdl_window().id() == window_id {
                            input_wind
                                .resize((width as u32, height as u32), video)
                                .expect("Fail to resize input window");
                        }
                    }
                }
                sdl2::event::WindowEvent::Close => {
                    if window.sdl_window().id() == window_id {
                        return std::ops::ControlFlow::Break(());
                    } else if let Some(ref mut dbg_wind) = debug_window {
                        if dbg_wind.sdl_window().id() == window_id {
                            *debug_window = None;
                        }
                    } else if let Some(ref mut input_wind) = input {
                        if input_wind.sdl_window().id() == window_id {
                            settings::save(joypad.get_config());
                            *input = None;
                        }
                    }
                }
                _ => {}
            },
            _ => {
                if !window.send_event(&event, sdl_context) {
                    if let Some(ref mut dbg_wind) = debug_window {
                        dbg_wind.send_event(&event, sdl_context);
                    }
                    if let Some(ref mut input_wind) = input {
                        input_wind.send_event(&event, sdl_context);
                    }
                }
            }
        }
    }
    std::ops::ControlFlow::Continue(())
}
