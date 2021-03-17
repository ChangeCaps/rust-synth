use crate::freq_nodes::*;
use eframe::egui::{plot::*, *};
use serde::{Deserialize, Serialize};
use serde_traitobject as s;
use std::collections::HashMap;

#[derive(Clone)]
pub struct NodeCtx {
    pub freq: f64,
    pub time: f64,
    pub sample_length: f64,
    pub last_sample: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SlotType {
    Float,
}

impl SlotType {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SlotValue {
    Float(f64),
    None,
}

impl SlotValue {
    pub fn unwrap_f64(self, default: f64) -> f64 {
        match self {
            SlotValue::Float(f) => f,
            SlotValue::None => default,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

pub trait NodeClone {
    fn box_clone(&self) -> Box<dyn Node>;
}

impl<T: Node + Clone> NodeClone for T {
    fn box_clone(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

pub trait Node: 'static + Send + Sync + NodeClone + s::Serialize + s::Deserialize {
    fn name(&self) -> &str;

    fn input_slot_types(&self) -> &[(&'static str, SlotType)];

    fn output_slot_types(&self) -> &[(&'static str, SlotType)];

    fn setup(&mut self) {}

    fn save_last_output(&self) -> &Option<&str> {
        &None
    }

    fn display_out(&self) -> &Option<&str> {
        &None
    }

    fn run(
        &self,
        ctx: &NodeCtx,
        input: HashMap<String, SlotValue>,
    ) -> Vec<(&'static str, SlotValue)>;

    fn ui(&mut self, ui: &mut Ui) -> bool;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OutputNode;

impl Node for OutputNode {
    fn name(&self) -> &str {
        "Output"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("out", SlotType::Float)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[]
    }

    fn run(
        &self,
        _ctx: &NodeCtx,
        input: HashMap<String, SlotValue>,
    ) -> Vec<(&'static str, SlotValue)> {
        vec![("output node", input["out"])]
    }

    fn ui(&mut self, _ui: &mut Ui) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeContainer {
    #[serde(with = "serde_traitobject")]
    pub inner: Box<dyn Node>,
    pub connections: HashMap<String, (NodeId, String)>,
    pub last_sample: Option<f64>,
}

impl Clone for NodeContainer {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.box_clone(),
            connections: self.connections.clone(),
            last_sample: self.last_sample.clone(),
        }
    }
}

impl NodeContainer {
    pub fn new(node: impl Node) -> Self {
        Self {
            inner: Box::new(node),
            connections: HashMap::new(),
            last_sample: None,
        }
    }
}

impl From<Box<dyn Node>> for NodeContainer {
    fn from(node: Box<dyn Node>) -> Self {
        Self {
            inner: node.into(),
            connections: HashMap::new(),
            last_sample: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeManager {
    pub selected_slot: Option<(String, NodeId, bool, SlotType)>,
    pub nodes: HashMap<NodeId, NodeContainer>,
    pub next_id: NodeId,
    pub input_node: NodeId,
    pub output_node: NodeId,
    pub segments: HashMap<NodeId, Vec<f64>>,
}

impl From<Vec<Box<dyn Node>>> for NodeManager {
    fn from(nodes: Vec<Box<dyn Node>>) -> Self {
        let mut node_manager = NodeManager::new();

        for node in nodes {
            node_manager.add(NodeContainer::from(node));
        }

        node_manager
    }
}

impl NodeManager {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();

        nodes.insert(NodeId(0), NodeContainer::new(InputFreqNode));
        nodes.insert(NodeId(1), NodeContainer::new(OutputNode));

        Self {
            selected_slot: None,
            nodes,
            next_id: NodeId(2),
            input_node: NodeId(0),
            output_node: NodeId(1),
            segments: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: NodeContainer) -> NodeId {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id.0 += 1;

        id
    }

    pub fn run(&mut self, ctx: &NodeCtx) -> f64 {
        let mut outputs: HashMap<(NodeId, &'static str), SlotValue> = HashMap::new();

        let inputs = self.gen_inputs(ctx, &self.output_node.clone(), &mut outputs);
        self.run_node(ctx, &self.output_node.clone(), inputs, &mut outputs);

        outputs[&(self.output_node, "output node")]
            .clone()
            .unwrap_f64(0.0)
    }

    const NUM_SAMPLES: usize = 100;

    pub fn calculate_segments(&mut self, freq: f64) {
        self.segments.clear();

        for id in self.nodes.keys().cloned().collect::<Vec<_>>() {
            let node = self.nodes.get_mut(&id).unwrap();

            node.inner.setup();
            node.last_sample = None;

            for i in 0..Self::NUM_SAMPLES {
                let mut output = HashMap::new();

                let sample_length = 2.0 / Self::NUM_SAMPLES as f64 / freq;

                let mut ctx = NodeCtx {
                    freq,
                    sample_length,
                    time: i as f64 * sample_length,
                    last_sample: self.nodes[&id].last_sample.unwrap_or(0.0),
                };

                let mut inputs = self.gen_inputs(&ctx, &id.clone(), &mut output);

                if let Some(freq) = inputs.get("freq") {
                    let freq = freq.unwrap_f64(ctx.freq);
                    ctx.sample_length = 2.0 / Self::NUM_SAMPLES as f64 / freq;
                    ctx.time = i as f64 * ctx.sample_length;

                    output = HashMap::new();

                    inputs = self.gen_inputs(&ctx, &id.clone(), &mut output);
                }

                self.run_node(&ctx, &id.clone(), inputs, &mut output);

                if let Some(name) = self.nodes[&id].inner.display_out() {
                    if let Some(value) = output.get(&(id, name)) {
                        if let SlotValue::Float(s) = value {
                            self.segments.entry(id).or_insert(Vec::new()).push(*s);
                        }
                    }
                }
            }
        }
    }

    pub fn gen_inputs(
        &mut self,
        ctx: &NodeCtx,
        id: &NodeId,
        outputs: &mut HashMap<(NodeId, &'static str), SlotValue>,
    ) -> HashMap<String, SlotValue> {
        let node = &self.nodes[id];
        let mut inputs = HashMap::new();

        for (input, (node, output)) in node.connections.clone() {
            if !outputs.contains_key(&(node, &output)) {
                let inputs = self.gen_inputs(ctx, &node, outputs);

                self.run_node(ctx, &node, inputs, outputs);
            }

            if let Some(output) = outputs.get(&(node, &output)) {
                inputs.insert(input.to_string(), output.clone());
            }
        }

        let node = &self.nodes[id];

        for (slot, _ty) in node.inner.input_slot_types() {
            if !inputs.contains_key(&slot.to_string()) {
                inputs.insert(slot.to_string(), SlotValue::None);
            }
        }

        inputs
    }

    pub fn run_node(
        &mut self,
        ctx: &NodeCtx,
        id: &NodeId,
        inputs: HashMap<String, SlotValue>,
        outputs: &mut HashMap<(NodeId, &'static str), SlotValue>,
    ) {
        let node = self.nodes.get_mut(&id).unwrap();

        let mut ctx = ctx.clone();
        ctx.last_sample = node.last_sample.unwrap_or(0.0);

        let output = node.inner.run(&ctx, inputs);

        for (name, value) in output {
            outputs.insert((*id, name), value);
        }

        if let Some(slot) = self.nodes[&id].inner.save_last_output() {
            self.nodes.get_mut(id).unwrap().last_sample =
                Some(outputs.get(&(*id, slot)).unwrap().unwrap_f64(0.0));
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, freq: f64) -> bool {
        let selected_slot = &mut self.selected_slot;
        let segments = &self.segments;

        let mut input_slot_positions = HashMap::new();
        let mut output_slot_positions = HashMap::new();

        let mut mutated = false;

        for (id, node) in &mut self.nodes {
            Area::new(id.clone()).show(&ui.ctx(), |ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(node.inner.name());

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                for (slot, ty) in node.inner.input_slot_types() {
                                    let (pos, connect) = input(slot, *id, *ty, selected_slot, ui);

                                    if connect {
                                        if let Some((s_slot, id, is_input, s_ty)) = selected_slot {
                                            if !*is_input && ty == s_ty {
                                                node.connections.insert(
                                                    slot.to_string(),
                                                    (*id, s_slot.to_string()),
                                                );

                                                mutated = true;
                                            }
                                        }
                                    }

                                    input_slot_positions.insert((*slot, *id), pos);
                                }
                            });

                            mutated = node.inner.ui(ui) || mutated;

                            if let Some(segment) = segments.get(id) {
                                ui.vertical(|ui| {
                                    let curve = Curve::from_values_iter(
                                        segment.iter().enumerate().map(|(i, s)| {
                                            Value::new(
                                                i as f64 * (1.0 / Self::NUM_SAMPLES as f64) * 2.0,
                                                *s,
                                            )
                                        }),
                                    );

                                    let color = Color32::from_gray(130).additive();

                                    let stroke = Stroke { width: 1.0, color };

                                    let plot = Plot::default()
                                        .curve(curve)
                                        .symmetrical_y_bounds(true)
                                        .view_aspect(1.0)
                                        .vline(VLine::new(2.0, stroke));

                                    ui.add(plot);
                                });
                            }

                            ui.vertical(|ui| {
                                for (name, ty) in node.inner.output_slot_types() {
                                    let (pos, _connect) = output(name, *id, *ty, selected_slot, ui);

                                    output_slot_positions.insert((*name, *id), pos);
                                }
                            });
                        });
                    });
                });
            });
        }

        for (id, node) in &self.nodes {
            for (i_slot, (o_id, o_slot)) in &node.connections {
                let i_pos = input_slot_positions[&(i_slot.as_str(), *id)];
                let o_pos = output_slot_positions[&(o_slot.as_str(), *o_id)];

                ui.painter()
                    .line_segment([i_pos, o_pos], ui.style().visuals.widgets.active.fg_stroke);
            }
        }

        if !ui.input().pointer.button_down(PointerButton::Primary) {
            self.selected_slot = None;
        }

        if let Some((name, id, input, _ty)) = &self.selected_slot {
            let slot_positions = if *input {
                &input_slot_positions
            } else {
                &output_slot_positions
            };

            if let Some(pos) = slot_positions.get(&(name, *id)) {
                if let Some(pointer) = ui.input().pointer.interact_pos() {
                    ui.painter()
                        .line_segment([*pos, pointer], ui.style().visuals.widgets.active.fg_stroke);
                }
            }
        }

        if mutated {
            self.calculate_segments(freq);
        }

        mutated
    }
}

fn input(
    name: &'static str,
    node_id: NodeId,
    ty: SlotType,
    selected_slot: &mut Option<(String, NodeId, bool, SlotType)>,
    ui: &mut Ui,
) -> (Pos2, bool) {
    ui.horizontal(|ui| {
        let desired_size = ui.spacing().interact_size.y * Vec2::new(0.5, 0.5);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        if response.drag_started() {
            *selected_slot = Some((name.to_string(), node_id, true, ty));
        }

        let visuals = ui.style().interact(&response);

        let radius = rect.height() * 0.5;

        ui.painter()
            .circle(rect.center(), radius, visuals.bg_fill, visuals.fg_stroke);

        ui.label(name);

        (
            rect.center(),
            response.hovered() && !ui.input().pointer.button_down(PointerButton::Primary),
        )
    })
    .inner
}

fn output(
    name: &'static str,
    node_id: NodeId,
    ty: SlotType,
    selected_slot: &mut Option<(String, NodeId, bool, SlotType)>,
    ui: &mut Ui,
) -> (Pos2, bool) {
    ui.horizontal(|ui| {
        ui.label(name);

        let desired_size = ui.spacing().interact_size.y * Vec2::new(0.5, 0.5);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        if response.drag_started() {
            *selected_slot = Some((name.to_string(), node_id, false, ty));
        }

        let visuals = ui.style().interact(&response);

        let radius = rect.height() * 0.5;

        ui.painter()
            .circle(rect.center(), radius, visuals.bg_fill, visuals.fg_stroke);

        (
            rect.center(),
            response.hovered() && !ui.input().pointer.button_down(PointerButton::Primary),
        )
    })
    .inner
}
