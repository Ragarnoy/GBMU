mod breakpoint;

use crate::debugger::breakpoints::breakpoint::Breakpoint;
use egui::{Color32, Label, Ui, Vec2};

pub struct BreakpointEditor {
    breakpoints: Vec<Breakpoint>,
    new_address: String,
}

impl Default for BreakpointEditor {
    fn default() -> Self {
        Self {
            breakpoints: Vec::with_capacity(20),
            new_address: String::with_capacity(8),
        }
    }
}

impl BreakpointEditor {
    pub fn draw(&mut self, ui: &mut Ui, pc: u16) {
        ui.label(Label::new("Breakpoints").text_color(Color32::WHITE));
        breakpoint_options(ui);

        ui.separator();
        self.new_address.retain(|c| c.is_ascii_hexdigit());
        if self.new_address.len() <= 5 {
            self.new_address.truncate(4)
        }
        ui.horizontal(|ui| {
            let add_button_response = ui.add(
                egui::Button::new("+")
                    .enabled(BreakpointEditor::is_valid_address(&self.new_address)),
            );
            ui.add(
                egui::Label::new("0x")
                    .text_color(Color32::from_gray(90))
                    .weak(),
            );
            let text_field_response = ui.add(
                egui::TextEdit::singleline(&mut self.new_address)
                    .desired_width(85.0)
                    .hint_text("555F"),
            );
            if (add_button_response.clicked()
                || (text_field_response.clicked() && ui.input().key_pressed(egui::Key::Enter)))
                && BreakpointEditor::is_valid_address(&self.new_address)
            {
                self.add_address_breakpoint(u16::from_str_radix(&self.new_address, 16).unwrap());
            }
            if text_field_response.lost_focus() {
                self.new_address.clear();
            }
        });

        let mut deletion_list: Vec<usize> = Vec::with_capacity(20);
        egui::Grid::new("breakpoints_".to_owned())
            .striped(true)
            .spacing(Vec2::new(6.5, 2.5))
            .show(ui, |ui| {
                ui.label(egui::Label::new("Delete"));
                ui.label(egui::Label::new("Active"));
                ui.label(egui::Label::new("Address"));
                ui.end_row();

                for (i, breakpoint) in &mut self.breakpoints.iter_mut().enumerate() {
                    if ui.add(egui::Button::new("-")).clicked() {
                        deletion_list.push(i)
                    }
                    ui.checkbox(&mut breakpoint.enabled, "");
                    if pc == breakpoint.address() && breakpoint.enabled {
                        ui.add(
                            egui::Label::new(breakpoint.to_string().clone())
                                .text_color(Color32::RED),
                        );
                    } else {
                        ui.add(egui::Label::new(breakpoint.to_string().clone()));
                    }
                    ui.end_row();
                }
                ui.end_row();
            });
        deletion_list.into_iter().for_each(|i| {
            self.breakpoints.remove(i);
        });
    }

    fn add_address_breakpoint(&mut self, address: u16) {
        if !self.breakpoints.iter().any(|x| x.address() == address) {
            self.breakpoints.push(Breakpoint::from_address(address));
        }
    }

    fn is_valid_address(address: &str) -> bool {
        !address.is_empty() && u16::from_str_radix(address, 16).is_ok()
    }

    pub fn are_breakpoints_triggered(&mut self, pc: u16) -> bool {
        for breakpoint in &mut self.breakpoints {
            if breakpoint.enabled && pc == breakpoint.address() {
                breakpoint.enabled = false;
                return true;
            }
        }
        false
    }
}

fn breakpoint_options(ui: &mut Ui) {
    egui::CollapsingHeader::new("🛠 Options")
        .id_source(55)
        .default_open(false)
        .show(ui, |ui| ui.label("Hello"));
}
