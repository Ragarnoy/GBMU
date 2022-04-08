use egui::Context;
use egui_wgpu_backend::ScreenDescriptor;
use winit::{event::WindowEvent, window::Window};

use crate::{state::State, EventProcessing, PseudoPixels, PseudoWindow};

pub struct GBWindow {
    pub window: Window,
    pub egui_ctx: Context,
    pub egui_state: egui_winit::State,
    pub screen_descriptor: ScreenDescriptor,
    pub(crate) state: State,
}

impl GBWindow {
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::from_pixels_per_point(scale_factor as f32);
        let screen_descriptor = ScreenDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: scale_factor as f32,
        };

        Self {
            window,
            egui_ctx,
            egui_state,
            screen_descriptor,
            state: State::default(),
        }
    }
}

impl PseudoWindow for GBWindow {
    fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    fn request_redraw(&self) {
        self.window.request_redraw()
    }
}

impl PseudoPixels for GBWindow {
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.screen_descriptor.physical_height = size.height;
        self.screen_descriptor.physical_width = size.width;
    }
}

impl EventProcessing for GBWindow {
    fn process_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.state.closed = true,
            WindowEvent::Focused(focused) => self.state.focused = focused,
            WindowEvent::CursorMoved { .. }
            | WindowEvent::Moved(_)
            | WindowEvent::ModifiersChanged(_) => log::debug!("ignore event {event:?}"),
            WindowEvent::Resized(new_size) => {
                self.resize(new_size);
                self.request_redraw();
            }
            _ => todo!("process window event {event:?}"),
        }
    }
}
