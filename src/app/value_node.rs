use crate::node::SlotValue;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ValueNode(f64);

impl ValueNode {
    pub fn new() -> Self {
        Self(0.0)
    }
}

crate::node! {
    ValueNode => "Value"(&mut self, ctx: &NodeCtx) -> [out: Float] {
        out = SlotValue::Float(self.0);
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let prev = self.0;

        ui.add(eframe::egui::DragValue::f64(&mut self.0).speed(0.5));

        self.0 != prev
    }
}
