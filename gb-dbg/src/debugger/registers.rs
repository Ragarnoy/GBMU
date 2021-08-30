use egui::{Ui, Color32};
use egui::Label;

struct Register(char, u8);

pub struct RegisterEditor {
    registers: Vec<Register>,
}

impl Default for RegisterEditor {
    /// Mock for debug purposes
    fn default() -> Self {
        Self {
            registers: vec![
                Register('A', 0x01),
                Register('B', 0x22),
                Register('C', 0x0F),
            ],
        }
    }
}

impl RegisterEditor {
    pub fn draw(&self, ui: &mut Ui) {
        ui.label(Label::new("Register Editors").text_color(Color32::WHITE));
        ui.separator();
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| ui.label("Hello"));
        ui.separator();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.columns(2, |columns| {
                    columns[0].label("Name");
                    columns[0].add(Label::new(self.registers.get(0).unwrap().0));
                    columns[1].label("Value");
                })
            });
        });
    }
}
