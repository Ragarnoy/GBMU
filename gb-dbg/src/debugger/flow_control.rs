use crate::run_duration::RunDuration;
use egui::Ui;
use std::ops::ControlFlow;

pub struct FlowController;

impl FlowController {
    pub fn draw(&self, ui: &mut Ui) -> Option<ControlFlow<(), RunDuration>> {
        let mut ret: Option<ControlFlow<(), RunDuration>> = None;
        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                log::error!("clicked on step");
                ret = Some(ControlFlow::Continue(RunDuration::Step));
            }
            if ui.button("Run one frame").clicked() {
                log::error!("clicked on frame");
                ret = Some(ControlFlow::Continue(RunDuration::RunFrame));
            }
            if ui.button("Run one second").clicked() {
                log::error!("clicked on one second");
                ret = Some(ControlFlow::Continue(RunDuration::RunSecond));
            }
        });
        ret
    }
}
