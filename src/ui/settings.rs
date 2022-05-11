use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::config::Mode;
use crate::{custom_event::CustomEvent, windows::WindowType};

pub(crate) fn draw_ui(
    ui: &mut Ui,
    event_proxy: &EventLoopProxy<CustomEvent>,
    mode: &mut Option<Mode>,
) {
    ui.menu_button("Settings", |ui| {
        ui.style_mut().override_text_style = None;
        if ui.button("Input").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Keybindings))
                .expect("cannot send open keybindings window event");
        }

        ui.separator();
        if ui.radio_value(mode, None, "auto").clicked() {
            event_proxy
                .send_event(CustomEvent::ChangedMode(None))
                .unwrap();
        }
        if ui.radio_value(mode, Some(Mode::Classic), "dmg").clicked() {
            event_proxy
                .send_event(CustomEvent::ChangedMode(Some(Mode::Classic)))
                .unwrap();
        }
        if ui.radio_value(mode, Some(Mode::Color), "cgb").clicked() {
            event_proxy
                .send_event(CustomEvent::ChangedMode(Some(Mode::Color)))
                .unwrap();
        }
    });
}
