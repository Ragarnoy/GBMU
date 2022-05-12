use pixels::{Error, Pixels, PixelsBuilder, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{
    context::Context, state::State, EventProcessing, PseudoPixels, PseudoWindow, RenderContext,
};

pub struct GBPixels<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32> {
    pub window: Window,
    pub pixels: Pixels,
    pub texture_id: egui::TextureId,

    pub context: Context,

    pub(crate) state: State,
}

impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32>
    GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
    pub fn new(window: Window) -> Result<Self, Error> {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let pixels = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
                .enable_vsync(false)
                .build()?
        };

        let mut context = Context::new(
            pixels.device(),
            pixels.render_texture_format(),
            scale_factor as f32,
            size,
        );

        let state = State::default();

        let texture_id = context.rpass.egui_texture_from_wgpu_texture(
            pixels.device(),
            &pixels.texture().create_view(&wgpu::TextureViewDescriptor {
                ..Default::default()
            }),
            wgpu::FilterMode::Nearest,
        );

        Ok(Self {
            window,
            pixels,
            texture_id,

            context,

            state,
        })
    }

    /// When the window is requested to closed
    pub fn closed(&self) -> bool {
        self.state.closed
    }

    pub fn texture_size_and_margin(&self) -> ((f32, f32), (f32, f32)) {
        let screen_ratio = WIDTH as f32 / HEIGHT as f32;
        let scale_factor = self.window.scale_factor() as f32;

        let actual_dim: (f32, f32) = self.window.inner_size().into();
        let actual_width = actual_dim.0 / scale_factor;
        let actual_height = actual_dim.1 / scale_factor - MENU_BAR_SIZE as f32;
        let actual_ratio = actual_width / actual_height;

        let mut margin = (0.0, 0.0);
        let target_dim = if screen_ratio > actual_ratio {
            let new_height = actual_width / screen_ratio;
            margin.1 = ((actual_height - new_height) / 2.0).round();
            (actual_width, new_height)
        } else {
            let new_width = actual_height * screen_ratio;
            margin.0 = ((actual_width - new_width) / 2.0).round();
            (new_width, actual_height)
        };
        (target_dim, margin)
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32> PseudoWindow
    for GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
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

impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32> PseudoPixels
    for GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
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
            .map_err(anyhow::Error::from)
    }
}

impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32> EventProcessing
    for GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
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
