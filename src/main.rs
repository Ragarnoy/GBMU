use rfd::FileDialog;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{gl_attr::GLAttr, GLProfile},
    EventPump, Sdl, VideoSubsystem,
};

use gb_lcd::{error::Error as LCDError, render, window::GBWindow};
use gb_ppu::PPU;

fn init_system() -> Result<(Sdl, VideoSubsystem, EventPump), LCDError> {
    let sdl_context = sdl2::init().map_err(|err| LCDError::MainSys(err))?;
    let video_subsystem = sdl_context.video().map_err(|err| LCDError::MainSys(err))?;

    let event_pump = sdl_context
        .event_pump()
        .map_err(|err| LCDError::MainSys(err))?;
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
    let (sdl_context, video_subsystem, mut event_pump) =
        init_system().expect("Error while initializing SDL2");
    let _gl_attr = init_gl(&video_subsystem);

    let mut gb_window = GBWindow::new(
        "GBMU",
        (
            render::SCREEN_WIDTH,
            render::SCREEN_HEIGHT + render::MENU_BAR_SIZE as u32,
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

    let mut display = render::Render::new();
    let mut ppu = PPU::new();

    let mut debug_window = None;

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        // render is updated just before drawing for now but we might want to change that later
        ppu.compute();
        display.update_render(ppu.pixels());
        // emulation render here
        display.draw();

        // set ui logic here
        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                if ui.button("Load").clicked() {
                    let files = FileDialog::new()
                        .add_filter("rom", &["gb", "rom"])
                        .set_directory(
                            std::env::current_dir().unwrap_or(std::path::PathBuf::from("/")),
                        )
                        .pick_file();
                    println!("picked file: {:?}", files);
                }
                if ui.button("Debug").clicked() {
                    if debug_window.is_none() {
                        debug_window = Some(
                            GBWindow::new("GBMU Debug", (800, 600), false, &video_subsystem)
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
                    // here for debug, maybe remove later ?
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
                            gb_window
                                .resize((width as u32, height as u32), &video_subsystem)
                                .expect("Fail to resize GB window");
                            display.resize(gb_window.sdl_window().size());
                        } else if let Some(ref mut dbg_wind) = debug_window {
                            if dbg_wind.sdl_window().id() == window_id {
                                dbg_wind
                                    .resize((width as u32, height as u32), &video_subsystem)
                                    .expect("Fail to resize debug window");
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
                    if !gb_window.send_event(&event, &sdl_context) {
                        if let Some(ref mut dbg_wind) = debug_window {
                            dbg_wind.send_event(&event, &sdl_context);
                        }
                    }
                }
            }
        }
        // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
