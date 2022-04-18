mod file;
mod settings;
mod tools;

use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
#[cfg(feature = "cgb")]
use crate::Opts;
// use egui::{Color32, Visuals};
#[cfg(feature = "debug_render")]
use native_dialog::FileDialog;

use gb_lcd::DrawEgui;

#[cfg(feature = "debug_fps")]
macro_rules! ui_fps {
    ($ui:expr, $context:expr, $fps:expr) => {
        $ui.add_space($ui.available_size().x - 50.0);
        $ui.label((format!("{:>7.2}", $fps)));
    };
}

pub fn draw_egui(
    context: &mut Context,
    #[cfg(feature = "cgb")] options: &mut Opts,
    #[cfg(feature = "debug_fps")] fps: f64,
) {
    context
        .windows
        .main
        .context
        .prepare_egui(&context.windows.main.window, |egui_ctx| {
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
            egui::containers::TopBottomPanel::top("Top menu").show(egui_ctx, |ui| {
                // egui::containers::CentralPanel::default().show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    // ui.set_height(crate::constant::MENU_BAR_SIZE);
                    // ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                    file::draw_ui(ui, &context.event_proxy);
                    tools::draw_ui(ui, &context.event_proxy);
                    settings::draw_ui(
                        ui,
                        &context.event_proxy,
                        #[cfg(feature = "cgb")]
                        &mut context.config,
                    );
                    // ui_debug!(ui, context);
                    // ui_settings!(ui, context, options, &mut context.custom_events);
                    // ui.style_mut().override_text_style = None;
                    #[cfg(feature = "debug_fps")]
                    ui_fps!(ui, context, fps);
                });
            });
        })
}
