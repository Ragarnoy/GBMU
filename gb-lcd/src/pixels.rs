use pixels::{Error, Pixels, SurfaceTexture};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::{GBWindow, PseudoWindow};

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
            let scale_factor = window.scale_factor();
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
}
