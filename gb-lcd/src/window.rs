use egui::{ClippedMesh, CtxRef};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use winit::{event::WindowEvent, window::Window};

use crate::{state::State, EventProcessing, PseudoPixels, PseudoWindow};

pub struct GBWindow {
    pub window: Window,

    pub egui_ctx: CtxRef,
    pub egui_state: egui_winit::State,
    pub screen_descriptor: ScreenDescriptor,
    pub rpass: RenderPass,
    pub paint_jobs: Vec<ClippedMesh>,

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

        let egui_ctx = CtxRef::default();
        let egui_state = egui_winit::State::from_pixels_per_point(scale_factor as f32);
        let screen_descriptor = ScreenDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: scale_factor as f32,
        };

        let rpass = RenderPass::new(&device, surface_format, 1);
        let paint_jobs = Vec::new();

        Self {
            window,

            egui_ctx,
            egui_state,

            screen_descriptor,
            rpass,
            paint_jobs,

            device,
            queue,

            surface_config,
            surface_format,
            surface,

            state: State::default(),
        }
    }
}

impl GBWindow {
    /// Prepare to render egui
    pub fn prepare_egui<F>(&mut self, render: F)
    where
        F: FnOnce(&CtxRef),
    {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let (output, paint_commands) = self.egui_ctx.run(raw_input, render);

        self.egui_state
            .handle_output(&self.window, &self.egui_ctx, output);
        self.paint_jobs = self.egui_ctx.tessellate(paint_commands);
    }

    /// Render egui
    pub fn render_egui(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
    ) -> Result<(), BackendError> {
        self.rpass
            .update_texture(&self.device, &self.queue, &self.egui_ctx.font_image());
        self.rpass.update_user_textures(&self.device, &self.queue);
        self.rpass.update_buffers(
            &self.device,
            &self.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )
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
