use super::Mesh;
use crate::renderer::vert::Vert;

pub struct Cube {
    pub mesh: Mesh,
}
impl Cube {
    pub fn new() -> Self {
        let mut mesh = Mesh::new();
        mesh.verts = vec![
            Vert::from_pos_with_uv([-1.0, -1.0, -1.0], [0.0, 0.0]),
            Vert::from_pos_with_uv([1.0, -1.0, -1.0], [1.0, 0.0]),
            Vert::from_pos_with_uv([1.0, -1.0, 1.0], [1.0, 1.0]),
            Vert::from_pos_with_uv([-1.0, -1.0, 1.0], [0.0, 1.0]),

            Vert::from_pos_with_uv([1.0, 1.0, 1.0], [0.0, 0.0]),
            Vert::from_pos_with_uv([-1.0, 1.0, 1.0], [1.0, 0.0]),
            Vert::from_pos_with_uv([1.0, 1.0, -1.0], [1.0, 1.0]),
            Vert::from_pos_with_uv([-1.0, 1.0, -1.0], [0.0, 1.0]),
        ];
        mesh.elements = vec![
            0, 1, 2, 0, 2, 3, 2, 3, 4, 4, 3, 5, 4, 5, 6, 5, 6, 7, 1, 2, 4, 1, 4, 6,
        ];
        Self { mesh }
    }
}
