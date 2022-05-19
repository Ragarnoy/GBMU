use crate::game::Game;
use egui::color::Hsva;
use egui::{Color32, Ui};

pub(crate) fn draw_ui(ui: &mut Ui, game: &mut Option<Game>) {
    ui.spacing_mut().item_spacing = (5.0, 5.0).into();
    ui.style_mut().spacing.slider_width = 150.0;

    if let Some(ref mut game_ref) = game {
        let mut apu = game_ref.apu.borrow_mut();
        ui.style_mut().visuals.widgets.inactive.bg_fill =
            Color32::from(Hsva::new(0.0, *apu.output_volume() as f32, 0.35, 1.0));
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            ui.add(egui::Slider::new::<f32>(apu.output_volume(), 0.0..=1.0).show_value(false));
            ui.add(egui::Label::new("Volume"));
        });
    } else {
        ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_gray(50);
        ui.add_enabled(
            false,
            egui::Slider::new::<f32>(&mut 0.7, 0.0..=1.0).show_value(false),
        );
        ui.add(egui::Label::new("Volume"));
    }
}
