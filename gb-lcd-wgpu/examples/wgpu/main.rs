use log::error;
use crate::gui::Gui;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;
const INITIAL_WIDTH: u32 = 160;
const INITIAL_HEIGHT: u32 = 144;
const ERROR_MARGIN: f64 = 0.00001;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(INITIAL_WIDTH as f64, INITIAL_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + egui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(true)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(INITIAL_WIDTH, INITIAL_HEIGHT, surface_texture)?;
        let gui = Gui::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, gui)
    };
    let mut menubar_height = 0.0;
    let mut scale_factor = 0.0;

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        gui.handle_event(&event);

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {

            // Prepare egui
            let new_menubar_height = gui.prepare(&window);
            if (new_menubar_height - menubar_height).abs() > f32::EPSILON {
                menubar_height = new_menubar_height;

                println!("New menubar height: {}", menubar_height);

                // You should probably set your window size to account for the menubar height.
                // In this example, we only adjust the minimum size, and allow the user to
                // resize the window however they want.
                let size = LogicalSize::new(INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32 + menubar_height);
                window.set_inner_size(size);
                window.set_min_inner_size(Some(size));
            }

            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render egui
                gui.render(encoder, render_target, context)?;

                Ok(())
            });

            // Basic error handling
            if render_result
                .map_err(|e| error!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(new_scale_factor) = input.scale_factor() {
                if (new_scale_factor - scale_factor).abs() > ERROR_MARGIN {
                    scale_factor = new_scale_factor;
                    gui.scale_factor(new_scale_factor);
                }
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if size.height > 0 && size.width > 0 {
                    pixels.resize_buffer(size.width, size.height);
                    pixels.resize_surface(size.width, size.height);
                    gui.resize(size.width, size.height);
                }
            }

            // Update internal state and request a redraw
            let frame = pixels.get_frame();
            frame.iter_mut().enumerate().for_each(|(e, pixel) | *pixel = e.overflowing_add(*pixel as usize).0 as u8);
            window.request_redraw();
        }
    })
}
