use super::gl;
use super::shaders::Program;
use gl::types::{GLchar, GLint, GLuint};

pub enum UniformType {
    Float(f32),
    Float2([f32; 2]),
    Float3([f32; 3]),
    Float4([f32; 4]),
    Mat3x3([f32; 9]),
    Mat4x4([f32; 16]),
    Int(i32),
    Int2([i32; 2]),
    Int3([i32; 3]),
    Int4([i32; 4]),
    Bool(bool),
    Texture(GLuint),
}

pub struct Uniform {
    name: String,
    id: Option<GLint>, // uniform location
}
impl Uniform {
    pub fn new(name: &str, value: UniformType, program: &Program, gl: &gl::Gl) -> Uniform {
        let cname = std::ffi::CString::new(name)
            .expect(&format!("Error getting CString from: {}", name)[..]);

        let location;
        unsafe {
            location = gl.GetUniformLocation(program.get_id(), cname.as_ptr());
        }
        if location == -1 {
            println!(
                "WARN: Uniform '{}' is inactive and got -1 as location.",
                name
            );
        }

        // DOCS: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform.xhtml
        match value {
            UniformType::Float(v) => unsafe {
                gl.Uniform1f(location, v);
            },
            UniformType::Float4(v) => unsafe {
                gl.Uniform4fv(location, 1, &v[0]); // v.as_ptr() as *const GLfloat
            },
            UniformType::Texture(tex_id) => unsafe {
                gl.Uniform1i(location, tex_id as i32); // The docs says to use Uniform1i or Uniform1iv.
            },
            UniformType::Mat4x4(v) => unsafe {
                gl.UniformMatrix4fv(location, 1, gl::FALSE, &v[0]);
            },
            _ => {
                panic!("Uniform::new This uniform type is unknown");
            }
        }

        return Uniform {
            name: String::from(name),
            id: Some(location),
        };
    }

    pub fn get_id(&self, program: &Program, gl: &gl::Gl) -> i32 {
        let location;
        match self.id {
            Some(id) => {
                location = id;
            }
            _ => unsafe {
                location =
                    gl.GetUniformLocation(program.get_id(), self.name.as_ptr() as *const GLchar);
            },
        }
        return location;
    }

    pub fn set(&mut self, value: UniformType, program: &Program, gl: &gl::Gl) {
        let location = self.get_id(program, gl);
        if location == -1 {
            println!(
                "WARN: Uniform '{}' is inactive and got -1 as location.",
                self.name
            );
        }
        match value {
            UniformType::Float(v) => unsafe {
                gl.Uniform1f(location, v);
            },
            UniformType::Float4(v) => unsafe {
                gl.Uniform4fv(location, 1, &v[0]); // v.as_ptr() as *const GLfloat
            },
            UniformType::Mat4x4(v) => unsafe {
                gl.UniformMatrix4fv(location, 1, gl::FALSE, &v[0]); // v.as_ptr() as *const GLfloat
            },
            _ => {
                panic!("Uniform::set This uniform type is unknown");
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }
}
