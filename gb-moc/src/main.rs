use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

const BAR_SIZE: f32 = 30.0;
const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const SCREEN_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

mod triangle;
mod window;

fn main() {
    println!("ratio: {}", SCREEN_RATIO);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);

    // OpenGL 3.3 is the minimum that we will support.
    gl_attr.set_context_version(3, 3);

    let mut gb_window = window::GBWindow::new(
        "GBMU",
        (SCREEN_WIDTH, SCREEN_HEIGHT + BAR_SIZE as u32),
        true,
        &video_subsystem,
    );
    let (width, height) = gb_window.sdl_window().size();

    gb_window
        .sdl_window_mut()
        .set_minimum_size(width, height)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let triangle = triangle::Triangle::new();

    let mut debug_window = None;

    'running: loop {
        gb_window.start_frame();

        // emulation render here
        triangle.draw();

        // set ui logic here
        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(BAR_SIZE);
                if ui.button("Load").clicked() {}
                if ui.button("Debug").clicked() {
                    if debug_window.is_none() {
                        debug_window = Some(window::GBWindow::new(
                            "GBMU Debug",
                            (800, 600),
                            false,
                            &video_subsystem,
                        ));
                    }
                }
            })
        });
        gb_window.end_frame();

        if let Some(ref mut dgb_wind) = debug_window {
            dgb_wind.start_frame();
            egui::containers::CentralPanel::default().show(dgb_wind.egui_ctx(), |ui| {
                ui.label("hello Debug");
            });
            dgb_wind.end_frame();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event,
                    window_id,
                    ..
                } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(width, height) => {
                        if gb_window.sdl_window().id() == window_id {
                            gb_window.resize((width as u32, height as u32), &video_subsystem);
                        } else if let Some(ref mut dbg_wind) = debug_window {
                            if dbg_wind.sdl_window().id() == window_id {
                                dbg_wind.resize((width as u32, height as u32), &video_subsystem);
                            }
                        }
                    }
                    sdl2::event::WindowEvent::Close => {
                        if gb_window.sdl_window().id() == window_id {
                            break 'running;
                        } else if let Some(ref mut dbg_wind) = debug_window {
                            if dbg_wind.sdl_window().id() == window_id {
                                debug_window = None;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {
                    if let Some(ref mut dbg_wind) = debug_window {
                        dbg_wind.send_event(event.clone());
                    }
                    gb_window.send_event(event);
                }
            }
        }
        std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
