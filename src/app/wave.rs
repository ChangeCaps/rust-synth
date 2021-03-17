use crate::knob::knob;
use crate::node::*;
use eframe::egui::*;
use std::collections::HashMap;

pub trait WaveClone {
    fn box_clone(&self) -> Box<dyn WaveGenerator>;
}

impl<T: Clone + WaveGenerator> WaveClone for T {
    fn box_clone(&self) -> Box<dyn WaveGenerator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn WaveGenerator> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub trait WaveGenerator: 'static + Send + Sync + WaveClone {
    fn gen(&self, freq: f64, time: f64) -> f64;

    fn ui(&mut self, ui: &mut Ui) -> bool;
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SquareWave {
    pub modulation: f64,
}

impl SquareWave {
    pub fn new() -> Self {
        Self { modulation: 0.5 }
    }
}

impl WaveGenerator for SquareWave {
    fn gen(&self, freq: f64, time: f64) -> f64 {
        if (time * freq) % 1.0 > self.modulation {
            1.0
        } else {
            -1.0
        }
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = true;

        ui.vertical(|ui| {
            ui.label("Mod");
            changed = knob(ui, &mut self.modulation, 0.0, 1.0) && changed;
        });

        changed
    }
}

impl Node for SquareWave {
    fn name(&self) -> &str {
        "Square Wave"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq", SlotType::Float)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq_out", SlotType::Float), ("out", SlotType::Float)]
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

        vec![
            ("freq_out", SlotValue::Float(freq)),
            ("out", SlotValue::Float(self.gen(freq, ctx.time))),
        ]
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        WaveGenerator::ui(self, ui)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SineWave {
    pub modulation: f64,
}

impl SineWave {
    pub fn new() -> Self {
        Self { modulation: 1.0 }
    }
}

impl WaveGenerator for SineWave {
    fn gen(&self, freq: f64, time: f64) -> f64 {
        let wave = (time * freq * std::f64::consts::PI * 2.0).sin();
        let sign = wave.signum();

        wave.abs().powf(self.modulation) * sign
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = true;

        ui.vertical(|ui| {
            ui.label("Mod");
            changed = knob(ui, &mut self.modulation, 0.0, 2.0) && changed;
        });

        changed
    }
}

impl Node for SineWave {
    fn name(&self) -> &str {
        "Sine Wave"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq", SlotType::Float)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq_out", SlotType::Float), ("out", SlotType::Float)]
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

        vec![
            ("freq_out", SlotValue::Float(freq)),
            ("out", SlotValue::Float(self.gen(freq, ctx.time))),
        ]
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        WaveGenerator::ui(self, ui)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SawWave {
    pub modulation: f64,
}

impl SawWave {
    pub fn new() -> Self {
        Self { modulation: 0.0 }
    }
}

impl WaveGenerator for SawWave {
    fn gen(&self, freq: f64, time: f64) -> f64 {
        let x = (time * freq + self.modulation * 0.25 + 0.5) % 1.0 * 2.0 - 1.0;

        if x.abs() > self.modulation {
            (time * freq + self.modulation * 0.25) % 1.0 * 2.0 - 1.0
        } else {
            x
        }
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = true;

        ui.vertical(|ui| {
            ui.label("Mod");
            changed = knob(ui, &mut self.modulation, 0.0, 0.5) && changed;
        });

        changed
    }
}

impl Node for SawWave {
    fn name(&self) -> &str {
        "Saw Wave"
    }

    fn input_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq", SlotType::Float)]
    }

    fn output_slot_types(&self) -> &[(&'static str, SlotType)] {
        &[("freq_out", SlotType::Float), ("out", SlotType::Float)]
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

        vec![
            ("freq_out", SlotValue::Float(freq)),
            ("out", SlotValue::Float(self.gen(freq, ctx.time))),
        ]
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        WaveGenerator::ui(self, ui)
    }
}

#[derive(Clone)]
pub struct Combined {
    pub waves: Vec<(String, bool, Box<dyn WaveGenerator>)>,
}

impl Combined {
    pub fn new() -> Self {
        Self {
            waves: vec![
                (String::from("Sine"), false, Box::new(SineWave::new())),
                (String::from("Square"), false, Box::new(SquareWave::new())),
                (String::from("Saw"), false, Box::new(SawWave::new())),
            ],
        }
    }
}

impl WaveGenerator for Combined {
    fn gen(&self, freq: f64, time: f64) -> f64 {
        let mut sample = 0.0;

        for (_name, selected, wave) in &self.waves {
            if !*selected {
                continue;
            }

            sample += wave.gen(freq, time);
        }

        sample
    }

    fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        ui.vertical(|ui| {
            for (name, selected, _wave) in &mut self.waves {
                let prev = *selected;

                ui.checkbox(selected, name.clone());

                changed = changed || prev == *selected;
            }

            for (_name, selected, wave) in &mut self.waves {
                if !*selected {
                    continue;
                }

                changed = wave.ui(ui) || changed;
            }
        });

        changed
    }
}
