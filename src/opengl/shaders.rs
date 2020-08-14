use super::gl;
use gl::types::{GLchar, GLint, GLuint};
use std::ffi::CString;

pub struct Shader {
    vertex_id: GLuint,
    fragment_id: GLuint,
}
impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str, gl: &gl::Gl) -> Shader {
        // vertex shader
        let vertex_shader = unsafe { gl.CreateShader(gl::VERTEX_SHADER) };
        let vert_source = CString::new(vertex_src).expect("Error loading vertex shader");
        unsafe {
            gl.ShaderSource(vertex_shader, 1, &vert_source.as_ptr(), std::ptr::null());
            gl.CompileShader(vertex_shader);
        }
        Self::handle_shader_errors(vertex_shader, gl);

        // fragment shader
        let fragment_shader = unsafe { gl.CreateShader(gl::FRAGMENT_SHADER) };
        let frag_source = CString::new(fragment_src).expect("Error loading fragment shader");

        unsafe {
            gl.ShaderSource(fragment_shader, 1, &frag_source.as_ptr(), std::ptr::null());
            gl.CompileShader(fragment_shader);
        }

        Self::handle_shader_errors(fragment_shader, gl);

        return Shader {
            vertex_id: vertex_shader,
            fragment_id: fragment_shader,
        };
    }

    pub fn get_vertex_id(&self) -> GLuint {
        self.vertex_id
    }

    pub fn get_fragment_id(&self) -> GLuint {
        self.fragment_id
    }

    fn handle_shader_errors(shader_id: GLuint, gl: &gl::Gl) {
        let mut success: GLint = 0;
        unsafe {
            gl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error_msg = whitespaces_cstring(len as usize);

            unsafe {
                gl.GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    error_msg.as_ptr() as *mut GLchar,
                );
            }

            println!(
                "Shader error: {}",
                error_msg
                    .into_string()
                    .expect("Error message into_string() failed")
            );
        }
    }

    pub fn delete(self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteShader(self.get_vertex_id());
            gl.DeleteShader(self.get_fragment_id());
        }
    }
}

pub struct Program {
    id: GLuint,
}
impl Program {
    pub fn new(shader: &Shader, gl: &gl::Gl) -> Program {
        let program: GLuint;
        unsafe {
            program = gl.CreateProgram();
            gl.AttachShader(program, shader.get_vertex_id());
            gl.AttachShader(program, shader.get_fragment_id());
            gl.LinkProgram(program);
        }

        // error handling ---

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error_msg = whitespaces_cstring(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error_msg.as_ptr() as *mut GLchar,
                );
            }

            println!(
                "Shader program error: {}",
                error_msg
                    .into_string()
                    .expect("Error message into_string() failed")
            );
        }

        // ---

        /*
        unsafe {
            gl.DetachShader(program, shader.get_vertex_id());
            gl.DetachShader(program, shader.get_fragment_id());
        }
        */

        return Program { id: program };
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.UseProgram(self.id);
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn delete(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteProgram(self.id);
        }
    }
}

pub fn whitespaces_cstring(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
