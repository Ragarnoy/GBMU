// pub mod error;
// pub mod render;
// mod shader;
pub mod pixels;
mod state;
pub mod window;

pub use crate::pixels::GBPixels;
pub use window::GBWindow;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::WindowId};

// use error::Error;
// use sdl2::{video::GLProfile, EventPump, Sdl, VideoSubsystem};

// pub fn init() -> Result<(Sdl, VideoSubsystem, EventPump), Error> {
//     let sdl_context = sdl2::init().map_err(Error::MainSys)?;
//     let video_subsystem = sdl_context.video().map_err(Error::MainSys)?;

//     let event_pump = sdl_context.event_pump().map_err(Error::MainSys)?;

//     let gl_attr = video_subsystem.gl_attr();
//     gl_attr.set_context_profile(GLProfile::Core);
//     // OpenGL 3.3 is the minimum that we will support.
//     gl_attr.set_context_version(3, 3);

//     Ok((sdl_context, video_subsystem, event_pump))
// }

pub trait PseudoWindow {
    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    ///
    /// See the [`dpi`](winit::dpi) module for more information.
    ///
    /// Note that this value can change depending on user action (for example if the window is
    /// moved to another screen); as such, tracking `WindowEvent::ScaleFactorChanged` events is
    /// the most robust way to track the DPI you need to use to draw.
    fn scale_factor(&self) -> f64;

    /// Returns the physical size of the window's client area.
    ///
    /// The client area is the content of the window, excluding the title bar and borders.
    fn inner_size(&self) -> PhysicalSize<u32>;

    /// Returns an identifier unique to the window.
    fn id(&self) -> WindowId;

    /// Request to redraw this window
    fn request_redraw(&self);
}

pub trait PseudoPixels {
    /// Resize the pixels surface
    fn resize_surface(&mut self, size: PhysicalSize<u32>);
}

pub trait EventProcessing {
    /// Add a event to be processed
    fn process_window_event(&mut self, event: WindowEvent);
}
