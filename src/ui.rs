mod file;
#[cfg(feature = "fps")]
mod fps;
mod settings;
mod tools;

use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
use gb_lcd::DrawEgui;
#[cfg(feature = "debug_render")]
use native_dialog::FileDialog;

pub fn draw_egui(context: &mut Context) {
    let (size, margin) = context.main_window.texture_size_and_margin();
    context
        .main_window
        .context
        .prepare_egui(&context.main_window.window, |egui_ctx| {
            let mut top_frame = egui::Frame::menu(&egui::style::Style::default());
            top_frame.margin = egui::style::Margin::symmetric(5.0, 0.0);
            egui::containers::TopBottomPanel::top("Top menu")
                .frame(top_frame)
                .show(egui_ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.set_height(crate::constant::MENU_BAR_SIZE - 1.0);
                        file::draw_ui(ui, &context.event_proxy);
                        tools::draw_ui(ui, &context.event_proxy);
                        settings::draw_ui(ui, &context.event_proxy, &mut context.config.mode);
                        #[cfg(feature = "fps")]
                        if ui.available_width() >= fps::FPS_WIDTH {
                            fps::draw_ui(ui, &context.time_frame);
                        }
                    });
                });
            let mut central_frame = egui::Frame::none();
            central_frame.margin = egui::style::Margin::symmetric(margin.0, margin.1);
            central_frame.margin.top += 1.0;
            egui::containers::CentralPanel::default()
                .frame(central_frame)
                .show(egui_ctx, |ui| {
                    ui.image(context.main_window.texture_id, size);
                });
        })
}
