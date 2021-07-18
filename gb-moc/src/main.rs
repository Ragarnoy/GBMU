use egui_sdl2_gl as egui_backend;

use egui_backend::sdl2::event::Event;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, gl, sdl2};
use std::time::Instant;

use egui_backend::egui::{vec2, Pos2, Rect};

const BAR_SIZE: f32 = 30.0;
const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const SCREEN_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
mod triangle;

fn main() {
    println!("ratio: {}", SCREEN_RATIO);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);

    // OpenGL 3.3 is the minimum that we will support.
    gl_attr.set_context_version(3, 3);

    let mut window = video_subsystem
        .window("GBMU", SCREEN_WIDTH, SCREEN_HEIGHT + BAR_SIZE as u32)
        // .resizable()
        .opengl()
        .build()
        .unwrap();

    window
        .set_minimum_size(SCREEN_WIDTH, SCREEN_HEIGHT + BAR_SIZE as u32)
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    let (width, height) = window.size();

    let mut painter = egui_backend::Painter::new(&video_subsystem, width, height);
    let mut egui_ctx = egui::CtxRef::default();

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let native_pixels_per_point = 96f32 / video_subsystem.display_dpi(0).unwrap().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });
    let start_time = Instant::now();

    //We will draw a crisp white triangle using OpenGL.
    let triangle = triangle::Triangle::new();

    'running: loop {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());

        //In egui 0.10.0 we seem to be losing the value to pixels_per_point,
        //so setting it every frame now.
        //TODO: Investigate if this is the right way.
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        //An example of how OpenGL can be used to draw custom stuff with egui
        //overlaying it:
        //First clear the background to something nice.
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        //Then draw our triangle.
        triangle.draw();

        egui::containers::TopBottomPanel::top("Top menu").show(&egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.set_height(BAR_SIZE);
                if ui.button("Load").clicked() {}
                if ui.button("Debug").clicked() {}
            })
        });

        let (egui_output, paint_cmds) = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !egui_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, egui_output.copied_text);
        }

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
            native_pixels_per_point,
        );

        window.gl_swap_window();

        //Using regular SDL2 event pipeline
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(width, height) => {
                        painter = egui_backend::Painter::new(
                            &video_subsystem,
                            width as u32,
                            height as u32,
                        );
                        egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
                            screen_rect: Some(Rect::from_min_size(
                                Pos2::new(0f32, 0f32),
                                vec2(width as f32, height as f32) / native_pixels_per_point,
                            )),
                            pixels_per_point: Some(native_pixels_per_point),
                            ..Default::default()
                        });
                    }
                    _ => {}
                },
                _ => {
                    egui_backend::input_to_egui(event, &mut egui_input_state);
                }
            }
        }
        std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
