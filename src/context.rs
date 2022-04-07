use crate::windows::Windows;
use egui::Window;
use gb_lcd::PseudoWindow;
use winit::window::WindowId;

pub struct Context {
    pub windows: Windows,
}

impl Context {
    pub fn new(windows: Windows) -> Self {
        Self { windows }
    }

    pub fn redraw(&self, window_id: WindowId) {
        if window_id == self.windows.main.id() {
            self.redraw_main_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn redraw_main_window(&self) {
        todo!("redraw main window")
    }
}
