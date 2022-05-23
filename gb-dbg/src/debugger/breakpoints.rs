mod breakpoint;
mod evaluation;

use crate::dbg_interfaces::DebugOperations;
use crate::debugger::breakpoints::breakpoint::Breakpoint;

use egui::{Color32, RichText, Ui, Vec2};

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
                .filter_map(|b| {
                    let bp = Breakpoint::from_expression(&b);
                    match bp {
                        Ok(p) => Some(p),
                        Err(e) => {
                            log::error!("invalid breakpoint expression \"{}\": {}", b, e);
                            None
                        }
                    }
                })
                .collect();
            Self {
                breakpoints,
                ..Default::default()
            }
        }
    }

    pub fn draw<DBG: DebugOperations>(&mut self, ui: &mut Ui, regs: &DBG) {
        ui.vertical(|ui| {
            ui.colored_label(Color32::LIGHT_BLUE, "Breakpoints");
            self.draw_breakpoint_options(ui);

            ui.separator();
            if self.options.is_advanced {
                self.draw_advanced_breakpoint_widget(ui);
            } else {
                self.draw_simple_breakpoint_widget(ui);
            }

            let mut deletion_list: Vec<usize> = Vec::with_capacity(20);
            egui::Grid::new("breakpoints_".to_owned())
                .striped(true)
                .spacing(Vec2::new(47.0, 7.0))
                .show(ui, |ui| {
                    ui.label("Delete");
                    ui.label("Enabled");
                    ui.label("Condition");
                    ui.end_row();

                    for (i, breakpoint) in &mut self.breakpoints.iter_mut().enumerate() {
                        if ui.button(RichText::new("-").color(Color32::RED)).clicked() {
                            deletion_list.push(i)
                        }
                        ui.checkbox(&mut breakpoint.enabled, "");
                        if breakpoint.is_triggered(regs) {
                            ui.label(
                                RichText::new(breakpoint.to_string().clone()).color(Color32::RED),
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

    fn add_address_breakpoint(&mut self, address: u16) {
        self.breakpoints.push(Breakpoint::from_address(address));
    }

    fn add_expr_breakpoint(&mut self, expr: &str) -> anyhow::Result<()> {
        let breakpoint = Breakpoint::from_expression(expr)?;
        self.breakpoints.push(breakpoint);
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

    fn draw_advanced_breakpoint_widget(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let add_button_response = ui.add_enabled(
                is_valid_expression(&self.breakpoint_field),
                egui::Button::new("+"),
            );
            let text_field_response = ui.add(
                egui::TextEdit::multiline(&mut self.breakpoint_field)
                    .desired_width(150.0)
                    .hint_text("AF == 0x80"),
            );
            if add_button_response.clicked()
                || (text_field_response.clicked()
                    && is_enter_not_modified(ui)
                    && is_valid_expression(&self.breakpoint_field))
            {
                if let Err(e) = self.add_expr_breakpoint(&self.breakpoint_field.clone()) {
                    log::warn!(
                        "cannot add breakpoint expression \"{}\", because: {}",
                        self.breakpoint_field,
                        e
                    );
                } else {
                    self.breakpoint_field.clear();
                }
            }
        });
    }

    fn draw_simple_breakpoint_widget(&mut self, ui: &mut Ui) {
        self.breakpoint_field.retain(|c| c.is_ascii_hexdigit());
        if self.breakpoint_field.len() <= 5 {
            self.breakpoint_field.truncate(4)
        }
        ui.horizontal(|ui| {
            let add_button_response = ui.add_enabled(
                is_valid_address(&self.breakpoint_field),
                egui::Button::new("+"),
            );
            ui.label(RichText::new("0x").color(Color32::from_gray(90)).weak());
            let text_field_response = ui.add(
                egui::TextEdit::singleline(&mut self.breakpoint_field)
                    .desired_width(85.0)
                    .hint_text("555F"),
            );
            if add_button_response.clicked()
                || (text_field_response.clicked()
                    && ui.input().key_pressed(egui::Key::Enter)
                    && is_valid_address(&self.breakpoint_field))
            {
                match u16::from_str_radix(&self.breakpoint_field, 16) {
                    Ok(v) => self.add_address_breakpoint(v),
                    Err(e) => {
                        log::error!(
                            "cannot address \"{}\" as breakpoint, because: {}",
                            self.breakpoint_field,
                            e
                        );
                        self.breakpoint_field.clear()
                    }
                }
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
    expr.chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || VALID_CHARS.contains(&c))
        && !expr.is_empty()
        && expr.split_whitespace().count() % 2 != 0
}

fn is_enter_not_modified(ui: &Ui) -> bool {
    ui.input().key_pressed(egui::Key::Enter) && ui.input().modifiers.is_none()
}
