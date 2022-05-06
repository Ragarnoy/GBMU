use egui::Ui;
use native_dialog::FileDialog;
use winit::event_loop::EventLoopProxy;

use crate::custom_event::CustomEvent;

pub(crate) fn draw_ui(ui: &mut Ui, event_proxy: &EventLoopProxy<CustomEvent>) {
    ui.menu_button("File", |ui| {
        ui.style_mut().override_text_style = None;
        if ui.button("Load").clicked() {
            let file = FileDialog::new()
                .set_location(
                    &std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
                )
                .add_filter("rom", &crate::constant::PREFERED_ROM_EXTS)
                .show_open_single_file();
            log::debug!("picked rom file {file:?}");
            if let Ok(Some(path)) = file {
                event_proxy
                    .send_event(CustomEvent::LoadFile(path))
                    .expect("cannot send load file event");
            }
        }
        {
            ui.separator();
            if ui.button("Save As").clicked() {
                todo!("save as")
            }
            if ui.button("Load Save").clicked() {
                todo!("load as")
            }
        }
    });
}
