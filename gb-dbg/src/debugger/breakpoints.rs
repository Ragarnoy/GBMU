mod breakpoint;

use egui::{Color32, Label, Ui, Vec2};
use crate::dbg_interfaces::RegisterDebugOperations;
use crate::debugger::breakpoints::breakpoint::Breakpoint;

pub struct BreakpointEditor {
    breakpoints: Vec<Breakpoint>,
}

impl BreakpointEditor {
    pub fn draw<MEM: RegisterDebugOperations>(&mut self, ui: &mut Ui, memory: &MEM) {
        ui.label(Label::new("Breakpoints").text_color(Color32::WHITE));
        breakpoint_options(ui);
        ui.separator();
        egui::Grid::new("dissas_".to_owned())
            .striped(true)
            .spacing(Vec2::new(2.5, 2.5))
            .show(ui, |ui| {
                ui.label(egui::Label::new("Active"));
                ui.label(egui::Label::new("Address"));
                ui.end_row();
                for breakpoint in &mut self.breakpoints {
                    let checked: &mut bool = &mut breakpoint.enabled;
                    let address = breakpoint.r#type();
                    ui.checkbox(checked, breakpoint.r#type());
                    ui.label(egui::Label::new("Address"));
                    ui.end_row();
                }
                ui.end_row();
            });
    }
}

fn breakpoint_options(ui: &mut Ui) {
    egui::CollapsingHeader::new("🛠 Options")
        .id_source(55)
        .default_open(false)
        .show(ui, |ui| ui.label("Hello"));
}