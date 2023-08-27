use std::{borrow::Cow, collections::HashMap};

use egui::{self, DragValue, TextStyle};
use egui_node_graph::*;

use crate::{renderer::shader::Shader, sdf::SDFBuilder};

type MyGraph = Graph<N3DNodeData, N3DDataType, N3DValueType>;
type MyEditorState =
    GraphEditorState<N3DNodeData, N3DDataType, N3DValueType, N3DNodeTemplate, MyGraphState>;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct N3DNodeData {
    template: N3DNodeTemplate,
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum N3DDataType {
    Scalar,
    Vec2,
    Vec3,
    SDFPosition,
    SDFVolume,
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum N3DValueType {
    Scalar { value: f32 },
    Vec2 { value: nalgebra::Vector2<f32> },
    Vec3 { value: nalgebra::Vector3<f32> },
    SDFPosition { value: String },
    SDFVolume { value: String },
}

impl Default for N3DValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Scalar { value: 0.0 }
    }
}

impl N3DValueType {
    /// Tries to downcast this value type to a vector
    pub fn try_to_vec2(self) -> anyhow::Result<nalgebra::Vector2<f32>> {
        if let N3DValueType::Vec2 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to vec2", self)
        }
    }
    pub fn try_to_vec3(self) -> anyhow::Result<nalgebra::Vector3<f32>> {
        if let N3DValueType::Vec3 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to vec3", self)
        }
    }
    pub fn try_to_scalar(self) -> anyhow::Result<f32> {
        if let N3DValueType::Scalar { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to scalar", self)
        }
    }
    pub fn try_to_sdf_position(self) -> anyhow::Result<String> {
        if let N3DValueType::SDFPosition { value } = self {
            Ok(value.clone())
        } else {
            anyhow::bail!("Invalid cast from {:?} to SDFPosition", self)
        }
    }
    pub fn try_to_sdf_volume(self) -> anyhow::Result<String> {
        if let N3DValueType::SDFVolume { value } = self {
            Ok(value.clone())
        } else {
            anyhow::bail!("Invalid cast from {:?} to SDFValue", self)
        }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum N3DNodeTemplate {
    NewScalar,
    ScalarAdd,
    ScalarSub,
    NewVec2,
    Vec2Add,
    Vec2Subtract,
    Vec2ScalarMul,
    NewVec3,
    Vec3Add,
    Vec3Sub,
    Vec3Dot,
    Vec3Cross,
    SDFPosition,
    SDFTranslate,
    SDFBox,
    SDFUnion,
    SDFViewer,
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct MyGraphState {
    pub active_node: Option<NodeId>,
    pub shader: Shader, 
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<MyGraphState> for N3DDataType {
    fn data_type_color(&self, _user_state: &mut MyGraphState) -> egui::Color32 {
        match self {
            N3DDataType::Scalar => egui::Color32::from_rgb(38, 109, 211),
            N3DDataType::Vec2 => egui::Color32::from_rgb(238, 207, 109),
            N3DDataType::Vec3 => egui::Color32::from_rgb(148, 255, 0),
            N3DDataType::SDFPosition => egui::Color32::from_rgb(99, 99, 199),
            N3DDataType::SDFVolume => egui::Color32::from_rgb(247, 37, 133),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            N3DDataType::Scalar => Cow::Borrowed("scalar"),
            N3DDataType::Vec2 => Cow::Borrowed("vec2"),
            N3DDataType::Vec3 => Cow::Borrowed("vec3"),
            N3DDataType::SDFPosition => Cow::Borrowed("SDF position"),
            N3DDataType::SDFVolume => Cow::Borrowed("SDF Volume"),
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for N3DNodeTemplate {
    type NodeData = N3DNodeData;
    type DataType = N3DDataType;
    type ValueType = N3DValueType;
    type UserState = MyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            N3DNodeTemplate::NewScalar => "New scalar",
            N3DNodeTemplate::ScalarAdd => "Scalar add",
            N3DNodeTemplate::ScalarSub => "Scalar subtract",
            N3DNodeTemplate::NewVec2 => "New Vec2",
            N3DNodeTemplate::Vec2Add => "Vec2 Add",
            N3DNodeTemplate::Vec2Subtract => "Vec2 Subtract",
            N3DNodeTemplate::Vec2ScalarMul => "Vec2 Scalar Multiply",
            N3DNodeTemplate::NewVec3 => "New Vec3",
            N3DNodeTemplate::Vec3Add => "Vec3 Add",
            N3DNodeTemplate::Vec3Sub => "Vec3 Sub",
            N3DNodeTemplate::Vec3Dot => "Vec3 Dot",
            N3DNodeTemplate::Vec3Cross => "Vec3 Cross",
            N3DNodeTemplate::SDFPosition => "SDF Position",
            N3DNodeTemplate::SDFTranslate => "SDF Translate",
            N3DNodeTemplate::SDFBox => "SDF Box",
            N3DNodeTemplate::SDFUnion => "SDF Union",
            N3DNodeTemplate::SDFViewer => "SDF Viewer",
        })
    }

    // this is what allows the library to show collapsible lists in the node finder.
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        match self {
            N3DNodeTemplate::NewScalar
            | N3DNodeTemplate::ScalarAdd
            | N3DNodeTemplate::ScalarSub => vec!["Scalar"],
            N3DNodeTemplate::NewVec2 | N3DNodeTemplate::Vec2Add | N3DNodeTemplate::Vec2Subtract => {
                vec!["Vec2"]
            }
            N3DNodeTemplate::Vec2ScalarMul => vec!["Vec2", "Scalar"],
            N3DNodeTemplate::NewVec3
            | N3DNodeTemplate::Vec3Add
            | N3DNodeTemplate::Vec3Sub
            | N3DNodeTemplate::Vec3Dot
            | N3DNodeTemplate::Vec3Cross => vec!["Vec3"],
            N3DNodeTemplate::SDFPosition
            | N3DNodeTemplate::SDFTranslate
            | N3DNodeTemplate::SDFBox
            | N3DNodeTemplate::SDFUnion
            | N3DNodeTemplate::SDFViewer => vec!["SDF"],
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        N3DNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_scalar = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                N3DDataType::Scalar,
                N3DValueType::Scalar { value: 0.0 },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let output_scalar = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), N3DDataType::Scalar);
        };
        let input_sdf_position = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                N3DDataType::SDFPosition,
                N3DValueType::SDFPosition { value: "p".to_string() },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let output_sdf_position = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), N3DDataType::SDFPosition);
        };
        let input_sdf_volume = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                N3DDataType::SDFVolume,
                N3DValueType::SDFVolume { value: "".to_string() },
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let output_sdf_volume = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), N3DDataType::SDFVolume);
        };
        let input_vec2 = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                N3DDataType::Vec2,
                N3DValueType::Vec2 {
                    value: nalgebra::Vector2::new(0.0, 0.0),
                },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let output_vec2 = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), N3DDataType::Vec2);
        };
        let input_vec3 = |graph: &mut MyGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                N3DDataType::Vec3,
                N3DValueType::Vec3 {
                    value: nalgebra::Vector3::new(0.0, 0.0, 0.0),
                },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let output_vec3 = |graph: &mut MyGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), N3DDataType::Vec3);
        };

        match self {
            N3DNodeTemplate::ScalarAdd => {
                // The first input param doesn't use the closure so we can comment
                // it in more detail.
                graph.add_input_param(
                    node_id,
                    // This is the name of the parameter. Can be later used to
                    // retrieve the value. Parameter names should be unique.
                    "A".into(),
                    // The data type for this input. In this case, a scalar
                    N3DDataType::Scalar,
                    // The value type for this input. We store zero as default
                    N3DValueType::Scalar { value: 0.0 },
                    // The input parameter kind. This allows defining whether a
                    // parameter accepts input connections and/or an inline
                    // widget to set its value.
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
                input_scalar(graph, "B");
                output_scalar(graph, "out");
            }
            N3DNodeTemplate::ScalarSub => {
                input_scalar(graph, "A");
                input_scalar(graph, "B");
                output_scalar(graph, "out");
            }
            N3DNodeTemplate::Vec2ScalarMul => {
                input_scalar(graph, "scalar");
                input_vec2(graph, "vector");
                output_vec2(graph, "out");
            }
            N3DNodeTemplate::Vec2Add => {
                input_vec2(graph, "v1");
                input_vec2(graph, "v2");
                output_vec2(graph, "out");
            }
            N3DNodeTemplate::Vec2Subtract => {
                input_vec2(graph, "v1");
                input_vec2(graph, "v2");
                output_vec2(graph, "out");
            }
            N3DNodeTemplate::NewVec2 => {
                input_scalar(graph, "x");
                input_scalar(graph, "y");
                output_vec2(graph, "out");
            }
            N3DNodeTemplate::NewScalar => {
                input_scalar(graph, "value");
                output_scalar(graph, "out");
            }
            N3DNodeTemplate::NewVec3 => {
                input_scalar(graph, "x");
                input_scalar(graph, "y");
                input_scalar(graph, "z");
                output_vec3(graph, "out");
            }
            N3DNodeTemplate::Vec3Add => {
                input_vec3(graph, "v1");
                input_vec3(graph, "v2");
                output_vec3(graph, "out");
            }
            N3DNodeTemplate::Vec3Sub => {
                input_vec3(graph, "v1");
                input_vec3(graph, "v2");
                output_vec3(graph, "out");
            }
            N3DNodeTemplate::Vec3Dot => {
                input_vec3(graph, "v1");
                input_vec3(graph, "v2");
                output_scalar(graph, "out");
            }
            N3DNodeTemplate::Vec3Cross => {
                input_vec3(graph, "v1");
                input_vec3(graph, "v2");
                output_vec3(graph, "out");
            }
            N3DNodeTemplate::SDFPosition => {
                output_sdf_position(graph, "out");
            }
            N3DNodeTemplate::SDFTranslate => {
                input_vec3(graph, "translation");
                input_sdf_position(graph, "sdf position");
                output_sdf_position(graph, "out");
            }
            N3DNodeTemplate::SDFBox => {
                input_vec3(graph, "dimensions");
                input_scalar(graph, "fillet");
                input_sdf_position(graph, "sdf position");
                output_sdf_volume(graph, "out");
            }
            N3DNodeTemplate::SDFUnion => {
                input_sdf_volume(graph, "sdf 1");
                input_sdf_volume(graph, "sdf 2");
                output_sdf_volume(graph, "out");
            }
            N3DNodeTemplate::SDFViewer => {
                input_sdf_volume(graph, "sdf");
            }
        }
    }
}

