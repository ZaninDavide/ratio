//! Examples from the glutin library:
//! https://github.com/rust-windowing/glutin/tree/f071c722f725143d80638f1c5c12a76d9d8e1be8/glutin_examples
//! https://github.com/rust-windowing/glutin/blob/f071c722f725143d80638f1c5c12a76d9d8e1be8/glutin_examples/examples/raw_context.rs

pub mod buffers;
pub mod shaders;
pub mod textures;
pub mod uniforms;

use glutin::{self, PossiblyCurrent};

use gl::types::{GLint, GLsizei};
use std::ffi::CStr;

/// Glwrapper safely provides all the necessary functions to communicate with openGl
/// ```
/// let gl = openGl::Glwrapper::new(&windowed_context.context());
/// gl.draw_frame([1.0, 0.0, 0.0, 1.0]); // draw red screen
/// ```
pub struct Glwrapper {
    pub gl: gl::Gl,
}

impl Glwrapper {
    pub fn new(gl_context: &glutin::Context<PossiblyCurrent>) -> Glwrapper {
        let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };
        unsafe {
            gl.Enable(gl::DEBUG_OUTPUT);
            gl.DebugMessageCallback(debug_callback, std::ptr::null());

            gl.Enable(gl::DEPTH_TEST);
            gl.DepthMask(gl::TRUE);
        }
        println!("OpenGL version {}", version);

        Glwrapper { gl: gl }
    }

    pub fn resize(&self, width: GLint, height: GLint) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }

    pub fn draw_frame(&self, color: [f32; 4]) {
        unsafe {
            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangles(&self, count: usize) {
        unsafe {
            self.gl.DrawArrays(gl::TRIANGLES, 0, count as i32);
        }
    }

    pub fn draw_elements(&self, indices_to_draw: i32) {
        unsafe {
            self.gl.ClearColor(0.8, 0.1, 0.3, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawElements(
                gl::TRIANGLES,
                indices_to_draw as GLsizei,
                gl::UNSIGNED_INT,
                std::ptr::null() as *const gl::types::GLvoid,
            );
        }
    }

    pub fn print_errors(&self) {
        let err;
        unsafe {
            err = self.gl.GetError();
        }
        match err {
            gl::NO_ERROR => {}
            gl::INVALID_VALUE => {
                println!("ERROR: GL_INVALID_VALUE");
            }
            gl::INVALID_OPERATION => {
                println!("ERROR: GL_INVALID_OPERATION");
            }
            _ => {
                println!("ERROR: {}", err);
            }
        }
    }

    pub fn bind_drawing_buffer(&self) {
        unsafe {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn clear_depth_buffer(&self) {
        unsafe {
            self.gl.Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn depth_test(&self, enable: bool) {
        if enable {
            unsafe {
                self.gl.Enable(gl::DEPTH_TEST);
            }
        } else {
            unsafe {
                self.gl.Disable(gl::DEPTH_TEST);
            }
        }
    }
}

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

extern "system" fn debug_callback(
    _source: gl::types::GLenum,
    etype: gl::types::GLenum,
    _id: gl::types::GLuint,
    _severity: gl::types::GLenum,
    _msg_length: gl::types::GLsizei,
    msg: *const gl::types::GLchar,
    _user_data: *mut std::ffi::c_void,
) {
    unsafe {
        if etype == gl::DEBUG_TYPE_ERROR {
            println!("ERROR: {:?}", std::ffi::CStr::from_ptr(msg));
        } else {
            println!("DEBUG: {:?}", std::ffi::CStr::from_ptr(msg));
        }
    }
}
