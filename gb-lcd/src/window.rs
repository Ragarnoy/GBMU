use winit::{event::WindowEvent, window::Window};

use crate::{
    context::Context, state::State, DrawEgui, EventProcessing, PseudoPixels, PseudoWindow,
    RenderContext,
};

pub struct GBWindow {
    pub window: Window,

    pub context: Context,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
    pub surface: wgpu::Surface,

    pub(crate) state: State,
}

impl GBWindow {
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: Some("basic_egui_window"),
            },
            None,
        ))
        .unwrap();

        let surface_format = surface.get_preferred_format(&adapter).unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let context = Context::new(&device, surface_format, scale_factor as f32, size);

        Self {
            window,

            context,

            device,
            queue,

            surface_config,
            surface_format,
            surface,

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
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.context.resize(size);
    }

    fn render_with<F>(&mut self, render_function: F) -> anyhow::Result<()>
    where
        F: FnOnce(
            &mut wgpu::CommandEncoder,
            &wgpu::TextureView,
            &RenderContext,
        ) -> anyhow::Result<()>,
    {
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("gb_window_encoder"),
            });

        let render_target = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let context = RenderContext::new(&self.device, &self.queue);

        render_function(&mut encoder, &render_target, &context).map_err(anyhow::Error::from)?;
        self.context
            .render_egui(&mut encoder, &render_target, &context)?;

        self.queue.submit(std::iter::once(encoder.finish()));
        output_frame.present();
        Ok(())
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
