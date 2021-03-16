use crate::knob::knob;
use crate::node::*;
use eframe::egui::*;
use std::collections::HashMap;

pub trait Modulator: 'static + Send + Sync {
    fn modulate(&mut self, input: f64, sample_length: f64, freq: f64, time: f64) -> f64;

    fn ui(&mut self, ui: &mut Ui) -> bool;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LowPassFilter {
    pub cutoff: f64,
    pub last_sample: Option<f64>,
}

impl Clone for LowPassFilter {
    fn clone(&self) -> Self {
        Self {
            cutoff: self.cutoff,
            last_sample: None,
        }
    }
}

impl LowPassFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 440.0,
            last_sample: None,
        }
    }
}

impl Modulator for LowPassFilter {
    fn modulate(&mut self, input: f64, sample_length: f64, _freq: f64, _time: f64) -> f64 {
        let rc = 1.0 / (self.cutoff * 2.0 * std::f64::consts::PI);
        let alpha = sample_length / (rc + sample_length);

        let last_sample = self.last_sample.unwrap_or(input);

        let out = last_sample + alpha * (input - last_sample);
        self.last_sample = Some(out);

        out
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

impl Node for LowPassFilter {
    fn name(&self) -> &str {
        "Low Pass Filter"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("in", SlotType::Sound)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("out", SlotType::Sound)]
    }

    fn run(
        &mut self,
        ctx: &NodeCtx,
        input: HashMap<String, SlotValue>,
    ) -> Vec<(&'static str, SlotValue)> {
        let out = self.modulate(
            input["in"].clone().unwrap_f64(),
            ctx.sample_length,
            ctx.freq,
            ctx.time,
        );

        vec![("out", SlotValue::Sound(out))]
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        Modulator::ui(self, ui)
    }
}
