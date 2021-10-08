use glium::index::NoIndices;
use glium::pixel_buffer::PixelBuffer;
use glium::{Display, Program, VertexBuffer};
use std::error::Error;

use glium::implement_vertex;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const MENU_BAR_SIZE: f32 = 30.0;

pub type RenderData<const WIDTH: usize, const HEIGHT: usize> = [[[u8; 3]; WIDTH]; HEIGHT];

const VS_SRC: &str = include_str!("render.vert");
const FS_SRC: &str = include_str!("render.frag");

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
}

static VERTICES: Vec<Vertex> = vec![
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

pub struct RenderImage<const WIDTH: usize, const HEIGHT: usize> {
    program: Program,
    vbo: VertexBuffer<Vertex>,
    vao: NoIndices,
    texture_buffer: PixelBuffer<u8>,
    scale: (f32, f32),
    offset: (f32, f32),
    menu_bar_size: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize> RenderImage<WIDTH, HEIGHT> {
    pub fn new(gb_display: &Display) -> Result<Self, Box<dyn Error>> {
        implement_vertex!(Vertex, position);

        Ok(Self {
            program: Program::from_source(gb_display, VS_SRC, FS_SRC, None)?,
            vbo: VertexBuffer::new(gb_display, &VERTICES)?,
            vao: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            texture_buffer: PixelBuffer::new_empty(gb_display, WIDTH * HEIGHT),
            scale: (1.0, 1.0),
            offset: (0.0, 0.0),
            menu_bar_size: 0.0,
        })
    }
}
