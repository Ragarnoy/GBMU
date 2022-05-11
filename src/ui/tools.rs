use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::{custom_event::CustomEvent, windows::WindowType};

pub(crate) fn draw_ui(ui: &mut Ui, event_proxy: &EventLoopProxy<CustomEvent>) {
    ui.menu_button("Tools", |ui| {
        if ui.button("cpu debugger").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Debugger(None)))
                .expect("cannot send open debugger window event");
        }
    });
}
