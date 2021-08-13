use sdl2::{event::Event, keyboard::Keycode};

use gb_lcd::{render, window::GBWindow};
use gb_ppu::{PPU, TILEMAP_DIM};

pub fn main() {
    let (sdl_context, video_subsystem, mut event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let mut gb_window = GBWindow::new(
        "TileSheet",
        (
            TILEMAP_DIM as u32,
            TILEMAP_DIM as u32 + render::MENU_BAR_SIZE as u32,
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

    let mut display =
        render::RenderImage::<TILEMAP_DIM, TILEMAP_DIM>::with_bar_size(render::MENU_BAR_SIZE);
    let mut ppu = PPU::new();
    ppu.control_mut().set_bg_win_tiledata_area(0);
    let dumps = [
        ("mario", include_bytes!("memory dumps/Super_Mario_Land.dmp")),
        (
            "zelda",
            include_bytes!("memory dumps/Legend_of_Zelda_link_Awaking.dmp"),
        ),
        ("pokemon", include_bytes!("memory dumps/Pokemon_Bleue.dmp")),
    ];
    ppu.overwrite_vram(dumps[0].1);
    let mut image = ppu.tilemap_image();

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                for (title, dump) in dumps {
                    if ui.button(title).clicked() {
                        ppu.overwrite_vram(dump);
                        image = ppu.tilemap_image();
                    }
                }
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
