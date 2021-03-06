pub mod driver;
pub mod freq_nodes;
pub mod knob;
pub mod macros;
pub mod math_nodes;
pub mod modulator;
pub mod node;
pub mod note;
pub mod value_node;
pub mod wave;

use crate::driver::*;
use crate::freq_nodes::*;
use crate::math_nodes::*;
use crate::modulator::*;
use crate::node::*;
use crate::value_node::*;
use crate::wave::*;
use eframe::{egui::*, epi};

pub struct App {
    visualiser_freq: f64,
    driver: DriverHandle,
    nodes: NodeManager,
}

impl App {
    pub fn new() -> Result<Self, anyhow::Error> {
        let driver = Driver::run()?;

        let nodes: Vec<Box<dyn Node>> = vec![
            Box::new(SquareWave::new()),
            Box::new(SineWave::new()),
            Box::new(SawWave::new()),
            Box::new(LowPassFilter::new()),
            Box::new(MathNode::new()),
            Box::new(MathNode::new()),
            Box::new(ValueNode::new()),
            Box::new(FreqShiftNode::new()),
        ];

        let mut nodes = NodeManager::from(nodes);
        nodes.calculate_segments(440.0);

        driver.set_nodes(nodes.clone());

        Ok(Self {
            visualiser_freq: 440.0,
            driver,
            nodes,
        })
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Rust synth"
    }

    fn load(&mut self, storage: &dyn epi::Storage) {
        if let Some(nodes_ron) = storage.get_string("nodes") {
            if let Ok(nodes) = serde_json::from_str::<NodeManager>(nodes_ron.as_str()) {
                self.nodes = nodes;
                self.driver.set_nodes(self.nodes.clone());
            }
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        let nodes_ron = serde_json::to_string(&self.nodes).unwrap();

        storage.set_string("nodes", nodes_ron);

        storage.flush();
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f32(10.0)
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(10000.0, 10000.0)
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Rust synth");

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Settings");

                    ui.horizontal(|ui| {
                        ui.label("Freq: ");
                        let prev = self.visualiser_freq;
                        ui.add(DragValue::f64(&mut self.visualiser_freq).speed(1.0));

                        if self.visualiser_freq != prev {
                            self.driver.set_freq(self.visualiser_freq);
                            self.nodes.calculate_segments(self.visualiser_freq);
                        }
                    });
                });
            });
        });

        let frame = Frame {
            fill: ctx.style().visuals.extreme_bg_color,
            ..Frame::none()
        };

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            let mutated = self.nodes.ui(ui, self.visualiser_freq);
            self.visualiser_freq = self.visualiser_freq.max(1.0);

            if mutated {
                self.driver.set_nodes(self.nodes.clone());
            }
        });
    }
}
