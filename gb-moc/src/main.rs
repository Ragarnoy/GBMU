use nfd2::Response;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{gl_attr::GLAttr, GLProfile},
    EventPump, Sdl, VideoSubsystem,
};

pub const MENU_BAR_SIZE: f32 = 30.0;

mod error;
mod render;
mod window;

fn init_system() -> Result<(Sdl, VideoSubsystem, EventPump), error::Error> {
    let sdl_context = sdl2::init().map_err(|err| error::Error::MainSys(err))?;
    let video_subsystem = sdl_context
        .video()
        .map_err(|err| error::Error::MainSys(err))?;

    let event_pump = sdl_context
        .event_pump()
        .map_err(|err| error::Error::MainSys(err))?;
    Ok((sdl_context, video_subsystem, event_pump))
}

fn init_gl<'a>(video_subsystem: &'a VideoSubsystem) -> GLAttr<'a> {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // OpenGL 3.3 is the minimum that we will support.
    gl_attr.set_context_version(3, 3);
    return gl_attr;
}

fn main() {
    let (_sdl_context, video_subsystem, mut event_pump) =
        init_system().expect("Error while initializing SDL2");
    let _gl_attr = init_gl(&video_subsystem);

    let mut gb_window = window::GBWindow::new(
        "GBMU",
        (
            render::SCREEN_WIDTH,
            render::SCREEN_HEIGHT + MENU_BAR_SIZE as u32,
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

    let mut triangle = render::Triangle::new();

    let mut debug_window = None;

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        // emulation render here
        triangle.draw();

        // set ui logic here
        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(MENU_BAR_SIZE);
                if ui.button("Load").clicked() {
                    match nfd2::open_file_dialog(None, None).expect("oh no") {
                        Response::Okay(file_path) => println!("File path = {:?}", file_path),
                        Response::OkayMultiple(files) => println!("Files {:?}", files),
                        Response::Cancel => println!("User canceled"),
                    }
                }
                if ui.button("Debug").clicked() {
                    if debug_window.is_none() {
                        debug_window = Some(
                            window::GBWindow::new(
                                "GBMU Debug",
                                (800, 600),
                                false,
                                &video_subsystem,
                            )
                            .expect("Error while building debug window"),
                        );
                    }
                }
            })
        });
        gb_window
            .end_frame()
            .expect("Fail at the end for the main window");

        if let Some(ref mut dgb_wind) = debug_window {
            dgb_wind
                .start_frame()
                .expect("Fail at the start for the debug window");
            egui::containers::CentralPanel::default().show(dgb_wind.egui_ctx(), |ui| {
                ui.label("hello Debug");
            });
            dgb_wind
                .end_frame()
                .expect("Fail at the end for the debug window");
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
                            triangle.resize(gb_window.sdl_window().size());
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
