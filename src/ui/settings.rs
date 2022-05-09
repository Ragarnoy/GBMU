use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::config::{Config, Mode};
use crate::{custom_event::CustomEvent, windows::WindowType};

pub(crate) fn draw_ui(ui: &mut Ui, event_proxy: &EventLoopProxy<CustomEvent>, config: &mut Config) {
    ui.menu_button("Settings", |ui| {
        ui.style_mut().override_text_style = None;
        if ui.button("Input").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Keybindings))
                .expect("cannot send open keybindings window event");
        }

        ui.separator();
        if ui.radio_value(&mut config.mode, None, "auto").clicked() {
            event_proxy
                .send_event(CustomEvent::ChangedMode(None))
                .unwrap();
        }
        if ui
            .radio_value(&mut config.mode, Some(Mode::Classic), "dmg")
            .clicked()
        {
            event_proxy
                .send_event(CustomEvent::ChangedMode(Some(Mode::Classic)))
                .unwrap();
        }
        if ui
            .radio_value(&mut config.mode, Some(Mode::Color), "cgb")
            .clicked()
        {
            event_proxy
                .send_event(CustomEvent::ChangedMode(Some(Mode::Color)))
                .unwrap();
        }
    });
}
