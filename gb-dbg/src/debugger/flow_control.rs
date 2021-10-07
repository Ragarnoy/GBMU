use crate::run_duration::RunDuration;
use egui::Ui;
use std::ops::ControlFlow;

pub struct FlowController;

impl FlowController {
    pub fn draw(&self, ui: &mut Ui) -> Option<ControlFlow<(), RunDuration>> {
        let mut ret: Option<ControlFlow<(), RunDuration>> = None;
        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                ret = Some(ControlFlow::Continue(RunDuration::Step));
            }
            if ui.button("Run one frame").clicked() {
                ret = Some(ControlFlow::Continue(RunDuration::RunFrame));
            }
            if ui.button("Run one second").clicked() {
                ret = Some(ControlFlow::Continue(RunDuration::RunSecond));
            }
        });
        ret
    }
}
