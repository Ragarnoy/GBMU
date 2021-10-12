use crate::gb_window::GBWindow;
use glium::glutin::event_loop::EventLoop;
use glium::{glutin, Frame};

pub struct Context {
    pub gbmu: GBWindow,
    pub debugger: Option<GBWindow>,
    pub input: Option<GBWindow>,
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        Self {
            gbmu: GBWindow::new("GBMU", (160, 144), event_loop),
            debugger: None,
            input: None,
        }
    }

    pub fn gbmu_window(&self) -> &GBWindow {
        &self.gbmu
    }

    pub fn gbmu_window_mut(&mut self) -> &mut GBWindow {
        &mut self.gbmu
    }
}
