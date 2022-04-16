use pixels::{Error, Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{
    context::Context, state::State, EventProcessing, PseudoPixels, PseudoWindow, RenderContext,
};

pub struct GBPixels {
    pub window: Window,
    pub pixels: Pixels,

    pub context: Context,

    pub(crate) state: State,
}

impl GBPixels {
    pub fn new<const WIDTH: u32, const HEIGHT: u32>(window: Window) -> Result<Self, Error> {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let pixels = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        let context = Context::new(
            pixels.device(),
            pixels.render_texture_format(),
            scale_factor as f32,
            size,
        );

        let state = State::default();

        Ok(Self {
            window,
            pixels,

            context,

            state,
        })
    }

    /// When the window is requested to closed
    pub fn closed(&self) -> bool {
        self.state.closed
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
        self.context.resize(size);
        self.pixels.resize_surface(size.width, size.height)
    }

    fn render_with<F>(&mut self, render_function: F) -> anyhow::Result<()>
    where
        F: FnOnce(
            &mut wgpu::CommandEncoder,
            &wgpu::TextureView,
            &RenderContext,
        ) -> anyhow::Result<()>,
    {
        self.pixels
            .render_with(|encoder, render_target, context| {
                render_function(encoder, render_target, &RenderContext::from(context))?;
                Ok(())
            })
            .map_err(|err| anyhow::Error::from(err))
    }
}

impl EventProcessing for GBPixels {
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
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.resize(*new_inner_size);
                self.context.scale_factor(scale_factor as f32);
            }
            _ => todo!("process window event {event:?}"),
        }
    }
}