pub struct AllN3DNodeTemplates;
impl NodeTemplateIter for AllN3DNodeTemplates {
    type Item = N3DNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            N3DNodeTemplate::NewScalar,
            N3DNodeTemplate::NewVec2,
            N3DNodeTemplate::ScalarAdd,
            N3DNodeTemplate::ScalarSub,
            N3DNodeTemplate::Vec2Add,
            N3DNodeTemplate::Vec2Subtract,
            N3DNodeTemplate::Vec2ScalarMul,
            N3DNodeTemplate::NewVec3,
            N3DNodeTemplate::Vec3Add,
            N3DNodeTemplate::Vec3Sub,
            N3DNodeTemplate::Vec3Dot,
            N3DNodeTemplate::Vec3Cross,
            N3DNodeTemplate::SDFPosition,
            N3DNodeTemplate::SDFTranslate,
            N3DNodeTemplate::SDFBox,
            N3DNodeTemplate::SDFUnion,
            N3DNodeTemplate::SDFViewer,
        ]
    }
}

impl WidgetValueTrait for N3DValueType {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type NodeData = N3DNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut MyGraphState,
        _node_data: &N3DNodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            N3DValueType::Vec2 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value.x).speed(0.1));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value.y).speed(0.1));
                });
            }
            N3DValueType::Vec3 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value.x).speed(0.1));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value.y).speed(0.1));
                    ui.label("z");
                    ui.add(DragValue::new(&mut value.z).speed(0.1));
                });
            }
            N3DValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value).speed(0.1));
                });
            }
            N3DValueType::SDFPosition { value: _ } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
            N3DValueType::SDFVolume { value: _ } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}
impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for N3DNodeData {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type DataType = N3DDataType;
    type ValueType = N3DValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<N3DNodeData, N3DDataType, N3DValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, N3DNodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("[ ] Set active").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("[x] Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::LIGHT_GREEN);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }

        responses
    }
}

#[derive(Default)]
pub struct NodeGraphExample {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: MyEditorState,

    user_state: MyGraphState,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "egui_node_graph";

impl NodeGraphExample {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new() -> Self {
        Self {
            state: MyEditorState::default(),
            user_state: MyGraphState::default(),
        }
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    pub fn update(&mut self, ctx: &egui::Context) -> Option<String> {
        let mut ret_val: Option<String> = None;
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        let graph_response = egui::TopBottomPanel::bottom("node_panel")
            .resizable(true)
            //.min_height(256.0)
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    AllN3DNodeTemplates,
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let text = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => {
                        if let N3DValueType::SDFVolume { ref value } = value {
                            ret_val = Some(value.clone());
                        }
                        format!("The result is: {:?}", value)
                    },
                    Err(err) => format!("Execution error: {}", err),
                };
                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            } else {
                self.user_state.active_node = None;
            }
        }
    ret_val
    }
}

type OutputsCache = HashMap<OutputId, N3DValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<N3DValueType> {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    struct Evaluator<'a> {
        graph: &'a MyGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(graph: &'a MyGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }
        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<N3DValueType> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }
        fn populate_output(
            &mut self,
            name: &str,
            value: N3DValueType,
        ) -> anyhow::Result<N3DValueType> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }
        fn input_vec2(&mut self, name: &str) -> anyhow::Result<nalgebra::Vector2<f32>> {
            self.evaluate_input(name)?.try_to_vec2()
        }
        fn output_vec2(
            &mut self,
            name: &str,
            value: nalgebra::Vector2<f32>,
        ) -> anyhow::Result<N3DValueType> {
            self.populate_output(name, N3DValueType::Vec2 { value })
        }
        fn input_vec3(&mut self, name: &str) -> anyhow::Result<nalgebra::Vector3<f32>> {
            self.evaluate_input(name)?.try_to_vec3()
        }
        fn output_vec3(
            &mut self,
            name: &str,
            value: nalgebra::Vector3<f32>,
        ) -> anyhow::Result<N3DValueType> {
            self.populate_output(name, N3DValueType::Vec3 { value })
        }
        fn input_scalar(&mut self, name: &str) -> anyhow::Result<f32> {
            self.evaluate_input(name)?.try_to_scalar()
        }
        fn output_scalar(&mut self, name: &str, value: f32) -> anyhow::Result<N3DValueType> {
            self.populate_output(name, N3DValueType::Scalar { value })
        }
        fn input_sdf_position(&mut self, name: &str) -> anyhow::Result<String> {
            self.evaluate_input(name)?.try_to_sdf_position()
        }
        fn output_sdf_position(&mut self, name: &str, value: String) -> anyhow::Result<N3DValueType> {
            self.populate_output(name, N3DValueType::SDFPosition { value })
        }
        fn input_sdf_volume(&mut self, name: &str) -> anyhow::Result<String> {
            self.evaluate_input(name)?.try_to_sdf_volume()
        }
        fn output_sdf_volume(&mut self, name: &str, value: String) -> anyhow::Result<N3DValueType> {
            self.populate_output(name, N3DValueType::SDFVolume { value })
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        N3DNodeTemplate::ScalarAdd => {
            let a = evaluator.input_scalar("A")?;
            let b = evaluator.input_scalar("B")?;
            evaluator.output_scalar("out", a + b)
        }
        N3DNodeTemplate::ScalarSub => {
            let a = evaluator.input_scalar("A")?;
            let b = evaluator.input_scalar("B")?;
            evaluator.output_scalar("out", a - b)
        }
        N3DNodeTemplate::Vec2ScalarMul => {
            let scalar = evaluator.input_scalar("scalar")?;
            let vector = evaluator.input_vec2("vector")?;
            evaluator.output_vec2("out", vector * scalar)
        }
        N3DNodeTemplate::Vec2Add => {
            let v1 = evaluator.input_vec2("v1")?;
            let v2 = evaluator.input_vec2("v2")?;
            evaluator.output_vec2("out", v1 + v2)
        }
        N3DNodeTemplate::Vec2Subtract => {
            let v1 = evaluator.input_vec2("v1")?;
            let v2 = evaluator.input_vec2("v2")?;
            evaluator.output_vec2("out", v1 - v2)
        }
        N3DNodeTemplate::NewVec2 => {
            let x = evaluator.input_scalar("x")?;
            let y = evaluator.input_scalar("y")?;
            evaluator.output_vec2("out", nalgebra::Vector2::new(x, y))
        }
        N3DNodeTemplate::NewScalar => {
            let value = evaluator.input_scalar("value")?;
            evaluator.output_scalar("out", value)
        }
        N3DNodeTemplate::NewVec3 => {
            let x = evaluator.input_scalar("x")?;
            let y = evaluator.input_scalar("y")?;
            let z = evaluator.input_scalar("z")?;
            evaluator.output_vec3("out", nalgebra::Vector3::new(x, y, z))
            //evaluator.output_vec3("out", nalgebra::Vector3::new(1.0, 2.0, 3.0))
        }
        N3DNodeTemplate::Vec3Add => {
            let v1 = evaluator.input_vec3("v1")?;
            let v2 = evaluator.input_vec3("v2")?;
            evaluator.output_vec3("out", v1 + v2)
        }
        N3DNodeTemplate::Vec3Sub => {
            let v1 = evaluator.input_vec3("v1")?;
            let v2 = evaluator.input_vec3("v2")?;
            evaluator.output_vec3("out", v1 - v2)
        }
        N3DNodeTemplate::Vec3Dot => {
            let v1 = evaluator.input_vec3("v1")?;
            let v2 = evaluator.input_vec3("v2")?;
            evaluator.output_scalar("out", v1.dot(&v2))
        }
        N3DNodeTemplate::Vec3Cross => {
            let v1 = evaluator.input_vec3("v1")?;
            let v2 = evaluator.input_vec3("v2")?;
            evaluator.output_vec3("out", v1.cross(&v2))
        }
        N3DNodeTemplate::SDFPosition => {
            evaluator.output_sdf_position("out", "p".to_string())
        }
        N3DNodeTemplate::SDFTranslate => {
            let t = evaluator.input_vec3("translation")?;
            let sdfp = evaluator.input_sdf_position("sdf position")?;
            evaluator.output_sdf_position("out", format!("{} - vec3({}, {}, {})", sdfp, t[0], t[1], t[2]))
        }
        N3DNodeTemplate::SDFBox => {
            let dim = evaluator.input_vec3("dimensions")?;
            let fil = evaluator.input_scalar("fillet")?;
            let pos = evaluator.input_sdf_position("sdf position")?;
            evaluator.output_sdf_volume("out", format!("sdf_box({}, vec3({}, {}, {}) - vec3({})) - {}", pos, dim[0], dim[1], dim[2], fil, fil))
        }
        N3DNodeTemplate::SDFUnion => {
            let sdf1 = evaluator.input_sdf_volume("sdf 1")?;
            let sdf2 = evaluator.input_sdf_volume("sdf 2")?;
            evaluator.output_sdf_volume("out", format!("op_union({}, {})", sdf1, sdf2))
        }
        N3DNodeTemplate::SDFViewer => {
            if let Ok(node) = evaluator.input_sdf_volume("sdf") {
                return Ok(N3DValueType::SDFVolume { value: node } );
            }
            anyhow::bail!("Viewer node error")
        }
    }
}
fn populate_output(
    graph: &MyGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: N3DValueType,
) -> anyhow::Result<N3DValueType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &MyGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<N3DValueType> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(other_value.clone())
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok(outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated").clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
