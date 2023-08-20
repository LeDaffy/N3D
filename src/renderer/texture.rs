use gl::{self, types::*};
use image::GenericImageView;
use std::error::Error;
use thiserror::Error;

#[derive(Debug)]
pub enum TextureFormat {
    Rgb,
    Rgba,
}
#[derive(Debug)]
pub enum TextureBuffer {
    Byte(Vec<u8>),
    Short(Vec<u8>),
    Float(Vec<u8>),
}

#[derive(Debug)]
pub struct Texture {
    pub id: Option<GLuint>,
    pub format: TextureFormat,
    pub buffer: TextureBuffer,
    pub dimensions: (usize, usize),
}

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("invalid format")]
    InvalidFormat,
    #[error("ID already set")]
    IDSet,
}

impl Texture {
    pub fn new<U: Into<usize>>(format: TextureFormat, buffer: TextureBuffer, dimensions: (U, U)) -> Self {
        Self {
            id: None,
            format,
            buffer,
            dimensions: (dimensions.0.into(), dimensions.1.into()),
        }
    }
    pub fn open<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let image = image::io::Reader::open(path)?.decode()?;
        //let buf = image.as_bytes();
        let mut format = TextureFormat::Rgb;
        match image.color() {
            image::ColorType::Rgb8 | image::ColorType::Rgb16 | image::ColorType::Rgb32F => {
                format = TextureFormat::Rgb
            }
            image::ColorType::Rgba8 | image::ColorType::Rgba16 | image::ColorType::Rgba32F => {
                format = TextureFormat::Rgba
            }
            _ => {}
        }
        let buf;
        match image.color() {
            image::ColorType::Rgb8 | image::ColorType::Rgba8 => {
                buf = TextureBuffer::Byte(image.as_bytes().to_vec());
            }
            image::ColorType::Rgb16 | image::ColorType::Rgba16 => {
                buf = TextureBuffer::Short(image.as_bytes().to_vec());
            }
            image::ColorType::Rgb32F | image::ColorType::Rgba32F => {
                buf = TextureBuffer::Float(image.as_bytes().to_vec());
            }
            _ => {
                return Err(Box::new(TextureError::InvalidFormat));
            }
        }

        Ok(Self {
            id: None,
            format: format,
            buffer: buf,
            dimensions: (image.dimensions().0 as usize, image.dimensions().1 as usize),
        })
    }

    /// Generate an opengl texture
    pub fn gen(&mut self) {
        self.id = Some(0);
        if let Some(ref mut id) = self.id {
            unsafe {
                gl::GenTextures(1, id);
                gl::BindTexture(gl::TEXTURE_2D, *id);

                // texture wrapping
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

                // texture filtering
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

                // texture filtering
                gl::TexParameteri(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_MIN_FILTER,
                    gl::NEAREST_MIPMAP_NEAREST as i32,
                );
                gl::TexParameteri(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_MAG_FILTER,
                    gl::NEAREST_MIPMAP_NEAREST as i32,
                );
            }
        }
        match (&self.format, &self.buffer) {
            (TextureFormat::Rgb, TextureBuffer::Byte(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
            (TextureFormat::Rgba, TextureBuffer::Byte(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
            (TextureFormat::Rgb, TextureBuffer::Short(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_SHORT,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
            (TextureFormat::Rgba, TextureBuffer::Short(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_SHORT,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
            (TextureFormat::Rgb, TextureBuffer::Float(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGB,
                    gl::FLOAT,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
            (TextureFormat::Rgba, TextureBuffer::Float(buf)) => unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    self.dimensions.0 as i32,
                    self.dimensions.1 as i32,
                    0,
                    gl::RGBA,
                    gl::FLOAT,
                    buf.as_ptr() as *const std::ffi::c_void,
                );
            },
        }
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
    pub fn bind(&self) -> Result<(), TextureError> {
        match self.id {
            Some(id) => {
                unsafe {
                    gl::BindTexture(gl::TEXTURE_2D, id);
                }
                Ok(())
            }
            _ => Err(TextureError::IDSet),
        }
    }
    pub fn set_unit(&self, unit: gl::types::GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
        }
    }
    pub fn set_active_unit(unit: gl::types::GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
        }
    }
}
