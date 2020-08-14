use super::gl;
use gl::types::{GLsizei, GLuint};

extern crate image;
use crate::opengl::textures::image::GenericImageView;

pub enum TextureDataType {
    Float,
    UnsignedByte,
}
pub enum TextureColorFormat {
    RGB,
    RGBA,
}

pub struct Texture {
    id: GLuint,
    location: GLuint,
    data_type: TextureDataType,
}
impl Texture {
    pub fn new(
        id_counter: u32,
        width: i32,
        height: i32,
        data: Vec<f32>,
        color_format: TextureColorFormat,
        gl: &gl::Gl,
    ) -> Texture {
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.ActiveTexture(gl::TEXTURE0 + id_counter);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            let color_format = match color_format {
                TextureColorFormat::RGB => gl::RGB,
                TextureColorFormat::RGBA => gl::RGBA,
            };
            let data = [data];
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                color_format as i32,
                width,
                height,
                0,
                color_format,
                gl::FLOAT,
                data.as_ptr() as *const std::ffi::c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        return Texture {
            id: id_counter,
            location: texture,
            data_type: TextureDataType::Float,
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

    pub fn delete(&self, gl: &gl::Gl) {
        // should be implemented in a drop fuction
        // the problem with that is that Texture should store a copy of gl
        // in that case we should specify lifetimes
        unsafe {
            gl.DeleteTextures(1, &self.id);
        }
    }
}
