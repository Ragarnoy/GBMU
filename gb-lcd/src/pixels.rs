use pixels::{Error, Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};
use winit_input_helper::WinitInputHelper;

use crate::{state::State, EventProcessing, GBWindow, PseudoPixels, PseudoWindow};

pub struct GBPixels {
    pub window: Window,
    pub input: WinitInputHelper,
    pub pixels: Pixels,
    state: State,
}

impl GBPixels {
    pub fn new(window: Window, input: WinitInputHelper, pixels: Pixels) -> Self {
        Self {
            window,
            input,
            pixels,
            state: State::default(),
        }
    }

    pub fn from_window<const WIDTH: u32, const HEIGHT: u32>(
        window: GBWindow,
    ) -> Result<Self, Error> {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window.window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        Ok(Self::new(window.window, window.input, pixels))
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
    fn resize_surface(&mut self, size: PhysicalSize<u32>) {
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
                self.resize_surface(new_size);
                self.request_redraw();
            }
            _ => todo!("process window event {event:?}"),
        }
    }
}
