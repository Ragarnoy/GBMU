mod context;
mod gb_window;
mod render;

use crate::context::Context;
use glium::glutin;
use glium::{implement_vertex, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut ctx = Context::new(&event_loop);

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
    let vertex_buffer = glium::VertexBuffer::new(&ctx.gbmu_window().display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &ctx.gbmu_window().display,
        include_str!("render.vert"),
        include_str!("render.frag"),
        None,
    )
    .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {
                    ctx.gbmu_window_mut().egui.on_event(&event);
                    ctx.gbmu_window()
                        .display
                        .gl_window()
                        .window()
                        .request_redraw();
                }
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {}
            glutin::event::Event::RedrawRequested(_id) => {
                ctx.gbmu_window_mut()
                    .egui
                    .begin_frame(&ctx.gbmu_window().display);
                egui::TopBottomPanel::top("top").show(ctx.gbmu_window().egui.ctx(), |ui| {
                    egui::menu::bar(ui, |ui| {
                        egui::menu::menu(ui, "hi", |ui| {
                            if ui.button("thing").clicked() {
                                println!("Hello");
                            }
                        });
                    });
                });
                let (repaint_needed, shapes) = ctx
                    .gbmu_window_mut()
                    .egui
                    .end_frame(&ctx.gbmu_window().display);
                if repaint_needed {
                    ctx.gbmu_window()
                        .display
                        .gl_window()
                        .window()
                        .request_redraw()
                }

                let mut target = ctx.gbmu_window().display.draw();
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
                ctx.gbmu_window_mut()
                    .egui
                    .paint(&ctx.gbmu_window().display, &mut target, shapes);
                target.finish().unwrap();
            }
            _ => {}
        }
    });
}
