use glium::index::NoIndices;
use glium::pixel_buffer::PixelBuffer;
use glium::{Display, Program, VertexBuffer};
use std::error::Error;

use glium::implement_vertex;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const MENU_BAR_SIZE: f32 = 30.0;

pub type RenderData<const SIZE: usize> = [(u8, u8, u8); SIZE];

const VS_SRC: &str = include_str!("render.vert");
const FS_SRC: &str = include_str!("render.frag");

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
}

static VERTICES: [Vertex; 6] = [
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

pub struct RenderImage<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize> {
    program: Program,
    vbo: VertexBuffer<Vertex>,
    vao: NoIndices,
    texture: PixelBuffer<(u8, u8, u8)>,
    scale: (f32, f32),
    offset: (f32, f32),
    menu_bar_size: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize, const SIZE: usize> RenderImage<WIDTH, HEIGHT, SIZE> {
    pub fn new(gb_display: &Display) -> Result<Self, Box<dyn Error>> {
        Self::with_bar_size(gb_display, MENU_BAR_SIZE)
    }

    pub fn with_bar_size(gb_display: &Display, size: f32) -> Result<Self, Box<dyn Error>> {
        implement_vertex!(Vertex, position);
        Ok(Self {
            program: Program::from_source(gb_display, VS_SRC, FS_SRC, None)?,
            vbo: VertexBuffer::new(gb_display, &VERTICES.to_vec())?,
            vao: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            texture: PixelBuffer::new_empty(gb_display, WIDTH * HEIGHT),
            scale: (1.0, 1.0),
            offset: (0.0, 0.0),
            menu_bar_size: size,
        })
    }

    pub fn resize(&mut self, dim: (u32, u32)) {
        let dim = (dim.0 as f32, dim.1 as f32);
        let screen_ratio = WIDTH as f32 / HEIGHT as f32;

        let free_dim = (dim.0, dim.1 - self.menu_bar_size);
        let free_ratio = dim.1 / free_dim.1;
        let actual_ratio = free_dim.0 / free_dim.1;

        let target_dim = if screen_ratio > actual_ratio {
            (dim.0, dim.0 / screen_ratio)
        } else {
            let tmp_dim = (dim.1 * screen_ratio, dim.1);
            if tmp_dim.1 > free_dim.1 {
                (tmp_dim.0 / free_ratio, tmp_dim.1 / free_ratio)
            } else {
                tmp_dim
            }
        };
        let final_dim = (target_dim.0 as u32, target_dim.1 as u32);

        self.scale = (final_dim.0 as f32 / dim.0, final_dim.1 as f32 / dim.1);
        self.offset = (0.0, self.scale.1 - 1.0);
        if screen_ratio > actual_ratio {
            self.offset.1 += 1.0 * (free_dim.1 - target_dim.1) / dim.1;
        }
    }

    pub fn update_render(&mut self, texture_pixels: &RenderData<SIZE>) {
        self.texture.write(texture_pixels);
    }
}
