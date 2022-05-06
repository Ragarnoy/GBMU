use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

use crate::shader;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const MENU_BAR_SIZE: f32 = 30.0;

pub type ImageRGB<const WIDTH: usize, const HEIGHT: usize> = [[[u8; 3]; WIDTH]; HEIGHT];
pub type Render = RenderImage<SCREEN_WIDTH, SCREEN_HEIGHT>;

const VS_SRC: &str = include_str!("render.vert");
const FS_SRC: &str = include_str!("render.frag");

static VERTEX_DATA: [GLfloat; 12] = [
    -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
];

pub struct RenderImage<const WIDTH: usize, const HEIGHT: usize> {
    vs: GLuint,
    fs: GLuint,
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    texture_buffer: GLuint,
    scale: (f32, f32),
    offset: (f32, f32),
    menu_bar_size: f32,
}

impl<const WIDTH: usize, const HEIGHT: usize> RenderImage<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Self::with_bar_size(MENU_BAR_SIZE)
    }

    pub fn with_bar_size(menu_bar_size: f32) -> Self {
        let texture_size = WIDTH * HEIGHT;
        // Create Vertex Array Object
        let mut vao = 0;
        let mut vbo = 0;
        let vs = shader::compile(VS_SRC, gl::VERTEX_SHADER);
        let fs = shader::compile(FS_SRC, gl::FRAGMENT_SHADER);
        let program = shader::link(vs, fs);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }
        let mut texture_data: Vec<[u8; 3]> = Vec::new();
        texture_data.resize(texture_size as usize, [255; 3]);
        let mut texture_buffer = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                WIDTH as i32,
                HEIGHT as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &texture_data.as_slice() as *const _ as *const c_void,
            );
        }
        RenderImage {
            vs,
            fs,
            program,
            vao,
            vbo,
            texture_buffer,
            scale: (1.0, 1.0),
            offset: (0.0, 0.0),
            menu_bar_size,
        }
    }

    #[cfg(feature = "debug_render")]
    pub fn switch_draw_mode(&self, lines: bool) {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, if lines { gl::LINE } else { gl::FILL });
        }
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

    pub fn update_render(&mut self, texture_pixels: &ImageRGB<WIDTH, HEIGHT>) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_buffer);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                WIDTH as i32,
                HEIGHT as i32,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                texture_pixels as *const _ as *const c_void,
            );
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::BindVertexArray(self.vao);

            // Create a Vertex Buffer Object and copy the vertex data to it

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&VERTEX_DATA[0]),
                gl::STATIC_DRAW,
            );

            // Use shader program
            gl::UseProgram(self.program);
            let uniform_loc = gl::GetUniformLocation(
                self.program,
                CString::new("scale").unwrap().as_c_str().as_ptr(),
            );
            gl::Uniform2f(uniform_loc, self.scale.0, self.scale.1);
            let uniform_loc = gl::GetUniformLocation(
                self.program,
                CString::new("offset").unwrap().as_c_str().as_ptr(),
            );
            gl::Uniform2f(uniform_loc, self.offset.0, self.offset.1);
            let c_out_color = CString::new("out_color").unwrap();
            gl::BindFragDataLocation(self.program, 0, c_out_color.as_ptr());

            let uniform_loc = gl::GetUniformLocation(
                self.program,
                CString::new("render_texture").unwrap().as_c_str().as_ptr(),
            );
            gl::Uniform1i(uniform_loc, 1);
            gl::ActiveTexture(gl::TEXTURE0 + 1);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_buffer);

            // Specify the layout of the vertex data
            let c_position = CString::new("position").unwrap();
            let pos_attr = gl::GetAttribLocation(self.program, c_position.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Drop for RenderImage<WIDTH, HEIGHT> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteTextures(1, &self.texture_buffer);
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for RenderImage<WIDTH, HEIGHT> {
    fn default() -> RenderImage<WIDTH, HEIGHT> {
        RenderImage::new()
    }
}
