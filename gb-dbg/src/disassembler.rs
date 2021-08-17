use egui::{Ui, Color32, Label};

pub struct Disassembler;


impl Disassembler {

    pub fn draw(&self, ui: &mut Ui) {
        ui.label(Label::new("Disassembler").text_color(Color32::WHITE));
        egui::CollapsingHeader::new("🛠 Options").id_source(55)
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Hello")
            });
        ui.separator();
        ui.vertical(|ui| {
            ui.columns(3, |columns| {
                columns[0].label("Address");
                columns[1].label("Instruction");
                columns[2].label("Data");
            })
        });
        ui.add_space(200.0);
    }
}
