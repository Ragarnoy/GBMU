pub mod error;
pub mod render;
pub mod window;

use error::Error;
use sdl2::{video::GLProfile, EventPump, Sdl, VideoSubsystem};

pub fn init<'a>() -> Result<(Sdl, VideoSubsystem, EventPump), Error> {
    let sdl_context = sdl2::init().map_err(|err| Error::MainSys(err))?;
    let video_subsystem = sdl_context.video().map_err(|err| Error::MainSys(err))?;

    let event_pump = sdl_context
        .event_pump()
        .map_err(|err| Error::MainSys(err))?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    // OpenGL 3.3 is the minimum that we will support.
    gl_attr.set_context_version(3, 3);

    return Ok((sdl_context, video_subsystem, event_pump));
}
