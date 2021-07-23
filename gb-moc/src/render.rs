use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
pub const TEXTURE_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
pub use crate::MENU_BAR_SIZE;

const VS_SRC: &'static str = include_str!("render.vert");
const FS_SRC: &'static str = include_str!("render.frag");

static VERTEX_DATA: [GLfloat; 12] = [
	-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
];

pub struct Triangle {
	pub vs: GLuint,
	pub fs: GLuint,
	pub program: GLuint,
	pub vao: GLuint,
	pub vbo: GLuint,
	pub texture_buffer: GLuint,
	pub scale: (f32, f32),
	pub offset: (f32, f32),
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
	let shader;
	unsafe {
		shader = gl::CreateShader(ty);
		// Attempt to compile the shader
		let c_str = CString::new(src.as_bytes()).unwrap();
		gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
		gl::CompileShader(shader);

		// Get the compile status
		let mut status = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

		// Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len = 0;
			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::with_capacity(len as usize);
			buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
			gl::GetShaderInfoLog(
				shader,
				len,
				ptr::null_mut(),
				buf.as_mut_ptr() as *mut GLchar,
			);
			panic!(
				"{}",
				str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
			);
		}
	}
	shader
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
	unsafe {
		let program = gl::CreateProgram();
		gl::AttachShader(program, vs);
		gl::AttachShader(program, fs);
		gl::LinkProgram(program);
		// Get the link status
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

		// Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::with_capacity(len as usize);
			buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
			gl::GetProgramInfoLog(
				program,
				len,
				ptr::null_mut(),
				buf.as_mut_ptr() as *mut GLchar,
			);
			panic!(
				"{}",
				str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
			);
		}
		program
	}
}

impl Triangle {
	pub fn new() -> Self {
		// Create Vertex Array Object
		let mut vao = 0;
		let mut vbo = 0;
		let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
		let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
		let program = link_program(vs, fs);
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);
		}
		let mut texture_data: [[u8; 3]; TEXTURE_SIZE] = [[255; 3]; TEXTURE_SIZE];
		for j in 0..SCREEN_HEIGHT {
			for i in 0..SCREEN_WIDTH {
				if (i + j) % 2 == 0 {
					texture_data[(i + j * SCREEN_WIDTH) as usize] =
						if j == 0 || j == SCREEN_HEIGHT - 1 || i == 0 || i == SCREEN_WIDTH - 1 {
							[150, 50, 50]
						} else {
							[100; 3]
						};
				}
			}
		}
		let mut texture_buffer = 0;
		unsafe {
			gl::GenTextures(1, &mut texture_buffer);
			gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGB as i32,
				160,
				144,
				0,
				gl::RGB,
				gl::UNSIGNED_BYTE,
				&texture_data as *const _ as *const c_void,
			);
		}
		Triangle {
			// Create GLSL shaders
			vs,
			fs,
			program,
			vao,
			vbo,
			texture_buffer,
			scale: (1.0, 0.82758623),
			offset: (0.0, 0.82758623 - 1.0),
		}
	}

	pub fn resize(&mut self, dim: (u32, u32)) {
		let dim = (dim.0 as f32, dim.1 as f32);
		let actual_ratio = dim.0 / dim.1;

		let free_dim = (dim.0, dim.1 - MENU_BAR_SIZE);
		let free_ratio = dim.1 / free_dim.1;

		let target_dim = if SCREEN_RATIO > actual_ratio {
			(dim.0, dim.0 / SCREEN_RATIO)
		} else {
			let tmp_dim = (dim.1 * SCREEN_RATIO, dim.1);
			if tmp_dim.1 > free_dim.1 {
				(tmp_dim.0 / free_ratio, tmp_dim.1 / free_ratio)
			} else {
				tmp_dim
			}
		};
		let final_dim = (target_dim.0 as u32, target_dim.1 as u32);

		self.scale = (final_dim.0 as f32 / dim.0, final_dim.1 as f32 / dim.1);
		self.offset = (0.0, self.scale.1 - 1.0);

		println!("scale: {:?};\t offset: {:?}", self.scale, self.offset);
		println!("target dim: {:?};\t dim: {:?}", target_dim, dim);
		println!(
			"dim:\tx: {};\t y: {}",
			self.scale.0 * dim.0,
			self.scale.1 * dim.1
		);
		println!(
			"off:\tx: {};\t y: {}",
			self.offset.0 * dim.0,
			self.offset.1 * dim.1
		)
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

impl Drop for Triangle {
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
