mod gb_window;

use glium::glutin;
use glium::glutin::dpi::LogicalSize;
use glium::implement_vertex;
use glium::Surface;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("GBMU")
        .with_inner_size(LogicalSize::new(160, 144));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut egui = egui_glium::EguiGlium::new(&display);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => egui.on_event(&event),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
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

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
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
            _ => return,
        }
    });
}

fn event_loop() {}
