use sdl2::{event::Event, keyboard::Keycode};

use gb_lcd::{render, window::GBWindow};
use gb_ppu::{
    OBJECT_LIST_RENDER_HEIGHT, OBJECT_LIST_RENDER_WIDTH, OBJECT_RENDER_HEIGHT, OBJECT_RENDER_WIDTH,
    PPU,
};

pub fn main() {
    let (sdl_context, video_subsystem, mut event_pump) =
        gb_lcd::init().expect("Error while initializing LCD");

    let bar_pixels_size = GBWindow::dots_to_pixels(&video_subsystem, render::MENU_BAR_SIZE)
        .expect("Error while computing bar size");
    let mut gb_window = GBWindow::new(
        "Objects",
        (
            OBJECT_RENDER_WIDTH as u32,
            OBJECT_RENDER_HEIGHT as u32 + bar_pixels_size,
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

    let mut view_display =
        render::RenderImage::<OBJECT_RENDER_WIDTH, OBJECT_RENDER_HEIGHT>::with_bar_size(
            bar_pixels_size as f32,
        );
    let mut list_display =
        render::RenderImage::<OBJECT_LIST_RENDER_WIDTH, OBJECT_LIST_RENDER_HEIGHT>::with_bar_size(
            bar_pixels_size as f32,
        );
    let mut ppu = PPU::new();
    let dumps = [
        (
            "mario",
            include_bytes!("memory dumps/vram/Super_Mario_Land.dmp"),
            include_bytes!("memory dumps/oam/Super_Mario_Land.dmp"),
            include_bytes!("memory dumps/io_registers/Super_Mario_Land.dmp"),
        ),
        (
            "zelda",
            include_bytes!("memory dumps/vram/Legend_of_Zelda_link_Awaking.dmp"),
            include_bytes!("memory dumps/oam/Legend_of_Zelda_link_Awaking.dmp"),
            include_bytes!("memory dumps/io_registers/Legend_of_Zelda_link_Awaking.dmp"),
        ),
        (
            "pokemon",
            include_bytes!("memory dumps/vram/Pokemon_Bleue.dmp"),
            include_bytes!("memory dumps/oam/Pokemon_Bleue.dmp"),
            include_bytes!("memory dumps/io_registers/Pokemon_Bleue.dmp"),
        ),
    ];
    ppu.overwrite_vram(dumps[0].1);
    ppu.overwrite_oam(dumps[0].2);
    *ppu.bg_palette_mut() = dumps[0].3[0x47].into();
    *ppu.obj_palette_0_mut() = dumps[0].3[0x48].into();
    *ppu.obj_palette_1_mut() = dumps[0].3[0x49].into();
    ppu.control_mut()
        .set_win_tilemap_area((dumps[0].3[0x40] & 0b0100_0000) != 0);
    ppu.control_mut()
        .set_bg_tilemap_area((dumps[0].3[0x40] & 0b0000_1000) != 0);
    ppu.control_mut()
        .set_bg_win_tiledata_area((dumps[0].3[0x40] & 0b0001_0000) != 0);
    ppu.control_mut()
        .set_obj_size((dumps[0].3[0x40] & 0b0000_0100) != 0);
    let mut list_mode = false;
    let mut view_image = ppu.objects_image();
    let mut list_image = ppu.objects_list_image();

    'running: loop {
        gb_window
            .start_frame()
            .expect("Fail at the start for the main window");

        egui::containers::TopBottomPanel::top("Top menu").show(gb_window.egui_ctx(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(render::MENU_BAR_SIZE);
                egui::menu::menu(ui, "dump", |ui| {
                    for (title, vram, oam, io_reg) in dumps {
                        if ui.button(title).clicked() {
                            ppu.overwrite_vram(vram);
                            ppu.overwrite_oam(oam);
                            *ppu.bg_palette_mut() = io_reg[0x47].into();
                            *ppu.obj_palette_0_mut() = io_reg[0x48].into();
                            *ppu.obj_palette_1_mut() = io_reg[0x49].into();
                            ppu.control_mut()
                                .set_win_tilemap_area((io_reg[0x40] & 0b0100_0000) != 0);
                            ppu.control_mut()
                                .set_bg_tilemap_area((io_reg[0x40] & 0b0000_1000) != 0);
                            ppu.control_mut()
                                .set_bg_win_tiledata_area((io_reg[0x40] & 0b0001_0000) != 0);
                            ppu.control_mut()
                                .set_obj_size((io_reg[0x40] & 0b0000_0100) != 0);
                            view_image = ppu.objects_image();
                            list_image = ppu.objects_list_image();
                        }
                    }
                });
                egui::menu::menu(ui, "mode", |ui| {
                    if ui.button("viewport").clicked() {
                        list_mode = false;
                    }
                    if ui.button("list").clicked() {
                        list_mode = true;
                    }
                });
            })
        });
        if !list_mode {
            view_display.update_render(&view_image);
            view_display.draw();
        } else {
            list_display.update_render(&list_image);
            list_display.draw();
        }
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
                            view_display.resize(gb_window.sdl_window().size());
                            list_display.resize(gb_window.sdl_window().size());
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
