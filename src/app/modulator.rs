use crate::knob::knob;
use crate::node::*;
use eframe::egui::*;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LowPassFilter {
    pub cutoff: f64,
}

impl Clone for LowPassFilter {
    fn clone(&self) -> Self {
        Self {
            cutoff: self.cutoff,
        }
    }
}

impl LowPassFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 440.0,
        }
    }
}

impl Node for LowPassFilter {
    fn name(&self) -> &str {
        "Low Pass Filter"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq", SlotType::Float), ("in", SlotType::Float)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq_out", SlotType::Float), ("out", SlotType::Float)]
    }

    fn save_last_output(&self) -> &Option<&str> {
        &Some("out")
    }

    fn display_out(&self) -> &Option<&str> {
        &Some("out")
    }

    fn run(
        &self,
        ctx: &NodeCtx,
        input: HashMap<String, SlotValue>,
    ) -> Vec<(&'static str, SlotValue)> {
        let freq = input["freq"].unwrap_f64(ctx.freq);
        let input = input["in"].clone().unwrap_f64(0.0);

        let rc = 1.0 / (self.cutoff * 2.0 * std::f64::consts::PI);
        let alpha = ctx.sample_length / (rc + ctx.sample_length);

        let out = ctx.last_sample + alpha * (input - ctx.last_sample);

        vec![("freq_out", SlotValue::Float(freq)), ("out", SlotValue::Float(out))]
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = true;

        ui.vertical(|ui| {
            ui.label("Cutoff");
            changed = knob(ui, &mut self.cutoff, 0.0, 440.0 * 8.0) && changed;
        });

        changed
    }
}
