use crate::dbg_interfaces::DebugOperations;
use crate::until::Until;
use egui::Ui;
use std::ops::ControlFlow;

pub struct FlowController;
const BIOS_FIRST_INSTRUCTION_MCYCLES: usize = 4;
impl FlowController {
    pub fn draw<DBGOPS: DebugOperations>(
        &self,
        ui: &mut Ui,
        game_ctx: &mut DBGOPS,
    ) -> Option<ControlFlow<Until>> {
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
        if ui.button("Run instruction").clicked() {
            log::debug!("clicked on instruction");

            let current_opcode = game_ctx.current_opcode();
            if current_opcode.is_empty() {
                ret = Some(ControlFlow::Break(Until::Step(
                    BIOS_FIRST_INSTRUCTION_MCYCLES,
                )));
            } else {
                ret = Some(ControlFlow::Break(Until::Instruction(current_opcode)));
            }
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
