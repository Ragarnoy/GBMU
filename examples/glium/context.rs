use glium::glutin::event_loop::EventLoop;
use glium::{Frame, glutin};
use crate::gb_window::GBWindow;

pub struct Context {
    pub event_loop: EventLoop<()>,
    pub windows: Windows,
}

pub struct Windows {
    pub gbmu: GBWindow,
    pub debugger: Option<GBWindow>,
    pub input: Option<GBWindow>
}

impl Context {
    pub fn new() -> Self {
        let event_loop = glutin::event_loop::EventLoop::new();

        Self {
            windows: Windows {
                gbmu: GBWindow::new("GBMU", (160, 144), &event_loop),
                debugger: None,
                input: None,
            },
            event_loop,
        }
    }

    pub fn gbmu_window(&self) -> &GBWindow {
        &self.windows.gbmu
    }
}
