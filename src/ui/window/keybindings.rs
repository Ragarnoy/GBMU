use egui::{CtxRef, Direction, Layout, Separator, Ui};
use gb_joypad::{input::INPUT_LIST, Config, InputType};

pub fn draw_window(ctx: &CtxRef, config: &mut Config) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let height = ui.available_size().y;
        egui::ScrollArea::vertical()
            .max_height(height - 50.0)
            .show(ui, |ui| {
                ui.set_height(height - 60.0);
                for input_type in INPUT_LIST.iter() {
                    ui.horizontal(|ui| {
                        if let Some(listened) = self.listening {
                            input_row(ui, input_type, &listened == input_type);
                        } else {
                            input_row(ui, input_type, false);
                        }
                    });
                }
            });
        ui.vertical(|ui| {
            ui.vertical_centered(|ui| {
                ui.add(Separator::default().horizontal().spacing(30.0));
                if ui.button("reset   ⟲").clicked() {
                    self.listening = None;
                    *config = Config::default();
                }
            });
        });
    });
}

fn input_row(ui: &mut Ui, input_type: &InputType, force_empty: bool) {
    ui.columns(3, |ui| {
        ui[0].label(format!("{:?}:", input_type));
        ui[1].with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if force_empty {
                    ui.label("---");
                } else {
                    match self
                        .input_map
                        .iter()
                        .find(|(_, map_val)| &input_type == map_val)
                    {
                        Some((code, _)) => ui.label(code.name()),
                        None => ui.label("---"),
                    };
                }
            },
        );
        ui[2].with_layout(Layout::right_to_left(), |ui| {
            if !force_empty && ui.button("⚙").clicked() {
                self.listening = Some(*input_type);
            } else if force_empty && ui.button("❌").clicked() {
                self.listening = None;
            }
        });
    });
}
