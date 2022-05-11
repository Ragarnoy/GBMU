use gb_lcd::GBPixels;
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{custom_event::CustomEvent, image::load_image_to_frame, windows::WindowType};
use gb_lcd::{DrawEgui, PseudoPixels};
use gb_ppu::Ppu;

#[derive(Clone, Copy)]
pub enum ToolType {
    Tilesheet,
    Tilemap { window: bool },
    Spritesheet,
}

impl From<ToolType> for WindowType {
    fn from(tool_type: ToolType) -> WindowType {
        match tool_type {
            ToolType::Tilesheet => WindowType::Tilesheet,
            ToolType::Tilemap { window: _ } => WindowType::Tilemap,
            ToolType::Spritesheet => WindowType::Spritesheet,
        }
    }
}

pub struct Context<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32> {
    pub window: GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>,
    event_proxy: EventLoopProxy<CustomEvent>,
    tool_type: ToolType,
}

impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32>
    Context<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
    pub fn new(
        window: GBPixels<WIDTH, HEIGHT, MENU_BAR_SIZE>,
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
impl<const WIDTH: u32, const HEIGHT: u32, const MENU_BAR_SIZE: u32>
    Context<WIDTH, HEIGHT, MENU_BAR_SIZE>
{
    pub(crate) fn redraw_window(&mut self, ppu: &Ppu) -> anyhow::Result<()> {
        let window = &mut self.window;
        let (size, margin) = window.texture_size_and_margin();

        let pixels = &mut window.pixels;
        let pixels_context = &mut window.context;

        pixels_context.prepare_egui(&window.window, |egui_ctx| {
            let mut top_frame = egui::Frame::menu(&egui::style::Style::default());
            top_frame.margin = egui::style::Margin::symmetric(5.0, 0.0);
            egui::containers::TopBottomPanel::top("Top menu")
                .frame(top_frame)
                .show(egui_ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.set_height(crate::constant::MENU_BAR_SIZE - 1.0);
                        match &mut self.tool_type {
                            ToolType::Tilesheet => {}
                            ToolType::Tilemap { ref mut window } => {
                                ui.checkbox(window, "window");
                            }

                            ToolType::Spritesheet => {}
                        };
                    });
                });
            let mut central_frame = egui::Frame::none();
            central_frame.margin = egui::style::Margin::symmetric(margin.0, margin.1);
            central_frame.margin.top += 1.0;
            egui::containers::CentralPanel::default()
                .frame(central_frame)
                .show(egui_ctx, |ui| {
                    ui.image(window.texture_id, size);
                });
        });

        match self.tool_type {
            ToolType::Tilesheet => {
                let image = ppu.tilesheet_image();
                let frame = pixels.get_frame();
                load_image_to_frame(&image, frame);
            }
            ToolType::Tilemap { window } => {
                let image = ppu.tilemap_image(window);
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
