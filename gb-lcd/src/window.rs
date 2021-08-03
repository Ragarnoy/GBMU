use crate::error::Error;
use egui::{vec2, CtxRef, Pos2, Rect};
use egui_sdl2_gl::{EguiInputState, Painter};
use sdl2::{
    event::Event,
    video::{GLContext, Window as SdlWindow},
    Sdl, VideoSubsystem,
};
use std::time::Instant;

pub struct GBWindow {
    sdl_window: SdlWindow,
    gl_ctx: GLContext,
    egui_ctx: CtxRef,
    egui_painter: Painter,
    egui_input_state: EguiInputState,
    pixels_per_point: f32,
    start_time: Instant,
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
            .map_err(|err| Error::GBWindowInit(err))?;

        let egui_painter = Painter::new(video_sys, dim.0, dim.1);
        let egui_ctx = CtxRef::default();

        let native_pixels_per_point = 96f32
            / video_sys
                .display_dpi(0)
                .map_err(|err| Error::GBWindowInit(err))?
                .0;

        let egui_input_state = EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(dim.0 as f32, dim.1 as f32) / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });

        let start_time = Instant::now();
        Ok(Self {
            sdl_window,
            gl_ctx,
            egui_painter,
            egui_ctx,
            egui_input_state,
            pixels_per_point: native_pixels_per_point,
            start_time,
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

    pub fn start_frame(&mut self) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(|err| Error::GBWindowFrame(err))?;
        self.egui_input_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.egui_ctx
            .begin_frame(self.egui_input_state.input.take());
        self.egui_input_state.input.pixels_per_point = Some(self.pixels_per_point);
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(|err| Error::GBWindowFrame(err))?;
        let (_egui_output, paint_cmds) = self.egui_ctx.end_frame();

        let paint_jobs = self.egui_ctx.tessellate(paint_cmds);

        self.egui_painter.paint_jobs(
            None,
            paint_jobs,
            &self.egui_ctx.texture(),
            self.pixels_per_point,
        );

        self.sdl_window.gl_swap_window();
        Ok(())
    }

    pub fn resize(&mut self, dim: (u32, u32), video_sys: &VideoSubsystem) -> Result<(), Error> {
        self.sdl_window
            .gl_make_current(&self.gl_ctx)
            .map_err(|err| Error::GBWindowFrame(err))?;
        self.egui_painter = Painter::new(video_sys, dim.0, dim.1);
        self.egui_input_state = EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(dim.0 as f32, dim.1 as f32) / self.pixels_per_point,
            )),
            pixels_per_point: Some(self.pixels_per_point),
            ..Default::default()
        });
        return Ok(());
    }

    pub fn send_event(&mut self, event: &Event, sdl_context: &Sdl) -> bool {
        if let (Some(id_mouse_focus), Some(id_keyboard_focus)) = (
            sdl_context.mouse().focused_window_id(),
            sdl_context.keyboard().focused_window_id(),
        ) {
            let id = self.sdl_window.id();
            if id_mouse_focus == id || id_keyboard_focus == id {
                egui_sdl2_gl::input_to_egui(event.clone(), &mut self.egui_input_state);
                return true;
            }
        }
        return false;
    }
}
