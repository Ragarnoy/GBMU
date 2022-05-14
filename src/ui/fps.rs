use egui::Ui;

use crate::time_frame::TimeStat;

pub const FPS_WIDTH: f32 = 50.0;

pub fn draw_ui(ui: &mut Ui, time_stat: &TimeStat) {
    // ui.add_space(ui.available_width() - FPS_WIDTH);
    ui.label(format!(
        "min: {min}ms mean: {mean}ms max: {max}ms current: {current}ms",
        min = time_stat.min.as_millis(),
        max = time_stat.max.as_millis(),
        mean = time_stat.mean().as_millis(),
        current = time_stat.last_value.as_millis()
    ));
}
