mod breakpoint;
mod parser;
mod evaluation;
mod test_parser;

use crate::dbg_interfaces::DebugOperations;
use crate::debugger::breakpoints::breakpoint::Breakpoint;

use egui::{Color32, Label, Ui, Vec2, Visuals};

const VALID_CHARS: &[char] = &['&', '|', '!', '=', '<', '>', '*', '%', '^', '(', ')'];

#[derive(Default, Debug)]
pub struct BreakpointOptions {
    is_advanced: bool,
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
    pub fn new(breakpoint_list: Vec<String>) -> Self {
        if breakpoint_list.is_empty() {
            Default::default()
        } else {
            let breakpoints = breakpoint_list
                .into_iter()
                .filter_map(|b| Breakpoint::from_expression(&b).ok())
                .collect();
            Self {
                breakpoints,
                ..Default::default()
            }
        }
    }

    pub fn draw<DBG: DebugOperations>(&mut self, ui: &mut Ui, regs: &DBG) {
        ui.vertical(|ui| {
            ui.label(Label::new("Breakpoints").text_color(Color32::LIGHT_BLUE));
            self.draw_breakpoint_options(ui);

            ui.separator();
            if self.options.is_advanced {
                self.draw_advanced_breakpoint_widget(ui, regs);
            } else {
                self.draw_simple_breakpoint_widget(ui, regs);
            }

            let mut deletion_list: Vec<usize> = Vec::with_capacity(20);
            egui::Grid::new("breakpoints_".to_owned())
                .striped(true)
                .spacing(Vec2::new(60.5, 6.5))
                .show(ui, |ui| {
                    ui.label(egui::Label::new("Delete"));
                    ui.label(egui::Label::new("Enabled"));
                    ui.label(egui::Label::new("Condition"));
                    ui.end_row();

                    for (i, breakpoint) in &mut self.breakpoints.iter_mut().enumerate() {
                        if ui
                            .add(egui::Button::new("-").text_color(Color32::RED))
                            .clicked()
                        {
                            deletion_list.push(i)
                        }
                        ui.checkbox(&mut breakpoint.enabled, "");
                        if breakpoint.is_triggered(regs) {
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
        });
    }

    fn add_address_breakpoint<DBG: DebugOperations>(&mut self, address: u16, regs: &DBG) {
        if !self.breakpoints.iter().any(|x| x.is_triggered(regs)) {
            self.breakpoints.push(Breakpoint::from_address(address));
        }
    }

    fn add_expr_breakpoint<DBG: DebugOperations>(
        &mut self,
        expr: &str,
        regs: &DBG,
    ) -> anyhow::Result<()> {
        if !self.breakpoints.iter().any(|x| x.is_triggered(regs)) {
            let breakpoint = Breakpoint::from_expression(expr)?;
            self.breakpoints.push(breakpoint);
        }
        Ok(())
    }

    pub fn are_breakpoints_triggered<DBG: DebugOperations>(&mut self, context: &DBG) -> bool {
        for breakpoint in &mut self.breakpoints {
            if breakpoint.active(context) {
                log::debug!("breakpoint triggered break={:?}", breakpoint);
                return true;
            }
        }
        false
    }

    fn draw_advanced_breakpoint_widget<DBG: DebugOperations>(&mut self, ui: &mut Ui, regs: &DBG) {
        ui.horizontal(|ui| {
            let add_button_response =
                ui.add(egui::Button::new("+").enabled(is_valid_expression(&self.breakpoint_field)));
            let text_field_response = ui.add(
                egui::TextEdit::multiline(&mut self.breakpoint_field)
                    .desired_width(150.0)
                    .hint_text("AF == 0x80"),
            );
            if (add_button_response.clicked()
                || text_field_response.clicked()
                    && ui.input().key_pressed(egui::Key::Enter)
                    && is_valid_expression(&self.breakpoint_field))
                && self
                    .add_expr_breakpoint(&self.breakpoint_field.clone(), regs)
                    .is_err()
            {
                log::error!("Debugger input is invalid")
            }
            if text_field_response.lost_focus() {
                self.breakpoint_field.clear();
                text_field_response.ctx.set_visuals(Visuals::default())
            }
        });
    }

    fn draw_simple_breakpoint_widget<DBG: DebugOperations>(&mut self, ui: &mut Ui, regs: &DBG) {
        self.breakpoint_field.retain(|c| c.is_ascii_hexdigit());
        if self.breakpoint_field.len() <= 5 {
            self.breakpoint_field.truncate(4)
        }
        ui.horizontal(|ui| {
            let add_button_response =
                ui.add(egui::Button::new("+").enabled(is_valid_address(&self.breakpoint_field)));
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
                    && is_valid_address(&self.breakpoint_field)
            {
                self.add_address_breakpoint(
                    u16::from_str_radix(&*self.breakpoint_field, 16).unwrap(),
                    regs,
                );
            }
            if text_field_response.lost_focus() {
                self.breakpoint_field.clear();
            }
        });
    }

    fn draw_breakpoint_options(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(true)
            .show(ui, |ui| {
                ui.checkbox(&mut self.options.is_advanced, "Advanced")
            });
    }
}

fn is_valid_address(address: &str) -> bool {
    !address.is_empty() && u16::from_str_radix(address, 16).is_ok()
}

fn is_valid_expression(expr: &str) -> bool {
    expr.chars().all(|c| {
        c.is_alphanumeric() || c.is_whitespace() || VALID_CHARS.contains(&c)
    }) && !expr.is_empty()
        && expr.split_whitespace().count() == 3
}
