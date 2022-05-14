use egui::{Layout, RichText, Ui};
use std::time::Duration;

use crate::time_frame::TimeStat;

pub const FPS_WIDTH: f32 = 50.0;
pub const ONE_SEC_NANO: f32 = 1_000_000_000.0;

pub fn draw_ui(ui: &mut Ui, time_stat: &TimeStat) {
    let current = time_stat.last();
    let it_time_stat = time_stat.iter();
    let count = it_time_stat.clone().count();
    let sum = it_time_stat.clone().sum::<Duration>();
    let min = it_time_stat.clone().min().unwrap_or(&Duration::ZERO);
    let max = it_time_stat.max().unwrap_or(&Duration::ZERO);
    let mean = sum / (count as u32);

    ui.with_layout(Layout::right_to_left(), |ui| {
        let fps = format!(
            "min: {min:.2} mean: {mean:.2} max: {max:.2} current: {current:.2} (ms)",
            min = min.as_micros() as f32 / 1000_f32,
            max = max.as_micros() as f32 / 1000_f32,
            mean = mean.as_micros() as f32 / 1000_f32,
            current = current.as_micros() as f32 / 1000_f32
        );
        let fps = format!(
            "min: {min:.1} mean: {mean:.1} max: {max:.1} current: {current:.1} (fps)",
            // We need to invert mix and max to get the correct MinMax for fps
            min = ONE_SEC_NANO / max.as_nanos() as f32,
            mean = ONE_SEC_NANO / mean.as_nanos() as f32,
            max = ONE_SEC_NANO / min.as_nanos() as f32,
            current = ONE_SEC_NANO / current.as_nanos() as f32,
        );
        let fps = format!("{:.2}ms", mean.as_micros() as f32 / 1000_f32);
        let fps = format!("{:.1}fps", ONE_SEC_NANO / current.as_nanos() as f32);

        ui.label(RichText::new(fps).monospace().size(8_f32))
    });
}
