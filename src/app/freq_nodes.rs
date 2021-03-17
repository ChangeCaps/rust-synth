use crate::node::SlotValue;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct InputFreqNode;

crate::node! {
    InputFreqNode => "Input Freq"(&mut self, ctx: &NodeCtx) -> [out: Float] {
        out = SlotValue::Float(ctx.freq);
    }

    fn ui(&mut self, _ui: &mut Ui) -> bool {
        false
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FreqShiftNode(f64);

impl FreqShiftNode {
    pub fn new() -> Self {
        Self(0.0)
    }
}

crate::node! {
    FreqShiftNode => "Freq Shift"(&mut self, ctx: &NodeCtx, freq: Float) -> [freq_out: Float] {
        let freq = freq.unwrap_f64(ctx.freq) * 2.0f64.powf(self.0 / 12.0);
        freq_out = SlotValue::Float(freq);
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let prev = self.0;

        ui.vertical(|ui| {
            ui.set_max_width(100.0);
            ui.add(eframe::egui::DragValue::f64(&mut self.0));
        });

        self.0 != prev
    }
}
