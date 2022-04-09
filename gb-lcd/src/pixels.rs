use egui::CtxRef;
use egui_wgpu_backend::BackendError;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{EventProcessing, GBWindow, PseudoPixels, PseudoWindow};

pub struct GBPixels {
    pub window: GBWindow,
    pub pixels: Pixels,
}

impl GBPixels {
    pub fn new(window: Window, pixels: Pixels) -> Self {
        Self {
            window: GBWindow::new(window),
            pixels,
        }
    }

    pub fn from_window<const WIDTH: u32, const HEIGHT: u32>(
        window: GBWindow,
    ) -> Result<Self, Error> {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window.window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        Ok(Self {
            window: window,
            pixels,
        })
    }

    /// When the window is requested to closed
    pub fn closed(&self) -> bool {
        self.window.state.closed
    }
}

impl GBPixels {
    pub fn prepare_egui<F>(&mut self, render: F)
    where
        F: FnOnce(&CtxRef),
    {
        self.window.prepare_egui(render)
    }

    pub fn render_egui(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
    ) -> Result<(), BackendError> {
        let context = self.pixels.context();

        self.window.rpass.update_texture(
            &context.device,
            &context.queue,
            &self.window.egui_ctx.font_image(),
        );
        self.window
            .rpass
            .update_user_textures(&context.device, &context.queue);
        self.window.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.window.paint_jobs,
            &self.window.screen_descriptor,
        );

        self.window.rpass.execute(
            encoder,
            render_target,
            &self.window.paint_jobs,
            &self.window.screen_descriptor,
            None,
        )
    }
}

impl PseudoWindow for GBPixels {
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
        self.window.request_redraw();
    }
}

impl PseudoPixels for GBPixels {
    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.window.resize(size);
        self.pixels.resize_surface(size.width, size.height)
    }
}

impl EventProcessing for GBPixels {
    fn process_window_event(&mut self, event: WindowEvent) {
        if let WindowEvent::Resized(new_size) = event {
            self.resize(new_size);
        } else {
            self.window.process_window_event(event);
        }
    }
}
