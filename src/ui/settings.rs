use egui::Ui;
use winit::event_loop::EventLoopProxy;

use crate::config::Mode;
use crate::{custom_event::CustomEvent, game::Game, windows::WindowType};

pub(crate) fn draw_ui(
    ui: &mut Ui,
    event_proxy: &EventLoopProxy<CustomEvent>,
    mode: &mut Option<Mode>,
    game: &mut Option<Game>,
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
        ui.separator();
        if let Some(ref mut game_ref) = game {
            let mut apu = game_ref.apu.borrow_mut();
            ui.spacing_mut().item_spacing = (5.0, 5.0).into();
            ui.add(egui::Label::new("Volume"));
            ui.style_mut().spacing.slider_width = 150.0;
            ui.add(egui::Slider::new::<f32>(apu.output_volume(), 0.0..=1.0).show_value(false));
        };
    });
}
