use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use gb_lcd::{render, window::GBWindow};
use native_dialog::FileDialog;

pub fn draw_egui(
    window: &mut GBWindow,
    debug_window: &mut Option<GBWindow>,
    video: &sdl2::VideoSubsystem,
    input: &mut Option<GBWindow>,
) {
    egui::containers::TopBottomPanel::top("Top menu").show(window.egui_ctx(), |ui| {
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
            if ui.button("Debug").clicked() && debug_window.is_none() {
                debug_window.replace(new_debug_window(video));
            }
            #[cfg(feature = "debug_render")]
            egui::menu::menu(ui, "PPU", |ui| {
                if ui.button("tilesheet").clicked() {}
                if ui.button("tilemap").clicked() {}
                if ui.button("objects").clicked() {}
            });
            if ui.button("Input").clicked() && input.is_none() {
                input.replace(
                    GBWindow::new(
                        "GBMU Input Settings",
                        (
                            GBWindow::dots_to_pixels(video, 250.0)
                                .expect("error while computing window size"),
                            GBWindow::dots_to_pixels(video, 250.0)
                                .expect("error while computing window size"),
                        ),
                        false,
                        video,
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
