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
        if ui.button("ppu tilesheet").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Tilesheet))
                .expect("cannot send open tilesheet window event");
        }
        if ui.button("ppu tilemap").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Tilemap))
                .expect("cannot send open tilemap window event");
        }
        if ui.button("ppu spritesheet").clicked() {
            event_proxy
                .send_event(CustomEvent::OpenWindow(WindowType::Spritesheet))
                .expect("cannot send open spritesheet window event");
        }
    });
}
