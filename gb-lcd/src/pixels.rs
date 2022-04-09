use egui::{ClippedMesh, CtxRef};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::{state::State, DrawEgui, EventProcessing, PseudoPixels, PseudoWindow};

pub struct GBPixels {
    pub window: Window,
    pub pixels: Pixels,

    pub egui_ctx: CtxRef,
    pub egui_state: egui_winit::State,
    pub rpass: RenderPass,
    pub screen_descriptor: ScreenDescriptor,
    pub paint_jobs: Vec<ClippedMesh>,

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

        let egui_ctx = CtxRef::default();
        let egui_state = egui_winit::State::from_pixels_per_point(scale_factor as f32);
        let screen_descriptor = ScreenDescriptor {
            physical_height: size.height,
            physical_width: size.width,
            scale_factor: scale_factor as f32,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);

        let state = State::default();

        Ok(Self {
            window,
            pixels,

            egui_ctx,
            egui_state,
            rpass,
            screen_descriptor,
            paint_jobs: Vec::new(),

            state,
        })
    }

    /// When the window is requested to closed
    pub fn closed(&self) -> bool {
        self.state.closed
    }
}

impl DrawEgui for GBPixels {
    fn prepare_egui<F>(&mut self, render: F)
    where
        F: FnOnce(&CtxRef),
    {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let (output, paint_commands) = self.egui_ctx.run(raw_input, render);

        self.egui_state
            .handle_output(&self.window, &self.egui_ctx, output);
        self.paint_jobs = self.egui_ctx.tessellate(paint_commands);
    }

    fn render_egui(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
    ) -> Result<(), BackendError> {
        let context = self.pixels.context();

        self.rpass
            .update_texture(&context.device, &context.queue, &self.egui_ctx.font_image());
        self.rpass
            .update_user_textures(&context.device, &context.queue);
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
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
        self.screen_descriptor.physical_width = size.width;
        self.screen_descriptor.physical_height = size.height;
        self.pixels.resize_surface(size.width, size.height)
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
                self.screen_descriptor.scale_factor = scale_factor as f32;
            }
            _ => todo!("process window event {event:?}"),
        }
    }
}
