use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
#[cfg(feature = "debug_render")]
use gb_ppu::{TILEMAP_DIM, TILESHEET_HEIGHT, TILESHEET_WIDTH};
use native_dialog::FileDialog;

pub fn draw_egui<const WIDTH: usize, const HEIGHT: usize>(
    context: &mut Context<WIDTH, HEIGHT>,
    #[cfg(feature = "debug_render")] game: &Option<Game>,
) {
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
                if let Some(game) = game {
                    if ui.button("tilesheet").clicked() {
                        let tilesheet = GBWindow::new(
                            "ppu tilesheet",
                            (TILESHEET_WIDTH as u32, TILESHEET_HEIGHT as u32),
                            true,
                            &context.video,
                        )
                        .expect("Error while building tilemap window");
                        context.windows.tilesheet = Some((
                            tilesheet,
                            render::RenderImage::<TILESHEET_WIDTH, TILESHEET_HEIGHT>::with_bar_size(
                                0.0,
                            ),
                            game.ppu.tilesheet_image(),
                        ))
                    }
                    if ui.button("tilemap").clicked() && context.windows.tilemap.is_none() {
                        let bar_pixels_size =
                            GBWindow::dots_to_pixels(&context.video, render::MENU_BAR_SIZE)
                                .expect("Error while computing bar size");
                        let tilemap = GBWindow::new(
                            "ppu tilemap",
                            (TILEMAP_DIM as u32, TILEMAP_DIM as u32 + bar_pixels_size),
                            true,
                            &context.video,
                        )
                        .expect("Error while building tilemap window");
                        context.windows.tilemap = Some((
                            tilemap,
                            render::RenderImage::<TILEMAP_DIM, TILEMAP_DIM>::with_bar_size(
                                bar_pixels_size as f32,
                            ),
                            game.ppu.tilemap_image(false),
                            false,
                        ))
                    }
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
    if let Some((ref mut tilemap_window, ref mut display, ref mut image, ref mut display_window)) =
        context.windows.tilemap
    {
        tilemap_window
            .start_frame()
            .expect("Fail at the start for the main window");
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
            *image = game.ppu.tilemap_image(*display_window);
        }
        display.update_render(image);
        display.draw();
        tilemap_window
            .end_frame()
            .expect("Fail at the end for the main window");
    }

    if let Some((ref mut tilesheet_window, ref mut display, ref mut image)) =
        context.windows.tilesheet
    {
        tilesheet_window
            .start_frame()
            .expect("Fail at the start for the main window");
        if let Some(ref mut game) = game {
            *image = game.ppu.tilesheet_image();
        }
        display.update_render(image);
        display.draw();
        tilesheet_window
            .end_frame()
            .expect("Fail at the end for the main window");
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
