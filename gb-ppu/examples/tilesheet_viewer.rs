use sdl2::{event::Event, keyboard::Keycode};

use gb_lcd::{render, window::GBWindow};
use gb_ppu::{PPU, TILESHEET_HEIGHT, TILESHEET_WIDTH};

pub fn tilesheet_viewer(bytes: [u8; 0x2000]) {
    let (sdl_context, video_subsystem, mut event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let mut gb_window = GBWindow::new(
        "TileSheet",
        (
            TILESHEET_WIDTH as u32,
            TILESHEET_HEIGHT as u32 + render::MENU_BAR_SIZE as u32,
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
        render::MENU_BAR_SIZE,
    );
    let mut ppu = PPU::new();
    ppu.overwrite_vram(bytes);
    let mut image = ppu.tilesheet_image();

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                if ui.button("refresh").clicked() {
                    println!("refresh tilesheet image");
                    image = ppu.tilesheet_image();
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

#[allow(dead_code)]
fn main() {
    tilesheet_viewer([0; 0x2000]);
}
