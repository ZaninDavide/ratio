use super::gl;
use gl::types::{GLsizei, GLuint};

extern crate image;
use crate::opengl::textures::image::GenericImageView;

pub enum TextureDataType {
    Float,
    UnsignedByte,
}
impl TextureDataType {
    fn to_gl(&self) -> u32 {
        match self {
            TextureDataType::Float => gl::FLOAT,
            TextureDataType::UnsignedByte => gl::UNSIGNED_BYTE,
        }
    }
}

pub enum TextureColorFormat {
    RGB,
    RGBA,
    RGBA4,
}
impl TextureColorFormat {
    fn to_gl(&self) -> u32 {
        match self {
            TextureColorFormat::RGB => gl::RGB,
            TextureColorFormat::RGBA => gl::RGBA,
            TextureColorFormat::RGBA4 => gl::RGBA4,
        }
    }
}

pub struct Texture {
    id: GLuint,
    location: GLuint,
    data_type: TextureDataType,
    color_format: TextureColorFormat,
}
impl Texture {
    pub fn new(
        id_counter: u32,
        width: i32,
        height: i32,
        data: Option<Vec<f32>>,
        color_format: TextureColorFormat,
        data_type: TextureDataType,
        gl: &gl::Gl,
    ) -> Texture {
        let pixels = match data {
            Some(d) => d.as_ptr() as *const std::ffi::c_void,
            None => std::ptr::null(),
        };
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.ActiveTexture(gl::TEXTURE0 + id_counter);
            gl.BindTexture(gl::TEXTURE_2D, texture);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                color_format.to_gl() as i32,
                width,
                height,
                0,
                color_format.to_gl(),
                data_type.to_gl(),
                pixels,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        return Texture {
            id: id_counter,
            location: texture,
            data_type: data_type,
            color_format: color_format,
        };
    }

    pub fn load_new(path: &str, id_counter: u32, gl: &gl::Gl) -> Texture {
        let img = image::open(&std::path::Path::new(path)).unwrap();
        let img_size = img.dimensions();
        let data = img.to_bytes();

        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.ActiveTexture(gl::TEXTURE0 + id_counter);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img_size.0 as GLsizei,
                img_size.1 as GLsizei,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        return Texture {
            id: id_counter,
            location: texture,
            data_type: TextureDataType::UnsignedByte,
            color_format: TextureColorFormat::RGB,
        };
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0 + self.id);
            gl.BindTexture(gl::TEXTURE_2D, self.location);
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_location(&self) -> u32 {
        self.location
    }

    pub fn delete(&self, gl: &gl::Gl) {
        // should be implemented in a drop fuction
        // the problem with that is that Texture should store a copy of gl
        // in that case we should specify lifetimes
        unsafe {
            gl.DeleteTextures(1, &self.location);
        }
    }

    pub fn attach_to_frame_buffer(&self, gl: &gl::Gl) {
        unsafe {
            self.bind(gl);
            gl.FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.location,
                0,
            );
        }
    }

    pub fn resize(&self, width: usize, height: usize, gl: &gl::Gl) {
        self.bind(gl);
        unsafe {
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.color_format.to_gl() as i32,
                width as GLsizei,
                height as GLsizei,
                0,
                self.color_format.to_gl(),
                self.data_type.to_gl(),
                std::ptr::null(),
            );
        }
    }
}
