mod breakpoint;
mod breakpoint_node;

use crate::debugger::breakpoints::breakpoint::Breakpoint;
use crate::until::Until;
use egui::{Color32, Label, Ui, Vec2};
use std::ops::ControlFlow;
use crate::dbg_interfaces::RegisterDebugOperations;

#[derive(Default, Debug)]
pub struct BreakpointOptions {
    is_advanced: bool,
    is_not: bool,
}

#[derive(Debug)]
pub struct BreakpointEditor {
    breakpoints: Vec<Breakpoint>,
    breakpoint_field: String,
    pub options: BreakpointOptions,
}

impl Default for BreakpointEditor {
    fn default() -> Self {
        Self {
            breakpoints: Vec::with_capacity(20),
            breakpoint_field: String::with_capacity(8),
            options: Default::default(),
        }
    }
}

impl BreakpointEditor {
    pub fn draw<T: RegisterDebugOperations>(&mut self, ui: &mut Ui, regs: &T) -> Option<ControlFlow<Until>> {
        ui.label(Label::new("Breakpoints").text_color(Color32::WHITE));
        self.draw_breakpoint_options(ui);

        let mut ret = None;
        ui.separator();
        if self.options.is_advanced {
            self.draw_advanced_breakpoint_widget(ui, regs);
        }
        else {
            self.draw_simple_breakpoint_widget(ui, regs);
        }

        let mut deletion_list: Vec<usize> = Vec::with_capacity(20);
        egui::Grid::new("breakpoints_".to_owned())
            .striped(true)
            .spacing(Vec2::new(60.5, 6.5))
            .show(ui, |ui| {
                ui.label(egui::Label::new("Delete"));
                ui.label(egui::Label::new("Active"));
                ui.label(egui::Label::new("Condition"));
                ui.end_row();

                for (i, breakpoint) in &mut self.breakpoints.iter_mut().enumerate() {
                    if ui.add(egui::Button::new("-").text_color(Color32::RED)).clicked() {
                        deletion_list.push(i)
                    }
                    ui.checkbox(&mut breakpoint.enabled, "");
                    if breakpoint.is_triggered(regs) {
                        ui.add(
                            egui::Label::new(breakpoint.to_string().clone())
                                .text_color(Color32::RED),
                        );
                        ret = Some(ControlFlow::Break(Until::Null));
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
        ret
    }

    fn add_address_breakpoint<T: RegisterDebugOperations>(&mut self, address: u16, regs: &T) {
        if !self.breakpoints.iter().any(|x| x.is_triggered(regs)) {
            self.breakpoints.push(Breakpoint::from_address(address));
        }
    }

    fn is_valid_address(&self, address: &str) -> bool {
        address.len() == 4 && u16::from_str_radix(address, 16).is_ok()
    }

    fn draw_advanced_breakpoint_widget<T: RegisterDebugOperations>(&mut self, ui: &mut Ui, regs: &T) {
        ui.horizontal(|ui| {
            let add_button_response =
                ui.add(egui::Button::new("+").enabled(self.is_valid_address(&self.breakpoint_field)));
            ui.checkbox(&mut self.options.is_not, "NOT");
        });
        let text_field_response = ui.add(
            egui::TextEdit::singleline(&mut self.breakpoint_field)
                .desired_width(150.0)
                .hint_text("AF == 0x80"),
        );
    }

    fn draw_simple_breakpoint_widget<T: RegisterDebugOperations>(&mut self, ui: &mut Ui, regs: &T) {
        self.breakpoint_field.retain(|c| c.is_ascii_hexdigit());
        if self.breakpoint_field.len() <= 5 {
            self.breakpoint_field.truncate(4)
        }
        ui.horizontal(|ui| {
            let add_button_response =
                ui.add(egui::Button::new("+").enabled(self.is_valid_address(&self.breakpoint_field)));
            ui.add(
                egui::Label::new("0x")
                    .text_color(Color32::from_gray(90))
                    .weak(),
            );
            let text_field_response = ui.add(
                egui::TextEdit::singleline(&mut self.breakpoint_field)
                    .desired_width(85.0)
                    .hint_text("555F"),
            );
            if add_button_response.clicked()
                || text_field_response.clicked()
                && ui.input().key_pressed(egui::Key::Enter)
                && self.is_valid_address(&self.breakpoint_field)
            {
                self.add_address_breakpoint(u16::from_str_radix(&*self.breakpoint_field, 16).unwrap(), regs);
            }
            if text_field_response.lost_focus() {
                self.breakpoint_field.clear();
            }
        });
    }

    fn draw_breakpoint_options(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| {
                ui.checkbox(&mut self.options.is_advanced, "Advanced")
            });
    }
}
