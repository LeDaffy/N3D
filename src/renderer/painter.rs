use super::shader::Shader;
use crate::renderer::{
    self,
    texture::{Texture, TextureBuffer, TextureFormat},
};

use egui::{
    epaint::{Color32, FontImage},
    ClippedPrimitive, TexturesDelta, TextureId,
};

pub struct Painter {
    texture: Option<Vec<Texture>>,
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

        textures.set.iter().for_each(|(id, image_delta)| {
            println!("new tex");
            match &image_delta.image {
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
                    if let None = self.texture {
                        self.texture = Some(Vec::new());
                    }
                    if let Some(ref mut tex) = self.texture {
                        tex.push(Texture::new(
                                renderer::texture::TextureFormat::Rgba,
                                buf,
                                dimensions,
                                ));
                        println!("num textures {}", tex.len());
                        match id {
                            egui::TextureId::Managed(id) => {
                                let num_textures = tex.len() as u32;
                                println!("Matched managed texture");
                                println!("num textures {}", num_textures);
                                tex.last_mut().unwrap().gen();
                                println!("Setting texture unit {}", *id);

                                tex.last_mut().unwrap().set_unit(gl::TEXTURE0 + num_textures - 1);
                                tex.last_mut().unwrap().bind().expect("Need to call gen before binding");
                                self.shader.enable();
                                self.shader.uniform_tex("fonts", num_textures as i32 - 1);
                                println!("Painter Tex: {:?}", tex.last_mut().unwrap().id);
                            }
                            egui::TextureId::User(id) => {
                                println!("Matched managed texture");
                                let num_textures = tex.len() as u32;
                                tex.last_mut().unwrap().gen();
                                tex.last_mut().unwrap().set_unit(gl::TEXTURE0 + num_textures - 1);
                                tex.last_mut().unwrap().bind().expect("Need to call gen before binding");
                                self.shader.enable();
                                self.shader.uniform_tex("fonts", *id as i32);
                                println!("Painter Tex: {:?}", tex.last_mut().unwrap().id);
                            }
                        }
                    }
                }
            }
        });
        meshes.iter().for_each(|m| {
            self.shader.enable();
            if let TextureId::Managed(id) = m.texture_id {
                if id != 0 {
                    println!("Id: {}", id);
                }
                self.shader.uniform_tex("fonts", id as i32);
            } else {
                self.shader.uniform_tex("fonts", 0);
            }
            self.shader.enable();
            renderer::render_egui_mesh(m);
        });
        //println!(
        //    "meshes {:?}",
        //    meshes
        //        .iter()
        //        .map(|m| m
        //            .vertices
        //            .iter()
        //            .map(|v| v.uv)
        //            .collect::<Vec<egui::epaint::Pos2>>())
        //        .flatten()
        //        .collect::<Vec<egui::epaint::Pos2>>()
        //);
        if textures.free.len() > 0 {
            println!("tex free len {:?}", textures.free.len());
        }
        if textures.set.len() > 0 {
            println!("tex set len {:?}", textures.set.len());
        }
        //println!(
        //    "tex id {:?}",
        //    textures
        //        .set
        //        .iter()
        //        .map(|t| t.0)
        //        .collect::<Vec<egui::epaint::TextureId>>()
        //);
    }
}
