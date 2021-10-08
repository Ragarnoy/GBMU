use egui_glium::EguiGlium;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::{glutin, Display, Frame};
use std::error::Error;

pub struct GBWindow {
    pub display: Display,
    pub egui: EguiGlium,
}

impl GBWindow {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32), event_loop: &EventLoop<()>) -> Self {
        let display = glium::Display::new(
            glutin::window::WindowBuilder::new()
                .with_title(title)
                .with_inner_size(LogicalSize::new(size.0, size.1)),
            glutin::ContextBuilder::new(),
            event_loop,
        )
        .unwrap();

        let egui = egui_glium::EguiGlium::new(&display);

        Self { display, egui }
    }

    pub fn start_frame(&mut self) -> Result<Frame, Box<dyn Error>> {
        let frame = self.display.draw();
        self.egui.begin_frame(&self.display);
        Ok(frame)
    }

    pub fn end_frame(&mut self, frame: Frame) -> Result<(), Box<dyn Error>> {
        frame.finish()?;
        self.egui.end_frame(&self.display);
        Ok(())
    }
}
