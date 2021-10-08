mod context;
mod gb_window;
mod render;

use crate::context::Context;
use glium::implement_vertex;
use glium::{glutin, Display};

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
}

fn main() {
    let gl_ctx = Context::new();

    implement_vertex!(Vertex, position);

    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
    ];

    launch_event_loop(gl_ctx, vertices);
}

fn launch_event_loop(mut context: Context, shape: Vec<Vertex>) {
    let vertex_buffer = glium::VertexBuffer::new(&context.gbmu_window().display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &context.gbmu_window().display,
        include_str!("render.vert"),
        include_str!("render.frag"),
        None,
    )
    .unwrap();

    context.event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {
                    context.gbmu_window().egui.on_event(&event);
                    display.gl_window().window().request_redraw();
                }
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {}
            glutin::event::Event::RedrawRequested(_id) => {
                egui.begin_frame(&display);
                egui::TopBottomPanel::top("top").show(egui.ctx(), |ui| {
                    egui::menu::bar(ui, |ui| {
                        egui::menu::menu(ui, "hi", |ui| {
                            if ui.button("thing").clicked() {
                                println!("Hello");
                            }
                        });
                    });
                });
                let (repaint_needed, shapes) = egui.end_frame(&display);
                if repaint_needed {
                    display.gl_window().window().request_redraw()
                }

                let mut target = context.draw();
                target.clear_color(1.0, 1.0, 1.0, 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
                egui.paint(&display, &mut target, shapes);
                target.finish().unwrap();
            }
            _ => {}
        }
    });
}
