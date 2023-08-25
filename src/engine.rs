use crate::{
    camera::Camera,
    input::{ElementState, Input, VirtualKeyCode},
    node_graph,
    ray_marcher::RayMarcher,
    renderer::{
        self,
        ebo::EBO,
        mesh::{self, primitives::Cube, Mesh},
        painter,
        shader::Shader,
        texture::Texture,
        vao::VAO,
        vbo::VBO,
        vert::Vert,
    },
    sdf::SDFBuilder,
    window::Window,
};
use egui::{self, Id};
use egui_winit;
use gl::{self};
use glutin::surface::GlSurface;
use nalgebra_glm;
use winit::{
    self,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

pub struct EngineState {
    pub graph: node_graph::NodeGraphExample,
    pub input: Input,

    pub ctx: egui::Context,
    pub egui_st: egui_winit::State,
    pub egui_painter: renderer::painter::Painter,

    pub resolution: [f32; 2],

    pub default_cube: mesh::primitives::Cube,
    pub default_texture: Texture,
    pub shader: Shader,

    pub ray_marcher: RayMarcher,

    pub time: std::time::SystemTime,
    pub itime: std::time::SystemTime,
    pub dt: std::time::Duration,
    pub camera: Camera,
    pub grid: Mesh,
    pub axes: Mesh,
    pub age: u32,
    pub fillet: f32,
}
impl EngineState {
    pub fn new(window: &Window) -> Self {
        let resolution = [
            window.window.inner_size().width as f32,
            window.window.inner_size().width as f32,
        ];
        Self {
            graph: node_graph::NodeGraphExample::new(),
            input: Input::new(),

            ctx: egui::Context::default(),
            egui_st: egui_winit::State::new(&window.window),
            egui_painter: renderer::painter::Painter::new(),

            resolution,

            default_cube: Cube::new(),
            default_texture: Texture::open("res/textures/default.tga").unwrap(),
            shader: Shader::from("res/shaders/hello.vert", "res/shaders/hello.frag"),

            ray_marcher: RayMarcher::new(),

            time: std::time::SystemTime::now(),
            itime: std::time::SystemTime::now(),
            dt: std::time::Duration::new(0, 0),
            camera: Camera::new(),
            grid: Mesh::new(),
            axes: Mesh::new(),
            age: 0,
            fillet: 1.0,
        }
    }
    pub fn setup(&mut self) {
        let sdf = SDFBuilder::new()
            .op_new(SDFBuilder::p_box::<String>(None, [0.4, 1.5, 0.5], 0.05))
            .op_diff_smooth(
                SDFBuilder::p_cylinder(
                    Some(SDFBuilder::rotate::<String>(None, [45.0, 0.0, 0.0])),
                    2.0,
                    0.25,
                    0.05,
                ),
                0.05,
            )
            .op_diff_smooth(
                SDFBuilder::p_box(
                    Some(SDFBuilder::translate(
                        Some(SDFBuilder::rotate::<String>(None, [45.0, 45.0, 0.0])),
                        [0.5, 0.5, 0.5],
                    )),
                    [0.5, 0.5, 0.5],
                    0.05,
                ),
                0.05,
            )
            .op_union_smooth(
                SDFBuilder::p_sphere(
                    Some(SDFBuilder::translate::<String>(None, [0.5, -0.5, -0.35])),
                    0.5,
                ),
                0.25,
            )
            .build();
        self.ray_marcher.shader =
            Shader::new(std::include_str!("../res/shaders/ray.vert"), sdf.as_str());
        println!("=============");
        println!("{}", sdf);
        println!("=============");
        self.camera.persp = nalgebra_glm::perspective_rh_no(
            self.resolution[0] / self.resolution[1],
            self.camera.fov.to_radians(),
            0.01,
            100.0,
        );
        self.ray_marcher.matcap.gen();
        self.ray_marcher.matcap.set_unit(gl::TEXTURE15);
        self.ray_marcher.matcap.bind().unwrap();
        self.ray_marcher.shader.enable();
        self.ray_marcher.shader.uniform_f32(
            "u_cam_zoom",
            (self.camera.pos - self.camera.look_at).magnitude(),
        );
        self.ray_marcher.shader.uniform_vec2(
            "u_resolution",
            self.resolution[0],
            self.resolution[1],
        );
        self.ray_marcher.shader.uniform_vec3v(
            "u_cam_translation",
            &(self.camera.pos - self.camera.look_at),
        );
        self.ray_marcher.shader.uniform_f32("u_fillet", self.fillet);

        self.default_texture.gen();
        self.default_texture.set_unit(gl::TEXTURE16);
        self.default_texture
            .bind()
            .expect("Need to call gen before binding");

        // build grid
        self.grid.verts = vec![
            Vert::from_pos_with_uv([-100.0, -100.0, 0.0], [0.0, 0.0]),
            Vert::from_pos_with_uv([100.0, -100.0, 0.0], [1.0, 0.0]),
            Vert::from_pos_with_uv([100.0, 100.0, 0.0], [1.0, 1.0]),
            Vert::from_pos_with_uv([-100.0, 100.0, 0.0], [0.0, 1.0]),
        ];
        self.grid.elements = vec![0, 1, 2, 0, 2, 3];

        self.axes.verts = vec![
            // z axis
            Vert::from_pos_with_col([-0.01, 0.0, 0.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.01, 0.0, 0.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.01, 0.0, 100.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([-0.01, 0.0, 100.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.0, -0.01, 0.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.0, 0.01, 0.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.0, 0.01, 100.0], [0.0, 0.5, 1.0]),
            Vert::from_pos_with_col([0.0, -0.01, 100.0], [0.0, 0.5, 1.0]),
            // y axis
            Vert::from_pos_with_col([0.0, 0.0, -0.01], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.0, 0.0, 0.01], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.0, 100.0, 0.01], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.0, 100.0, -0.01], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([-0.01, 0.0, 0.0], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.01, 0.0, 0.0], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.01, 100.0, 0.0], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([-0.01, 100.0, 0.0], [0.0, 1.0, 0.5]),
            Vert::from_pos_with_col([0.0, -0.01, 0.0], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([0.0, 0.01, 0.0], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([100.0, 0.01, 0.0], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([100.0, -0.01, 0.0], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([0.0, 0.0, -0.01], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([0.0, 0.0, 0.01], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([100.0, 0.0, 0.01], [1.0, 0.25, 0.25]),
            Vert::from_pos_with_col([100.0, 0.0, -0.01], [1.0, 0.25, 0.25]),
        ];
        self.axes.elements = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ];

        self.egui_st.set_max_texture_side(4096);
        self.egui_st
            .set_pixels_per_point(self.ctx.pixels_per_point());
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::MULTISAMPLE);
            gl::ClearColor(0.15, 0.15, 0.15, 1.0);
        }
    }
    pub fn handle_input(&mut self) {
        // Camera Controls
        match self.input.mouse.delta() {
            (x, y) => {
                if self.input.buttons.held(2)
                    && !self.input.keys.held(winit::event::VirtualKeyCode::LShift)
                {
                    let speed = 250.0;
                    self.camera.rotate_right(y * self.dt.as_secs_f32() * -speed);
                    self.camera.rotate_z(-x * self.dt.as_secs_f32() * speed);
                } else if self.input.buttons.held(2) {
                    let speed = 1.0;
                    self.camera.pan(
                        -x * self.dt.as_secs_f32() * speed,
                        y * self.dt.as_secs_f32() * speed,
                    );
                }
            }
        }
        match self.input.scroll.delta() {
            (_, y) if y.abs() < 0.001 => {}
            (_, y) => {
                self.camera.zoom(y / 10.0);
                self.ray_marcher.zoom += y / 10.0;
                self.ray_marcher.shader.enable();
                self.ray_marcher.shader.uniform_f32(
                    "u_cam_zoom",
                    (self.camera.pos - self.camera.look_at).magnitude(),
                );
            }
        }
        if self.input.keys.pressed(winit::event::VirtualKeyCode::C)
            && self.input.keys.held(winit::event::VirtualKeyCode::LShift)
        {
            self.camera.reset();
        }
        static mut WIREFRAME: bool = false;
        if self.input.keys.pressed(VirtualKeyCode::Z) {
            unsafe {
                WIREFRAME = !WIREFRAME;
                if WIREFRAME {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                } else {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }
        }
    }
    pub fn draw(&mut self) {
        self.camera.persp = nalgebra_glm::perspective_rh_no(
            self.resolution[0] / self.resolution[1],
            self.camera.fov.to_radians(),
            0.01,
            100.0,
        );
        //render ray march
        self.ray_marcher.shader.enable();
        self.ray_marcher
            .shader
            .uniform_mat3("u_cam_rot", &self.camera.rot);
        self.ray_marcher
            .shader
            .uniform_f32("u_time", self.itime.elapsed().unwrap().as_secs_f32());
        self.ray_marcher
            .shader
            .uniform_f32("u_fov", self.camera.fov);
        self.ray_marcher.shader.uniform_f32("u_fillet", self.fillet);
        self.ray_marcher
            .shader
            .uniform_mat4("view", &self.camera.view());
        self.ray_marcher
            .shader
            .uniform_mat4("persp", &self.camera.persp);
        self.ray_marcher.shader.uniform_vec3v(
            "u_cam_translation",
            &(self.camera.look_at - nalgebra::Point3::new(0.0, 0.0, 0.0)),
        );
        Texture::set_active_unit(gl::TEXTURE15);
        self.ray_marcher.matcap.bind().unwrap();
        self.ray_marcher.shader.uniform_tex("u_matcap", 15);
        //unsafe { gl::Disable(gl::DEPTH_TEST); }
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        unsafe {
            gl::DepthMask(gl::TRUE);
        }
        renderer::render_mesh(&self.ray_marcher.mesh);
        //unsafe { gl::DepthMask(gl::TRUE); }
        //unsafe { gl::Enable(gl::DEPTH_TEST); }

        //render cube
        self.shader.enable();
        self.shader.uniform_mat4("view", &self.camera.view());
        self.shader.uniform_mat4("persp", &self.camera.persp);
        self.shader
            .uniform_f32("u_time", self.itime.elapsed().unwrap().as_secs_f32());
        self.grid.verts.iter().for_each(|v| {
            //println!("World Space: {:?}", nalgebra::Vector4::new(v.pos.x, v.pos.y, v.pos.z, 1.0));
            //println!("View Space: {:?}", nalgebra_glm::perspective_rh_no(self.resolution[0]/self.resolution[1], self.camera.fov.to_radians(), 0.01, 100.0) * self.camera.view() * nalgebra::Vector4::new(v.pos.x, v.pos.y, v.pos.z, 1.0));
            let vndc = nalgebra_glm::perspective_rh_no(
                self.resolution[0] / self.resolution[1],
                self.camera.fov.to_radians(),
                0.01,
                100.0,
            ) * self.camera.view()
                * nalgebra::Vector4::new(v.pos.x, v.pos.y, v.pos.z, 1.0);
            //println!("Clip Space: {:?}", nalgebra_glm::perspective_rh_no(self.resolution[0]/self.resolution[1], self.camera.fov.to_radians(), 0.01, 100.0) * self.camera.view() * nalgebra::Vector4::new(v.pos.x, v.pos.y, v.pos.z, 1.0));
            //println!("vndc: {:?}", vndc.xyz() / vndc.w);
        });

        Texture::set_active_unit(gl::TEXTURE16);
        let _ = self.default_texture.bind();
        self.shader.uniform_tex("default_tex", 0);
        //renderer::render_mesh(&self.default_cube.mesh);

        // render grid
        let grid_shader = Shader::from("res/shaders/hello.vert", "res/shaders/grid.frag");
        grid_shader.enable();
        grid_shader.uniform_mat4("view", &self.camera.view());
        grid_shader.uniform_mat4("persp", &self.camera.persp);
        grid_shader.uniform_f32("dist", self.camera.dist());
        renderer::render_mesh(&self.grid);
        renderer::render_mesh(&self.axes);
    }
    pub fn update(&mut self) {}
    pub fn run(&mut self, window: &Window, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => {
                let _event_response = self.egui_st.on_event(&self.ctx, &event);
                match event {
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        control_flow.set_exit();
                    }
                    WindowEvent::Resized(size) => {
                        window.gl_surface.resize(
                            &window.gl_context,
                            std::num::NonZeroU32::new(size.width).unwrap(),
                            std::num::NonZeroU32::new(size.height).unwrap(),
                        );
                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }
                        self.resolution[0] = size.width as f32;
                        self.resolution[1] = size.height as f32;
                        self.camera.persp = nalgebra_glm::perspective_rh_no(
                            self.resolution[0] / self.resolution[1],
                            self.camera.fov,
                            0.01,
                            100.0,
                        );
                        self.egui_painter.shader.enable();
                        self.egui_painter.shader.uniform_vec2(
                            "u_screen_size",
                            size.width as f32,
                            size.height as f32,
                        );
                        self.shader.enable();
                        self.shader.uniform_vec2(
                            "u_resolution",
                            size.width as f32,
                            size.height as f32,
                        );
                        self.ray_marcher.shader.enable();
                        self.ray_marcher.shader.uniform_vec2(
                            "u_resolution",
                            size.width as f32,
                            size.height as f32,
                        );
                        println!("{} x {}", size.width as f32, size.height as f32);
                    }
                    _ => {
                        // do nothing
                        // ;
                    }
                }
            }
            Event::DeviceEvent {
                device_id: _,
                event,
            } => {
                match event {
                    winit::event::DeviceEvent::Key(winit::event::KeyboardInput {
                        scancode: _,
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    }) => {
                        // Map keys
                        if let Some((current_state, _)) = self.input.keys.0.get(&keycode) {
                            self.input.keys.0.insert(keycode, (state, *current_state));
                        } else {
                            self.input
                                .keys
                                .0
                                .insert(keycode, (state, ElementState::Released));
                        }
                    }
                    winit::event::DeviceEvent::Button { button: id, state } => {
                        // Map buttons
                        if let Some((current_state, _)) = self.input.buttons.0.get(&id) {
                            self.input.buttons.0.insert(id, (state, *current_state));
                        } else {
                            self.input
                                .buttons
                                .0
                                .insert(id, (state, ElementState::Released));
                        }
                    }
                    winit::event::DeviceEvent::MouseWheel { delta } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            self.input.scroll.set(x, y);
                        }
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            self.input.scroll.set(pos.x as f32, pos.y as f32);
                        }
                    },
                    winit::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
                        self.input.mouse.set(x as f32, y as f32);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                self.time = std::time::SystemTime::now();
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::Clear(gl::DEPTH_BUFFER_BIT);
                }

                self.handle_input();
                self.draw();

                self.egui_painter.shader.enable();
                self.egui_painter
                    .shader
                    .uniform_mat4("view", &self.camera.view());
                self.egui_painter
                    .shader
                    .uniform_mat4("persp", &self.camera.persp);
                let raw_input = self.egui_st.take_egui_input(&window.window);
                let full_output = self.ctx.run(raw_input, |ctx| {
                    self.graph.update(ctx);
                    egui::Window::new("Outliner")
                        .resizable(true)
                        .show(&ctx, |ui| {
                            ui.label("Hello world!");
                            let sdt = format!(
                                "Milliseconds: {}\n FPS: {}",
                                self.dt.as_secs_f32() * 1000.0,
                                1.0 / (self.dt.as_secs_f32())
                            );
                            ui.label(sdt);
                            if ui.button("Click me").clicked() {
                                // take some action here
                                println!("Clicked");
                            }
                            ui.add(
                                egui::Slider::new(&mut self.camera.fov, 1.0..=120.0).text("age"),
                            );
                            ui.add(egui::Slider::new(&mut self.fillet, -2.0..=2.0).text("fillet"));
                            static mut MY_BOOL: bool = false;
                            unsafe {
                                ui.add(egui::Checkbox::new(&mut MY_BOOL, "Checked"));
                            }
                        });
                });
                self.egui_st.handle_platform_output(
                    &window.window,
                    &self.ctx,
                    full_output.platform_output,
                );
                let clipped_primitives = self.ctx.tessellate(full_output.shapes); // create triangles to paint

                self.egui_painter
                    .paint(&clipped_primitives, &full_output.textures_delta);

                let _ = window.gl_surface.swap_buffers(&window.gl_context);
                window.window.request_redraw();
                self.dt = self.time.elapsed().unwrap();
                self.egui_st.take_egui_input(&window.window);
            }
            _ => (),
        }
    }
}
