use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::PseudoWindow;

pub struct GBWindow {
    pub window: Window,
    pub input: WinitInputHelper,
}

impl GBWindow {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            input: WinitInputHelper::new(),
        }
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
