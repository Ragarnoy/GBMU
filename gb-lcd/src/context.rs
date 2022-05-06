use egui::{ClippedMesh, Context as CtxRef, TexturesDelta};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use winit::{dpi::PhysicalSize, window::Window};

use crate::DrawEgui;

pub struct Context {
    pub egui_ctx: CtxRef,
    pub egui_state: egui_winit::State,
    pub screen_descriptor: ScreenDescriptor,
    pub rpass: RenderPass,
    pub paint_jobs: Vec<ClippedMesh>,
    pub textures_delta: Option<TexturesDelta>
}

impl Context {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        scale_factor: f32,
        size: PhysicalSize<u32>,
    ) -> Self {
        let egui_ctx = CtxRef::default();
        let egui_state = egui_winit::State::from_pixels_per_point((1024 * 1024) as usize, scale_factor);

        let screen_descriptor = ScreenDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor,
        };
        let rpass = RenderPass::new(device, texture_format, 1);

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            textures_delta: None
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.height > 0 && size.width > 0 {
            self.screen_descriptor.physical_width = size.width;
            self.screen_descriptor.physical_height = size.height;
        }
    }

    pub fn scale_factor(&mut self, scale_factor: f32) {
        self.screen_descriptor.scale_factor = scale_factor;
    }
}

impl DrawEgui for Context {
    fn prepare_egui<F>(&mut self, window: &Window, render: F)
    where
        F: FnOnce(&CtxRef),
    {
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, render);

        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.textures_delta = Some(output.textures_delta);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    fn render_egui(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &crate::RenderContext,
    ) -> Result<(), egui_wgpu_backend::BackendError> {
        let texture_delta = self.textures_delta.take().unwrap();
        self.rpass.add_textures(context.device, context.queue, &texture_delta)?;
        self.rpass.update_buffers(
            context.device,
            context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )?;
        self.rpass.remove_textures(texture_delta)
    }

    fn on_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.egui_state.on_event(&self.egui_ctx, event)
    }
}
