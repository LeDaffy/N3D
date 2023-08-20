use super::vert::Vert;
use egui::{self};
use gl::{self, types::*};

pub struct VBO(pub GLuint);

impl VBO {
    pub fn new() -> Self {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        Self(vbo)
    }
    pub fn set_buffer_data(&self, verts: &Vec<Vert>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * std::mem::size_of::<Vert>()) as isize,
                verts.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
    }
    pub fn set_egui_data(&self, verts: &Vec<egui::epaint::Vertex>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * std::mem::size_of::<egui::epaint::Vertex>()) as isize,
                verts.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.0);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}
