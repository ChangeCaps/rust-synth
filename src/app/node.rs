use eframe::egui::{plot::*, *};
use serde::{Deserialize, Serialize};
use serde_traitobject as s;
use std::collections::HashMap;

pub struct NodeCtx {
    pub freq: f64,
    pub time: f64,
    pub sample_length: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SlotType {
    Sound,
    Modulator,
}

impl SlotType {
    pub fn to_default(&self) -> SlotValue {
        match self {
            SlotType::Sound => SlotValue::Sound(0.0),
            SlotType::Modulator => SlotValue::Sound(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SlotValue {
    Sound(f64),
    Modulator(f64),
}

impl SlotValue {
    pub fn unwrap_f64(self) -> f64 {
        match self {
            SlotValue::Sound(v) => v,
            SlotValue::Modulator(v) => v,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

pub trait NodeClone {
    fn box_clone(&self) -> s::Box<dyn Node>;
}

impl<T: Node + Clone> NodeClone for T {
    fn box_clone(&self) -> s::Box<dyn Node> {
        s::Box::new(self.clone())
    }
}

pub trait Node: 'static + Send + Sync + NodeClone + s::Serialize + s::Deserialize {
    fn name(&self) -> &str;

    fn input_slot_types(&self) -> &[(&'static str, SlotType)];

    fn output_slot_types(&self) -> &[(&'static str, SlotType)];

    fn run(
        &mut self,
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
        &[("out", SlotType::Sound)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[]
    }

    fn run(
        &mut self,
        _ctx: &NodeCtx,
        input: HashMap<String, SlotValue>,
    ) -> Vec<(&'static str, SlotValue)> {
        vec![("out", input["out"])]
    }

    fn ui(&mut self, _ui: &mut Ui) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeContainer {
    pub inner: s::Box<dyn Node>,
    pub connections: HashMap<String, (NodeId, String)>,
}

impl Clone for NodeContainer {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.box_clone(),
            connections: self.connections.clone(),
        }
    }
}

impl NodeContainer {
    pub fn new(node: impl Node) -> Self {
        Self {
            inner: s::Box::new(node),
            connections: HashMap::new(),
        }
    }
}

impl From<Box<dyn Node>> for NodeContainer {
    fn from(node: Box<dyn Node>) -> Self {
        Self {
            inner: node.into(),
            connections: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeManager {
    pub selected_slot: Option<(String, NodeId, bool, SlotType)>,
    pub nodes: HashMap<NodeId, NodeContainer>,
    pub next_id: NodeId,
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

        nodes.insert(NodeId(0), NodeContainer::new(OutputNode));

        Self {
            selected_slot: None,
            nodes,
            next_id: NodeId(1),
            output_node: NodeId(0),
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

        self.run_node(ctx, &self.output_node.clone(), &mut outputs);

        outputs[&(self.output_node, "out")].clone().unwrap_f64()
    }

    pub fn calculate_segments(&mut self, freq: f64) {
        self.segments.clear();

        for i in 0..500 {
            let mut output = HashMap::new();

            let sample_length = 2.0 / 500.0 / freq;

            let ctx = NodeCtx {
                freq,
                sample_length,
                time: i as f64 * sample_length,
            };

            self.run_node(&ctx, &self.output_node.clone(), &mut output);

            for id in self.nodes.keys() {
                if let Some(value) = output.get(&(*id, "out")) {
                    if let SlotValue::Sound(s) = value {
                        self.segments.entry(*id).or_insert(vec![0.0; 500])[i] = *s;
                    }
                }
            }
        }
    }

    pub fn run_node(
        &mut self,
        ctx: &NodeCtx,
        id: &NodeId,
        outputs: &mut HashMap<(NodeId, &'static str), SlotValue>,
    ) {
        let node = &self.nodes[&id];
        let mut inputs = HashMap::new();

        for (input, (node, output)) in node.connections.clone() {
            if !outputs.contains_key(&(node, &output)) {
                self.run_node(ctx, &node, outputs);
            }

            if let Some(output) = outputs.get(&(node, &output)) {
                inputs.insert(input.to_string(), output.clone());
            }
        }

        let node = self.nodes.get_mut(&id).unwrap();

        for (slot, ty) in node.inner.input_slot_types() {
            if !inputs.contains_key(&slot.to_string()) {
                inputs.insert(slot.to_string(), ty.to_default());
            }
        }

        let output = node.inner.run(ctx, inputs);

        for (name, value) in output {
            outputs.insert((*id, name), value);
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
                                        segment
                                            .iter()
                                            .enumerate()
                                            .map(|(i, s)| Value::new(i as f64 * 0.004, *s)),
                                    );

                                    let plot =
                                        Plot::default().curve(curve).symmetrical_y_bounds(true);

                                    ui.add(plot);
                                });
                            }

                            ui.vertical(|ui| {
                                for (name, ty) in node.inner.output_slot_types() {
                                    let (pos, connect) = output(name, *id, *ty, selected_slot, ui);

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

        if let Some((name, id, input, ty)) = &self.selected_slot {
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
