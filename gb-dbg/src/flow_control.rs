use egui::Ui;

pub struct FlowController;

impl FlowController {
    pub fn draw(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Step").clicked() {
                self.step();
            }
            if ui.button("Run one frame").clicked() {
                self.run_one_frame();
            }
            if ui.button("Run one second").clicked() {
                self.run_one_second();
            }
        });
    }

    pub fn step(&self) {
        todo!()
    }

    pub fn run_one_frame(&self) {
        todo!()
    }

    pub fn run_one_second(&self) {
        todo!()
    }
}
