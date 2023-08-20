use super::vert::Vert;
use gl::{self, types::*};

pub struct VAO(pub GLuint);

impl VAO {
    pub fn new() -> Self {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        Self(vao)
    }
    pub fn set_attributes(&self) {
        self.bind();
        unsafe {
            // Position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vert>()) as i32,
                (Vert::OFFSET_POS) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(0);
            // UV
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vert>()) as i32,
                (Vert::OFFSET_UV) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(1);
            // Col
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<Vert>()) as i32,
                (Vert::OFFSET_COL) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(2);
        }
        self.unbind();
    }
    pub fn set_egui_attributes(&self) {
        self.bind();
        unsafe {
            // Position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<egui::epaint::Vertex>()) as i32,
                (0) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(0);
            // UV
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<egui::epaint::Vertex>()) as i32,
                (std::mem::size_of::<egui::epaint::Pos2>() * 1) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(1);
            // Color
            gl::VertexAttribPointer(
                2,
                4,
                gl::UNSIGNED_BYTE,
                gl::FALSE,
                (std::mem::size_of::<egui::epaint::Vertex>()) as i32,
                (std::mem::size_of::<egui::epaint::Pos2>() * 2) as *const std::ffi::c_void,
            );
            gl::EnableVertexAttribArray(2);
        }
        self.unbind();
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}
