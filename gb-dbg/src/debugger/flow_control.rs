use crate::until::Until;
use egui::Ui;
use std::ops::ControlFlow;

pub struct FlowController;

impl FlowController {
    pub fn draw(&self, ui: &mut Ui) -> Option<ControlFlow<Until>> {
        let mut ret: Option<ControlFlow<Until>> = None;
        ui.horizontal(|ui| {
            if ui.button("Continue").clicked() {
                log::error!("clicked on step");
                ret = Some(ControlFlow::Continue(()));
            }
            if ui.button("Step").clicked() {
                log::error!("clicked on step");
                ret = Some(ControlFlow::Break(Until::Step(1)));
            }
            if ui.button("Run one frame").clicked() {
                log::error!("clicked on frame");
                ret = Some(ControlFlow::Break(Until::Frame(1)));
            }
            if ui.button("Run one second").clicked() {
                log::error!("clicked on one second");
                ret = Some(ControlFlow::Break(Until::Second(1)));
            }
        });
        ret
    }
}
