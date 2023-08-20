use gl::{self, types::*};

pub struct EBO(pub GLuint);

impl EBO {
    pub fn new() -> Self {
        let mut ebo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
        }
        Self(ebo)
    }
    pub fn set_buffer_data(&self, elements: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (elements.len() * std::mem::size_of::<u32>()) as isize,
                elements.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
        // remember: do NOT unbind the EBO while a VAO is active as the bound element buffer object IS stored in the VAO; keep the EBO bound.
    }
    pub fn set_egui_data(&self, elements: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (elements.len() * std::mem::size_of::<u32>()) as isize,
                elements.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
        // remember: do NOT unbind the EBO while a VAO is active as the bound element buffer object IS stored in the VAO; keep the EBO bound.
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.0);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}
