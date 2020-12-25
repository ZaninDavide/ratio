use super::gl;
use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use std::mem::size_of;

use super::textures::{Texture, TextureColorFormat, TextureDataType};

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
            // gl.BindBuffer(gl::ARRAY_BUFFER, 0);
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

pub struct FrameBuffer {
    id: GLuint,
    texture: Texture,
}
impl FrameBuffer {
    pub fn new(
        texture_counter: u32,
        vw: usize,
        vh: usize,
        data_type: TextureDataType,
        gl: &gl::Gl,
    ) -> FrameBuffer {
        let mut fbo: GLuint = 0;
        let texture;
        unsafe {
            gl.GenFramebuffers(1, &mut fbo);
            gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);

            texture = Texture::new(
                texture_counter,
                vw as i32,
                vh as i32,
                None,
                TextureColorFormat::RGB,
                data_type,
                gl,
            );
            texture.attach_to_frame_buffer(gl);

            let status = gl.CheckFramebufferStatus(gl::FRAMEBUFFER);
            match status {
                gl::FRAMEBUFFER_COMPLETE => {}
                gl::FRAMEBUFFER_UNDEFINED => {
                    println!("ERROR: frame buffer GL_FRAMEBUFFER_UNDEFINED");
                }
                gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    println!("ERROR: frame buffer GL_INCOMPLETE_ATTACHMENT");
                }
                gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    println!("ERROR: frame buffer GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT");
                }
                gl::FRAMEBUFFER_UNSUPPORTED => {
                    println!("ERROR: frame buffer FRAMEBUFFER_UNSUPPORTED");
                }
                _ => {
                    println!("ERROR: frame buffer: {}", status);
                }
            }

            // back to default frame buffer
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        FrameBuffer {
            id: fbo,
            texture: texture,
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn bind_texture(&self, gl: &gl::Gl) {
        self.texture.bind(gl);
    }

    pub fn delete(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteFramebuffers(1, &self.id);
        }
    }

    pub fn resize_texture(&self, width: usize, height: usize, gl: &gl::Gl) {
        self.texture.resize(width, height, gl);
    }
}

pub struct RenderBuffer {
    id: GLuint,
}
impl RenderBuffer {
    pub fn new(width: usize, height: usize, gl: &gl::Gl) -> RenderBuffer {
        let mut rbo: GLuint = 0;
        unsafe {
            gl.GenRenderbuffers(1, &mut rbo);
            gl.BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl.RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
            gl.BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl.FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            let status = gl.CheckFramebufferStatus(gl::FRAMEBUFFER);
            match status {
                gl::FRAMEBUFFER_COMPLETE => {}
                gl::FRAMEBUFFER_UNDEFINED => {
                    println!("ERROR: render buffer GL_FRAMEBUFFER_UNDEFINED");
                }
                gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    println!("ERROR: render buffer GL_INCOMPLETE_ATTACHMENT");
                }
                gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    println!("ERROR: render buffer GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT");
                }
                gl::FRAMEBUFFER_UNSUPPORTED => {
                    println!("ERROR: render buffer FRAMEBUFFER_UNSUPPORTED");
                }
                _ => {
                    println!("ERROR: render buffer: {}", status);
                }
            }

            // back to default frame buffer
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        RenderBuffer { id: rbo }
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn resize(&self, width: usize, height: usize, gl: &gl::Gl) {
        unsafe {
            gl.BindRenderbuffer(gl::RENDERBUFFER, self.id);
            gl.RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
        }
    }
}
