use super::shader::Shader;
use crate::renderer::{
    self,
    texture::{Texture, TextureBuffer, TextureFormat},
};

use egui::{
    epaint::{Color32, FontImage},
    ClippedPrimitive, TexturesDelta,
};

pub struct Painter {
    texture: Option<Texture>,
    pub shader: Shader,
}

impl Painter {
    pub fn new() -> Self {
        Self {
            texture: None,
            shader: Shader::from("res/shaders/egui.vert", "res/shaders/egui.frag"),
        }
    }
    pub fn paint(&mut self, primitives: &Vec<ClippedPrimitive>, textures: &TexturesDelta) {
        let meshes: Vec<egui::epaint::Mesh> = primitives
            .iter()
            .filter_map(|c| match &c.primitive {
                egui::epaint::Primitive::Mesh(mesh) => Some(mesh.to_owned()),
                _ => None,
            })
            .collect();

        textures
            .set
            .iter()
            .for_each(|(id, image_delta)| match &image_delta.image {
                epaint::image::ImageData::Color(_color) => {
                    println!("Matched color");
                }
                epaint::image::ImageData::Font(luminance) => {
                    println!("Matched fonts");
                    let dimensions = (luminance.width(), luminance.height());
                    let buf: TextureBuffer = TextureBuffer::Byte(
                        luminance
                            .srgba_pixels(None)
                            .map(|c| [c.r(), c.g(), c.b(), c.a()])
                            .flatten()
                            .collect::<Vec<u8>>(),
                    );
                    self.texture = Some(Texture::new(
                        renderer::texture::TextureFormat::Rgba,
                        buf,
                        dimensions,
                    ));
                    if let Some(ref mut tex) = self.texture {
                        match id {
                            egui::TextureId::Managed(id) => {
                                println!("Matched managed texture");
                                tex.gen();
                                tex.set_unit(gl::TEXTURE0 + (*id as u32));
                                tex.bind().expect("Need to call gen before binding");
                                self.shader.enable();
                                self.shader.uniform_tex("fonts", *id as i32);
                                println!("Painter Tex: {:?}", tex.id);
                            }
                            egui::TextureId::User(id) => {
                                println!("Matched user texture");
                                tex.gen();
                                tex.set_unit(gl::TEXTURE0 + (*id as u32));
                                tex.bind().expect("Need to call gen before binding");
                                self.shader.enable();
                                self.shader.uniform_tex("fonts", *id as i32);
                                println!("Painter Tex: {:?}", tex.id);
                            }
                        }
                    }
                }
            });
        meshes.iter().for_each(|m| {
            self.shader.enable();
            renderer::render_egui_mesh(m);
        });
        static mut ONCE: bool = true;
        unsafe {
            if ONCE {
                println!(
                    "meshes {:?}",
                    meshes
                        .iter()
                        .map(|m| m
                            .vertices
                            .iter()
                            .map(|v| v.uv)
                            .collect::<Vec<egui::epaint::Pos2>>())
                        .flatten()
                        .collect::<Vec<egui::epaint::Pos2>>()
                );
                println!("tex set len {:?}", textures.set.len());
                println!(
                    "tex id {:?}",
                    textures
                        .set
                        .iter()
                        .map(|t| t.0)
                        .collect::<Vec<egui::epaint::TextureId>>()
                );
            }
            ONCE = false;
        }
    }
}
