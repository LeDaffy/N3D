pub mod ebo;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod vbo;
pub mod vert;

pub mod painter;

use egui;
use gl::{self};

pub fn render_mesh(mesh: &mesh::Mesh) {
    let vao = vao::VAO::new();
    let vbo = vbo::VBO::new();
    let ebo = ebo::EBO::new();

    vao.bind();
    vbo.bind();
    vbo.set_buffer_data(&mesh.verts);
    ebo.bind();
    ebo.set_buffer_data(&mesh.elements);

    vao.set_attributes();

    vao.bind();
    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.elements.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }
    vao.unbind();
    vbo.unbind();
    ebo.unbind();
}

pub fn render_egui_mesh(mesh: &egui::Mesh) {
    let vao = vao::VAO::new();
    let vbo = vbo::VBO::new();
    let ebo = ebo::EBO::new();

    vao.bind();
    vbo.bind();
    vbo.set_egui_data(&mesh.vertices);
    ebo.bind();
    ebo.set_buffer_data(&mesh.indices);
    vao.set_egui_attributes();
    vao.unbind();
    vbo.unbind();
    ebo.unbind();
    vao.bind();
    unsafe {
        // gl::Clear(gl::DEPTH_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.indices.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        //gl::Clear(gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
    }

}
