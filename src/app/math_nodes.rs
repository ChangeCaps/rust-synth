use crate::node::SlotValue;

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MathMode {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct MathNode {
    mode: MathMode,
}

impl MathNode {
    pub fn new() -> Self {
        Self {
            mode: MathMode::Add,
        }
    }
}

crate::node! {
    MathNode => "Math Node"(&mut self, ctx: &NodeCtx, freq: Float, a: Float, b: Float) -> [freq_out: Float, out: Float] {
        freq_out = SlotValue::Float(freq.unwrap_f64(ctx.freq));

        let a = a.unwrap_f64(0.0);
        let b = b.unwrap_f64(0.0);

        match self.mode {
            MathMode::Add => out = SlotValue::Float(a + b),
            MathMode::Sub => out = SlotValue::Float(a - b),
            MathMode::Mul => out = SlotValue::Float(a * b),
            MathMode::Div => out = SlotValue::Float(a / b),
        }
    }

    display out;

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let prev = self.mode.clone();

        ui.vertical(|ui| {
            ui.set_max_width(100.0);
            ui.radio_value(&mut self.mode, MathMode::Add, "Add");
            ui.radio_value(&mut self.mode, MathMode::Sub, "Sub");
            ui.radio_value(&mut self.mode, MathMode::Mul, "Mul");
            ui.radio_value(&mut self.mode, MathMode::Div, "Div");
        });

        self.mode != prev
    }
}
