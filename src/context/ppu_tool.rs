use gb_lcd::GBPixels;
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{custom_event::CustomEvent, image::load_image_to_frame, windows::WindowType};
use gb_lcd::{DrawEgui, PseudoPixels};
use gb_ppu::Ppu;

#[derive(Clone, Copy)]
pub enum ToolType {
    Tilesheet,
    Tilemap,
    Spritesheet,
}

impl From<ToolType> for WindowType {
    fn from(tool_type: ToolType) -> WindowType {
        match tool_type {
            ToolType::Tilesheet => WindowType::Tilesheet,
            ToolType::Tilemap => WindowType::Tilemap,
            ToolType::Spritesheet => WindowType::Spritesheet,
        }
    }
}

pub struct Context {
    pub window: GBPixels,
    event_proxy: EventLoopProxy<CustomEvent>,
    tool_type: ToolType,
}

impl Context {
    pub fn new(
        window: GBPixels,
        event_proxy: EventLoopProxy<CustomEvent>,
        tool_type: ToolType,
    ) -> Self {
        Self {
            window,
            event_proxy,
            tool_type,
        }
    }
}

/// Context impl for keybindings window
impl Context {
    pub(crate) fn redraw_window(&mut self, ppu: &Ppu) -> anyhow::Result<()> {
        let window = &mut self.window;

        let pixels = &mut window.pixels;
        let pixels_context = &mut window.context;

        pixels_context.prepare_egui(&window.window, |_ctx| {});

        match self.tool_type {
            ToolType::Tilesheet => {
                let image = ppu.tilesheet_image();
                let frame = pixels.get_frame();
                load_image_to_frame(&image, frame);
            }
            ToolType::Tilemap => {
                let image = ppu.tilemap_image(false);
                let frame = pixels.get_frame();
                load_image_to_frame(&image, frame);
            }
            ToolType::Spritesheet => {
                let image = ppu.sprites_image(false);
                let frame = pixels.get_frame();
                load_image_to_frame(&image, frame);
            }
        }

        pixels
            .render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);
                pixels_context.render_egui(
                    encoder,
                    render_target,
                    &gb_lcd::RenderContext::from(context),
                )?;

                Ok(())
            })
            .map_err(anyhow::Error::from)
    }

    pub(crate) fn process_window_event(&mut self, event: WindowEvent) {
        let window = &mut self.window;
        if window.context.on_event(&event) {
            return;
        }

        match event {
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                window.context.scale_factor(scale_factor as f32);
                window.resize(*new_inner_size);
            }
            WindowEvent::Resized(size) => window.resize(size),
            WindowEvent::CloseRequested => self
                .event_proxy
                .send_event(CustomEvent::CloseWindow(self.tool_type.into()))
                .unwrap(),
            _ => {}
        }
    }
}
