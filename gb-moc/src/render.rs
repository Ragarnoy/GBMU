use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;
pub use crate::MENU_BAR_SIZE;

const VS_SRC: &'static str = "
#version 330
uniform vec2 scale;
uniform vec2 offset;
in vec2 position;
out vec2 text_coord;
void main() {
	text_coord = position;
	gl_Position = vec4(position * scale + offset, 0.0, 1.0);
}";

const FS_SRC: &'static str = "
#version 330
in vec2 text_coord;
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

static VERTEX_DATA: [GLfloat; 6] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0];

pub struct Triangle {
	pub vs: GLuint,
	pub fs: GLuint,
	pub program: GLuint,
	pub vao: GLuint,
	pub vbo: GLuint,
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
		Triangle {
			// Create GLSL shaders
			vs,
			fs,
			program,
			vao,
			vbo,
			scale: (1.0, 0.82758623),
			offset: (0.0, 0.82758623 - 1.0),
		}
	}

	pub fn resize(&mut self, dim: (u32, u32)) {
		let available_dim = (dim.0 as f32, dim.1 as f32 - MENU_BAR_SIZE);
		let available_ratio = available_dim.0 / available_dim.1;
		let target_dim = if SCREEN_RATIO > available_ratio {
			(available_dim.0, available_dim.0 / SCREEN_RATIO)
		} else {
			(available_dim.1 * SCREEN_RATIO, available_dim.1)
		};
		// println!("target dim: {:?};\t dim: {:?}", target_dim, dim);
		let final_dim = if target_dim.1 + MENU_BAR_SIZE > available_dim.1 {
			(target_dim.0, target_dim.1 * target_dim.1 / dim.1 as f32)
		} else {
			target_dim
		};
		self.scale = (final_dim.0 / available_dim.0, final_dim.1 / available_dim.1);
		self.offset = (0.0, self.scale.1 - 1.0);
		// if SCREEN_RATIO > available_ratio {
		// 	self.offset.1 += 0.5 * (available_dim.1 / dim.1 as f32);
		// }
		// println!("scale: {:?};\t offset: {:?}", self.scale, self.offset);
	}

	pub fn draw(&self) {
		unsafe {
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
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
		}
	}
}
