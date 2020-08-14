use super::gl;
use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use std::mem::size_of;

pub struct VertexBuffer {
    id: GLuint,
}
impl VertexBuffer {
    pub fn new(vertices: &Vec<f32>, gl: &gl::Gl) -> VertexBuffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        VertexBuffer { id: vbo }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
    pub fn delete(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteBuffers(1, &self.id);
        }
    }
}

pub enum AttributeType {
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}
impl AttributeType {
    pub fn bytes(&self) -> usize {
        match &self {
            Self::Float => size_of::<f32>(),
            Self::Float2 => size_of::<f32>() * 2,
            Self::Float3 => size_of::<f32>() * 3,
            Self::Float4 => size_of::<f32>() * 4,
            Self::Mat3 => size_of::<f32>() * 3 * 3,
            Self::Mat4 => size_of::<f32>() * 4 * 4,
            Self::Int => size_of::<i32>(),
            Self::Int2 => size_of::<i32>() * 2,
            Self::Int3 => size_of::<i32>() * 3,
            Self::Int4 => size_of::<i32>() * 4,
            Self::Bool => size_of::<bool>(),
        }
    }
    pub fn size(&self) -> GLint {
        match &self {
            Self::Float => 1,
            Self::Float2 => 2,
            Self::Float3 => 3,
            Self::Float4 => 4,
            Self::Mat3 => 3 * 3,
            Self::Mat4 => 4 * 4,
            Self::Int => 1,
            Self::Int2 => 2,
            Self::Int3 => 3,
            Self::Int4 => 4,
            Self::Bool => 1,
        }
    }
    pub fn gl_type(&self) -> GLenum {
        match &self {
            Self::Float => gl::FLOAT,
            Self::Float2 => gl::FLOAT,
            Self::Float3 => gl::FLOAT,
            Self::Float4 => gl::FLOAT,
            Self::Mat3 => gl::FLOAT,
            Self::Mat4 => gl::FLOAT,
            Self::Int => gl::INT,
            Self::Int2 => gl::INT,
            Self::Int3 => gl::INT,
            Self::Int4 => gl::INT,
            Self::Bool => gl::BOOL,
        }
    }
}

pub struct VertexBufferLayout {
    id: GLuint,
    attributes: Vec<(String, AttributeType)>,
}
impl VertexBufferLayout {
    pub fn new(attributes: Vec<(String, AttributeType)>, gl: &gl::Gl) -> VertexBufferLayout {
        let total_size: usize = attributes.iter().map(|a| -> usize { a.1.bytes() }).sum();

        let mut vao: GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut offset: usize = 0;
            for (i, attr) in attributes.iter().enumerate() {
                gl.VertexAttribPointer(
                    i as GLuint,
                    attr.1.size(),
                    attr.1.gl_type(),
                    gl::FALSE,
                    total_size as GLsizei,
                    offset as _,
                );
                gl.EnableVertexAttribArray(i as GLuint);
                offset += attr.1.bytes();
            }
            // gl.BindVertexArray(vao);
        }

        VertexBufferLayout {
            id: vao,
            attributes,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            // gl.BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl.BindVertexArray(self.id);
        }
    }

    pub fn delete(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct IndexBuffer {
    id: GLuint,
    indices_count: usize,
}
impl IndexBuffer {
    pub fn new(indices: Vec<u32>, gl: &gl::Gl) -> IndexBuffer {
        let mut ib: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut ib);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ib);

            let bytes = indices.len() * size_of::<u32>();
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                bytes as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }

        IndexBuffer {
            id: ib,
            indices_count: indices.len(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn get_indices_count(&self) -> usize {
        self.indices_count
    }
}
