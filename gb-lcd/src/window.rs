use crate::error::Error;
use egui::{CtxRef, Painter};
use egui_sdl2_gl::{DpiScaling, EguiStateHandler};
use sdl2::{
    event::Event,
    video::{GLContext, Window as SdlWindow},
    Sdl, VideoSubsystem,
};
use std::time::Instant;

const RESOLUTION_DOT: f32 = 96.0;

pub struct GBWindow {
    sdl_window: SdlWindow,
    gl_ctx: GLContext,
    egui_ctx: CtxRef,
    egui_painter: Painter,
    egui_state: EguiStateHandler,
    pixels_per_point: f32,
    start_time: Instant,
    #[cfg(feature = "debug_render")]
    debug: bool,
}

impl GBWindow {
    pub fn new(
        title: &str,
        dim: (u32, u32),
        resizable: bool,
        video_sys: &VideoSubsystem,
    ) -> Result<Self, Error> {
        let mut builder = video_sys.window(title, dim.0, dim.1);
        builder.opengl();
        if resizable {
            builder.resizable();
        }
        let sdl_window = builder
            .build()
            .map_err(|err| Error::GBWindowInit(err.to_string()))?;

        let gl_ctx = sdl_window
            .gl_create_context()
            .map_err(Error::GBWindowInit)?;

        let (egui_painter, egui_state) = egui_sdl2_gl::with_sdl2(&sdl_window, DpiScaling::Default);
        let egui_ctx = CtxRef::default();

        let native_pixels_per_point =
            RESOLUTION_DOT / video_sys.display_dpi(0).map_err(Error::GBWindowInit)?.0;

        let start_time = Instant::now();
        Ok(Self {
            sdl_window,
            gl_ctx,
            egui_painter,
            egui_ctx,
            egui_state,
            pixels_per_point: native_pixels_per_point,
            start_time,
            #[cfg(feature = "debug_render")]
            debug: false,
        })
    }

    #[allow(dead_code)]
    pub fn sdl_window(&self) -> &SdlWindow {
        &self.sdl_window
    }
    #[allow(dead_code)]
    pub fn sdl_window_mut(&mut self) -> &mut SdlWindow {
        &mut self.sdl_window
    }
    #[allow(dead_code)]
    pub fn egui_ctx(&self) -> &CtxRef {
        &self.egui_ctx
    }
    #[allow(dead_code)]
    pub fn egui_ctx_mut(&mut self) -> &mut CtxRef {
        &mut self.egui_ctx
    }

    pub fn start_frame(&mut self) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(Error::GBWindowFrame)?;
        self.egui_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.egui_ctx.begin_frame(self.egui_state.input.take());
        self.egui_state.input.pixels_per_point = Some(self.pixels_per_point);
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            #[cfg(feature = "debug_render")]
            if self.debug {
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            }
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(Error::GBWindowFrame)?;
        let (_egui_output, paint_cmds) = self.egui_ctx.end_frame();

        let paint_jobs = self.egui_ctx.tessellate(paint_cmds);

        self.egui_painter
            .paint_jobs(None, paint_jobs, &self.egui_ctx.texture());

        self.sdl_window.gl_swap_window();
        Ok(())
    }

    pub fn resize(&mut self) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(Error::GBWindowFrame)?;
        let (egui_painter, egui_state) =
            egui_sdl2_gl::with_sdl2(&self.sdl_window, DpiScaling::Default);
        self.egui_painter = egui_painter;
        self.egui_state = egui_state;
        Ok(())
    }

    pub fn send_event(&mut self, event: &Event, sdl_context: &Sdl) -> bool {
        if let (Some(id_mouse_focus), Some(id_keyboard_focus)) = (
            sdl_context.mouse().focused_window_id(),
            sdl_context.keyboard().focused_window_id(),
        ) {
            let id = self.sdl_window.id();
            if id_mouse_focus == id || id_keyboard_focus == id {
                self.egui_state.process_input(
                    &self.sdl_window,
                    event.clone(),
                    &mut self.egui_painter,
                );
                return true;
            }
        }
        false
    }

    #[cfg(feature = "debug_render")]
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn dots_to_pixels(video_sys: &VideoSubsystem, dots: f32) -> Result<u32, Error> {
        Ok(
            (dots * RESOLUTION_DOT / video_sys.display_dpi(0).map_err(Error::GBWindowInit)?.0)
                .ceil() as u32
                + 4,
        )
    }
}
