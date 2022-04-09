mod file;

use crate::Context;
#[cfg(feature = "debug_render")]
use crate::Game;
#[cfg(feature = "cgb")]
use crate::Opts;
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
            egui::containers::TopBottomPanel::top("Top menu").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.set_height(crate::constant::MENU_BAR_SIZE);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
                    file::draw_ui(ui, &context.event_proxy);
                    // ui_debug!(ui, context);
                    // ui_settings!(ui, context, options, &mut context.custom_events);
                    // ui.style_mut().override_text_style = None;
                    #[cfg(feature = "debug_fps")]
                    ui_fps!(ui, context, fps);
                });
            });
        })
}
