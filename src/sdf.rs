pub enum Shading {
    Matcap,
    Diffuse,
    Pos,
    Normal,
    TangentSpaceNormal,
}

pub struct SDFBuilder { 
    prelude: String,
    declarations: String,
    definitions: String,
    scene: String,
    main: String,
    
}

impl SDFBuilder {
    pub fn new() -> Self {
        let mut definitions = std::include_str!("sdf/strings/srgb.frag").to_string();
        definitions.push_str(std::include_str!("sdf/strings/math.frag"));
        definitions.push_str(std::include_str!("sdf/strings/space.frag"));
        definitions.push_str(std::include_str!("sdf/strings/primitives.frag"));
        definitions.push_str(std::include_str!("sdf/strings/ops.frag"));
        definitions.push_str(std::include_str!("sdf/strings/ray.frag"));
        definitions.push_str(std::include_str!("sdf/strings/camera.frag"));
        Self {
        prelude: std::include_str!("sdf/strings/prelude.frag").to_string(),
        declarations: std::include_str!("sdf/strings/declarations.frag").to_string(),
        definitions,
        scene: r#"float scene(vec3 p) {"#.to_string(),
        main: std::include_str!("sdf/strings/main.frag").to_string(),
        }
    }
    pub fn translate<S: std::fmt::Display>(position: Option<S>, translation: [f32; 3]) -> String {
        match position {
            None => format!("translate(p, vec3({}, {}, {}))", translation[0], translation[1], translation[2]),
            Some(pos) => format!("translate({}, vec3({}, {}, {}))", pos, translation[0], translation[1], translation[2]),
        }
    }
    pub fn rotate<S: std::fmt::Display>(position: Option<S>, rotation: [f32; 3]) -> String {
        match position {
            None => format!("rotate(p, vec3({}, {}, {}))", rotation[0], rotation[1], rotation[2]),
            Some(pos) => format!("rotate({}, vec3({}, {}, {}))", pos, rotation[0], rotation[1], rotation[2]),
        }
    }
    pub fn p_box<S: std::fmt::Display>(position: Option<S>, size: [f32; 3], fillet: f32) -> String {
        match position {
            None => format!("sdf_box(p, vec3({}, {}, {})) - {}", size[0] - fillet, size[1] - fillet, size[2] - fillet, fillet),
            Some(pos) => format!("sdf_box({}, vec3({}, {}, {})) - {}", pos, size[0] - fillet, size[1] - fillet, size[2] - fillet, fillet),
        }
    }
    pub fn p_cylinder<S: std::fmt::Display>(position: Option<S>, height: f32, radius: f32, fillet: f32) -> String {
        match position {
            None => format!("sdf_cylinder(p, {}, {}) - {}", height - fillet, radius - fillet, fillet),
            Some(pos) => format!("sdf_cylinder({}, {}, {}) - {}", pos, height - fillet, radius - fillet, fillet),
        }
    }
    pub fn p_sphere<S: std::fmt::Display>(position: Option<S>, radius: f32) -> String {
        match position {
            None => format!("sdf_sphere(p, {})", radius),
            Some(pos) => format!("sdf_sphere({}, {})", pos, radius),
        }
    }
    pub fn op_new<S: std::fmt::Display>(mut self, operand: S) -> Self {
        self.scene = format!("{}", operand);
        self
    }
    pub fn op_union<S: std::fmt::Display>(mut self, operand: S) -> Self {
        self.scene = format!("op_union({}, {})", self.scene, operand);
        self
    }
    pub fn op_diff<S: std::fmt::Display>(mut self, operand: S) -> Self {
        self.scene = format!("op_diff({}, {})", self.scene, operand);
        self
    }
    pub fn op_int<S: std::fmt::Display>(mut self, operand: S) -> Self {
        self.scene = format!("op_int({}, {})", self.scene, operand);
        self
    }
    pub fn op_union_smooth<S: std::fmt::Display>(mut self, operand: S, smooth: f32) -> Self {
        self.scene = format!("op_union_smooth({}, {}, {})", self.scene, operand, smooth);
        self
    }
    pub fn op_diff_smooth<S: std::fmt::Display>(mut self, operand: S, smooth: f32) -> Self {
        self.scene = format!("op_diff_smooth({}, {}, {})", self.scene, operand, smooth);
        self
    }
    pub fn op_int_smooth<S: std::fmt::Display>(mut self, operand: S, smooth: f32) -> Self {
        self.scene = format!("op_int_smooth({}, {}, {})", self.scene, operand, smooth);
        self
    }
    pub fn build(&self) -> String {
        let mut ans = self.prelude.to_owned();
        ans.push_str(self.declarations.as_str());
        ans.push_str(self.definitions.as_str());
        ans.push_str("float scene(vec3 p) {\n    return ");
        ans.push_str(self.scene.as_str());
        ans.push_str(";}\n\n");
        ans.push_str(self.main.as_str());

        ans
    }
}
