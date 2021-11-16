use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
#[cfg(feature = "debug_render")]
use gb_ppu::TILEMAP_DIM;
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
                    if ui.button("tilesheet").clicked() {}
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
                    if ui.button("objects").clicked() {}
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

pub fn new_debug_window(video: &sdl2::VideoSubsystem) -> GBWindow {
    GBWindow::new(
        "GBMU Debug",
        (DEBUGGER_WIDTH as u32, DEBUGGER_HEIGHT as u32),
        false,
        video,
    )
    .expect("Error while building debug window")
}
