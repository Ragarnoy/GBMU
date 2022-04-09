use egui::{ClippedMesh, CtxRef};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use winit::{dpi::PhysicalSize, window::Window};

use crate::DrawEgui;

pub struct Context {
    pub egui_ctx: CtxRef,
    pub egui_state: egui_winit::State,
    pub screen_descriptor: ScreenDescriptor,
    pub rpass: RenderPass,
    pub paint_jobs: Vec<ClippedMesh>,
}

impl Context {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        scale_factor: f32,
        size: PhysicalSize<u32>,
    ) -> Self {
        let egui_ctx = CtxRef::default();
        let egui_state = egui_winit::State::from_pixels_per_point(scale_factor);

        let screen_descriptor = ScreenDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: scale_factor,
        };
        let rpass = RenderPass::new(device, texture_format, 1);

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.screen_descriptor.physical_width = size.width;
        self.screen_descriptor.physical_height = size.height;
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
        let (output, paint_commands) = self.egui_ctx.run(raw_input, render);

        self.egui_state
            .handle_output(window, &self.egui_ctx, output);
        self.paint_jobs = self.egui_ctx.tessellate(paint_commands);
    }

    fn render_egui(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &crate::RenderContext,
    ) -> Result<(), egui_wgpu_backend::BackendError> {
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
