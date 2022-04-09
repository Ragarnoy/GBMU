use crate::{custom_event::CustomEvent, windows::Windows};
use gb_lcd::{DrawEgui, PseudoPixels, PseudoWindow};
use winit::{event::WindowEvent, event_loop::EventLoopProxy, window::WindowId};

pub struct Context {
    pub windows: Windows,
    pub event_proxy: EventLoopProxy<CustomEvent>,
}

impl Context {
    pub fn new(windows: Windows, event_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            windows,
            event_proxy,
        }
    }

    pub fn redraw(&mut self, window_id: WindowId) -> anyhow::Result<()> {
        if window_id == self.windows.main.id() {
            self.redraw_main_window()
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }

    pub fn process_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        if window_id == self.windows.main.id() {
            self.process_main_window_event(event)
        } else {
            panic!("unexpected window id {window_id:?}")
        }
    }
}

/// Context impl for main window
impl Context {
    pub fn redraw_main_window(&mut self) -> anyhow::Result<()> {
        let _ = self.windows.main.pixels.get_frame();

        crate::ui::draw_egui(self);
        let main_pixels = &mut self.windows.main.pixels;
        let main_context = &mut self.windows.main.context;

        main_pixels.render_with(|encoder, render_target, context| {
            // Render pixels buffer
            context.scaling_renderer.render(encoder, render_target);

            main_context.render_egui(
                encoder,
                render_target,
                &gb_lcd::RenderContext::from(context),
            )?;

            Ok(())
        })?;

        Ok(())
    }

    fn process_main_window_event(&mut self, event: WindowEvent) {
        if self.windows.main.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::Resized(new_size) => {
                self.windows.main.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.windows.main.resize(*new_inner_size);
                self.windows.main.context.scale_factor(scale_factor as f32);
            }
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::Quit)
                .expect("cannot send quit event"),
            WindowEvent::DroppedFile(path) => self
                .event_proxy
                .send_event(CustomEvent::LoadFile(path))
                .expect("cannot send load file event"),
            WindowEvent::CursorMoved { .. }
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::CursorLeft { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::AxisMotion { .. }
            | WindowEvent::Moved(_)
            | WindowEvent::Focused(_)
            | WindowEvent::ModifiersChanged(_) => {
                // log::debug!("ignore main window event {event:?}")
            }
            _ => todo!("process main window event {event:?}"),
        }
    }
}
