use crate::{custom_event::CustomEvent, windows::Windows};
use gb_lcd::{EventProcessing, PseudoWindow};
use winit::{event::WindowEvent, event_loop::EventLoopProxy, window::WindowId};

pub struct Context {
    pub windows: Windows,
}

impl Context {
    pub fn new(windows: Windows) -> Self {
        Self { windows }
    }

    pub fn redraw(&self, window_id: WindowId) -> anyhow::Result<()> {
        if window_id == self.windows.main.id() {
            self.redraw_main_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn process_window_event(
        &mut self,
        window_id: WindowId,
        event: WindowEvent,
        event_proxy: &EventLoopProxy<CustomEvent>,
    ) {
        if window_id == self.windows.main.id() {
            self.process_main_window_event(event, event_proxy)
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }
}

/// Context impl for main window
impl Context {
    pub fn redraw_main_window(&self) -> anyhow::Result<()> {
        self.windows
            .main
            .pixels
            .render_with(|encoder, render_target, context| {
                // Render pixels buffer
                context.scaling_renderer.render(encoder, render_target);

                Ok(())
            })?;

        Ok(())
    }

    fn process_main_window_event(
        &mut self,
        event: WindowEvent,
        event_proxy: &EventLoopProxy<CustomEvent>,
    ) {
        self.windows.main.process_window_event(event);
        if self.windows.main.closed() {
            event_proxy
                .send_event(CustomEvent::Quit)
                .expect("cannot send quit event");
        }
    }
}
