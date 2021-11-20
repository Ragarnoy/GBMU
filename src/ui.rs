use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
#[cfg(feature = "debug_render")]
use gb_ppu::{
    SPRITE_LIST_RENDER_HEIGHT, SPRITE_LIST_RENDER_WIDTH, SPRITE_RENDER_HEIGHT, SPRITE_RENDER_WIDTH,
    TILEMAP_DIM, TILESHEET_HEIGHT, TILESHEET_WIDTH,
};
use native_dialog::FileDialog;

pub fn draw_egui<const WIDTH: usize, const HEIGHT: usize>(context: &mut Context<WIDTH, HEIGHT>) {
    egui::containers::TopBottomPanel::top("Top menu").show(context.windows.main.egui_ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_height(render::MENU_BAR_SIZE);
            if ui.button("Load").clicked() {
                let files = FileDialog::new()
                    .set_location(
                        &std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
                    )
                    .add_filter("rom", &["gb", "gbc", "rom"])
                    .show_open_single_file();
                log::debug!("picked file: {:?}", files);
            }
            if ui.button("Debug").clicked() && context.windows.debug.is_none() {
                context
                    .windows
                    .debug
                    .replace(new_debug_window(&context.video));
            }
            #[cfg(feature = "debug_render")]
            egui::menu::menu(ui, "PPU", |ui| {
                if ui.button("tilesheet").clicked() && context.windows.tilesheet.is_none() {
                    let mut tilesheet = GBWindow::new(
                        "ppu tilesheet",
                        (TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32),
                        true,
                        &context.video,
                    )
                    .expect("Error while building tilesheet window");
                    tilesheet
                        .sdl_window_mut()
                        .set_minimum_size(TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32)
                        .expect("Failed to configure tilesheet window");
                    context.windows.tilesheet = Some((
                        tilesheet,
                        render::RenderImage::<TILESHEET_WIDTH, TILESHEET_HEIGHT>::with_bar_size(
                            0.0,
                        ),
                    ))
                }
                if ui.button("tilemap").clicked() && context.windows.tilemap.is_none() {
                    let bar_pixels_size =
                        GBWindow::dots_to_pixels(&context.video, render::MENU_BAR_SIZE)
                            .expect("Error while computing bar size");
                    let mut tilemap = GBWindow::new(
                        "ppu tilemap",
                        (TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size),
                        true,
                        &context.video,
                    )
                    .expect("Error while building tilemap window");
                    tilemap
                        .sdl_window_mut()
                        .set_minimum_size(TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size)
                        .expect("Failed to configure tilemap window");
                    context.windows.tilemap = Some((
                        tilemap,
                        render::RenderImage::<TILEMAP_DIM, TILEMAP_DIM>::with_bar_size(
                            bar_pixels_size as f32,
                        ),
                        false,
                    ))
                }
                if ui.button("objects").clicked() && context.windows.oam.is_none() {
                    let bar_pixels_size =
                        GBWindow::dots_to_pixels(&context.video, render::MENU_BAR_SIZE)
                            .expect("Error while computing bar size");
                    let mut oam = GBWindow::new(
                        "ppu oam",
                        (SPRITE_RENDER_WIDTH as u32, SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size),
                        true,
                        &context.video,
                    )
                    .expect("Error while building oam window");
                    oam.sdl_window_mut()
                        .set_minimum_size(SPRITE_RENDER_WIDTH as u32, SPRITE_RENDER_HEIGHT as u32 + bar_pixels_size)
                        .expect("Failed to configure oam window");
                    context.windows.oam = Some((
                        oam,
                        render::RenderImage::<SPRITE_RENDER_WIDTH, SPRITE_RENDER_HEIGHT>::with_bar_size(
                            bar_pixels_size as f32,
                        ),
                        render::RenderImage::<SPRITE_LIST_RENDER_WIDTH, SPRITE_LIST_RENDER_HEIGHT>::with_bar_size(
                            bar_pixels_size as f32,
                        ),
                        false
                    ))
                }
            });
            if ui.button("Input").clicked() && context.windows.input.is_none() {
                context.windows.input.replace(
                    GBWindow::new(
                        "GBMU Input Settings",
                        (
                            GBWindow::dots_to_pixels(&context.video, 250.0)
                                .expect("error while computing window size"),
                            GBWindow::dots_to_pixels(&context.video, 250.0)
                                .expect("error while computing window size"),
                        ),
                        false,
                        &context.video,
                    )
                    .expect("Error while building input window"),
                );
            }
        })
    });
}

#[cfg(feature = "debug_render")]
pub fn draw_ppu_debug_ui<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    game: &mut Option<Game>,
) {
    if let Some((ref mut tilemap_window, ref mut display, ref mut display_window)) =
        context.windows.tilemap
    {
        tilemap_window
            .start_frame()
            .expect("Fail at the start for the tilemap window");
        if let Some(ref mut game) = game {
            egui::containers::TopBottomPanel::top("Top menu").show(
                tilemap_window.egui_ctx(),
                |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.set_height(render::MENU_BAR_SIZE);
                        egui::menu::menu(ui, "bg/win", |ui| {
                            if ui.button("background").clicked() {
                                *display_window = false;
                            }
                            if ui.button("window").clicked() {
                                *display_window = true;
                            }
                        });
                    })
                },
            );
            display.update_render(&game.ppu.tilemap_image(*display_window));
        }
        display.draw();
        tilemap_window
            .end_frame()
            .expect("Fail at the end for the tilemap window");
    }

    if let Some((ref mut tilesheet_window, ref mut display)) = context.windows.tilesheet {
        tilesheet_window
            .start_frame()
            .expect("Fail at the start for the tilesheet window");
        if let Some(ref mut game) = game {
            display.update_render(&game.ppu.tilesheet_image());
        }
        display.draw();
        tilesheet_window
            .end_frame()
            .expect("Fail at the end for the tilesheet window");
    }

    if let Some((
        ref mut oam_window,
        ref mut display_view,
        ref mut display_list,
        ref mut display_mode,
    )) = context.windows.oam
    {
        oam_window
            .start_frame()
            .expect("Fail at the start for the oam window");
        if let Some(ref mut game) = game {
            egui::containers::TopBottomPanel::top("Top menu").show(oam_window.egui_ctx(), |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.set_height(render::MENU_BAR_SIZE);
                    egui::menu::menu(ui, "mode", |ui| {
                        if ui.button("viewport").clicked() {
                            *display_mode = false;
                        }
                        if ui.button("list").clicked() {
                            *display_mode = true;
                        }
                    });
                })
            });
            if !*display_mode {
                display_view.update_render(&game.ppu.sprites_image());
                display_view.draw();
            } else {
                display_list.update_render(&game.ppu.sprites_list_image());
                display_list.draw();
            }
        }
        oam_window
            .end_frame()
            .expect("Fail at the end for the oam window");
    }
}

pub fn new_debug_window(video: &sdl2::VideoSubsystem) -> GBWindow {
    GBWindow::new(
        "GBMU Debug",
        (DEBUGGER_WIDTH as u32, DEBUGGER_HEIGHT as u32),
        false,
        video,
    )
    .expect("Error while building debug window")
}
