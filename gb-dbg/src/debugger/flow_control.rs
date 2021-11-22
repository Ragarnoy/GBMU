use crate::until::Until;
use egui::Ui;
use std::ops::ControlFlow;

pub struct FlowController;

impl FlowController {
    pub fn draw(&self, ui: &mut Ui) -> Option<ControlFlow<Until>> {
        let mut ret: Option<ControlFlow<Until>> = None;
        if ui.button("Continue").clicked() {
            log::debug!("clicked on continue");
            ret = Some(ControlFlow::Continue(()));
        }
        if ui.button("Pause").clicked() {
            log::debug!("clicked on pause");
            ret = Some(ControlFlow::Break(Until::Null));
        }
        if ui.button("Step").clicked() {
            log::debug!("clicked on step");
            ret = Some(ControlFlow::Break(Until::Step(1)));
        }
        if ui.button("Run one frame").clicked() {
            log::debug!("clicked on frame");
            ret = Some(ControlFlow::Break(Until::Frame(1)));
        }
        if ui.button("Run one second").clicked() {
            log::debug!("clicked on one second");
            ret = Some(ControlFlow::Break(Until::Second(1)));
        }
        ret
    }
}
