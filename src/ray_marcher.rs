use crate::renderer::{mesh::Mesh, shader::Shader, vert::Vert, texture::Texture};

pub struct RayMarcher {
    /// Plane which ray marching is drawn to
    pub mesh: Mesh,
    /// Ray marching shader
    pub shader: Shader,
    /// Uniform camera zoom sent to shader
    pub zoom: f32,
    pub matcap: Texture,
}

impl RayMarcher {
    pub fn new() -> Self {
        let mut mesh = Mesh::new();
        mesh.verts = vec![Vert::from_pos_with_uv([-1.0, -1.0, 0.0], [0.0, 0.0]),
                          Vert::from_pos_with_uv([ 1.0, -1.0, 0.0], [1.0, 0.0]),
                          Vert::from_pos_with_uv([ 1.0,  1.0, 0.0], [1.0, 1.0]),
                          Vert::from_pos_with_uv([-1.0,  1.0, 0.0], [0.0, 1.0])];
        mesh.elements = vec![0, 1, 2, 0, 2, 3];

        Self {
            mesh,
            shader: Shader::from("res/shaders/ray.vert", "res/shaders/ray.frag"),
            zoom: 1.0,
            matcap: Texture::open("res/matcap/metal_shiny.tga").unwrap(),
            //matcap: Texture::open("res/matcap/jade.tga").unwrap(),
            //matcap: Texture::open("res/matcap/metal_carpaint.tga").unwrap(),
            //matcap: Texture::open("res/matcap/reflection_check_horizontal.tga").unwrap(),
        }
    }
}
