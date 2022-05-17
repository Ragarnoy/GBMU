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
            // let c1 = Color32::from_rgba_unmultiplied(168, 115, 232, 64);
            // let c2 = Color32::from_rgba_unmultiplied(241, 91, 181, 128);
            // let c3 = Color32::from_rgba_unmultiplied(254, 228, 64, 255);
            // let c4 = Color32::from_rgba_unmultiplied(0, 187, 249, 255);
            // let c5 = Color32::from_rgba_unmultiplied(0, 245, 212, 255);
            // use egui::{
            //     style::{WidgetVisuals, Widgets},
            //     Stroke,
            // };

            // let widget_style = Widgets {
            //     noninteractive: WidgetVisuals {
            //         bg_fill: c1,
            //         bg_stroke: Stroke::none(),
            //         corner_radius: 3.,
            //         fg_stroke: Stroke::none(),
            //         expansion: 3.,
            //     },
            //     inactive: WidgetVisuals {
            //         bg_fill: c2,
            //         bg_stroke: Stroke::none(),
            //         corner_radius: 3.,
            //         fg_stroke: Stroke::none(),
            //         expansion: 3.,
            //     },
            //     hovered: WidgetVisuals {
            //         bg_fill: c3,
            //         bg_stroke: Stroke::none(),
            //         corner_radius: 3.,
            //         fg_stroke: Stroke::none(),
            //         expansion: 3.,
            //     },
            //     active: WidgetVisuals {
            //         bg_fill: c4,
            //         bg_stroke: Stroke::none(),
            //         corner_radius: 3.,
            //         fg_stroke: Stroke::none(),
            //         expansion: 3.,
            //     },
            //     open: WidgetVisuals {
            //         bg_fill: c5,
            //         bg_stroke: Stroke::none(),
            //         corner_radius: 3.,
            //         fg_stroke: Stroke::none(),
            //         expansion: 3.,
            //     },
            // };
            // egui_ctx.set_visuals(Visuals {
            //     // dark_mode: false,
            //     // code_bg_color: Color32::from_rgba_unmultiplied(128, 18, 43, 128),
            //     widgets: widget_style,
            //     ..Default::default()
            // });

            let mut top_frame = egui::Frame::menu(&egui::style::Style::default());
            top_frame.margin = egui::style::Margin::symmetric(5.0, 0.0);
            egui::containers::TopBottomPanel::top("Top menu")
                .frame(top_frame)
                .show(egui_ctx, |ui| {
                    // egui::containers::CentralPanel::default().show(egui_ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.set_height(crate::constant::MENU_BAR_SIZE - 1.0);
                        // ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                        file::draw_ui(ui, &context.event_proxy);
                        tools::draw_ui(ui, &context.event_proxy);
                        settings::draw_ui(
                            ui,
                            &context.event_proxy,
                            &mut context.config.mode,
                            &mut context.game,
                        );
                        // ui_debug!(ui, context);
                        // ui.style_mut().override_text_style = None;
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
