use super::vert::Vert;
pub mod primitives;

pub struct Mesh {
    pub elements: Vec<u32>,
    pub verts: Vec<Vert>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            verts: Vec::new(),
        }
    }
}
