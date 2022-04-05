use pixels::{Error, Pixels, SurfaceTexture};
use winit::{event::WindowEvent, window::Window};
use winit_input_helper::WinitInputHelper;

use crate::{EventProcessing, GBWindow, PseudoWindow};

pub struct GBPixels {
    pub window: Window,
    pub input: WinitInputHelper,
    pub pixels: Pixels,
}

impl GBPixels {
    pub fn new(window: Window, pixels: Pixels) -> Self {
        Self {
            window,
            input: WinitInputHelper::new(),
            pixels,
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

        Ok(Self {
            window: window.window,
            input: window.input,
            pixels,
        })
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
}

impl EventProcessing for GBPixels {
    fn process_window_event(&self, event: WindowEvent) {
        match event {
            _ => todo!("process window event {event:?}"),
        }
    }
}
