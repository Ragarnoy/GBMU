use gb_joypad::Joypad;
use gb_lcd::render::RenderImage;
#[cfg(feature = "audio")]
use sdl2::audio::AudioQueue;

use std::{cell::RefCell, rc::Rc};

use crate::custom_event::CustomEvent;

pub struct Context<const WIDTH: usize, const HEIGHT: usize> {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub windows: crate::windows::Windows,
    pub display: RenderImage<WIDTH, HEIGHT>,
    pub joypad: Rc<RefCell<Joypad>>,
    #[cfg(feature = "audio")]
    pub audio_queue: Rc<RefCell<AudioQueue<f32>>>,
    #[cfg(feature = "debug_render")]
    pub debug_render: bool,
    pub custom_events: Vec<CustomEvent>,
}
