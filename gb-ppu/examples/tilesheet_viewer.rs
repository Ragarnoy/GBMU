use sdl2::{event::Event, keyboard::Keycode};

use gb_lcd::{render, window::GBWindow};
use gb_ppu::{PPU, TILESHEET_HEIGHT, TILESHEET_WIDTH};

pub fn main() {
    let (sdl_context, video_subsystem, mut event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let bar_pixels_size = GBWindow::dots_to_pixels(&video_subsystem, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut gb_window = GBWindow::new(
        "TileSheet",
        (
            TILESHEET_WIDTH as u32,
            TILESHEET_HEIGHT as u32 + bar_pixels_size,
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

    let mut display = render::RenderImage::<TILESHEET_WIDTH, TILESHEET_HEIGHT>::with_bar_size(
        bar_pixels_size as f32,
    );
    let mut ppu = PPU::new();
    let dumps = [
        ("mario", include_bytes!("memory dumps/vram/Super_Mario_Land.dmp")),
        (
            "zelda",
            include_bytes!("memory dumps/vram/Legend_of_Zelda_link_Awaking.dmp"),
        ),
        ("pokemon", include_bytes!("memory dumps/vram/Pokemon_Bleue.dmp")),
    ];
    ppu.overwrite_vram(dumps[0].1);
    let mut image = ppu.tilesheet_image();

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                egui::menu::menu(ui, "dump", |ui| {
                    for (title, dump) in dumps {
                        if ui.button(title).clicked() {
                            ppu.overwrite_vram(dump);
                            image = ppu.tilesheet_image();
                        }
                    }
                });
            })
        });
        display.update_render(&image);
        display.draw();
        gb_window
            .end_frame()
            .expect("Fail at the end for the main window");

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
                            gb_window
                                .resize((width as u32, height as u32), &video_subsystem)
                                .expect("Fail to resize example window");
                            display.resize(gb_window.sdl_window().size());
                        }
                    }
                    sdl2::event::WindowEvent::Close => {
                        if gb_window.sdl_window().id() == window_id {
                            break 'running;
                        }
                    }
                    _ => {}
                },
                _ => {
                    gb_window.send_event(&event, &sdl_context);
                }
            }
        }
        // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
