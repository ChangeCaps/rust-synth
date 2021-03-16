use crate::modulator::*;
use crate::node::*;
use crate::wave::*;
use cpal::{traits::*, *};
use crossbeam::channel::{unbounded, Receiver, Sender};

pub enum DriverCommand {
    SetNodes(NodeManager),
    SetFreq(f64),
}

pub struct DriverHandle {
    stream: Option<Stream>,
    sender: Sender<DriverCommand>,
}

impl DriverHandle {
    pub fn set_nodes(&self, nodes: NodeManager) {
        self.sender.send(DriverCommand::SetNodes(nodes)).unwrap();
    }

    pub fn set_freq(&self, freq: f64) {
        self.sender.send(DriverCommand::SetFreq(freq)).unwrap();
    }
}

pub struct Driver {
    nodes: Option<NodeManager>,
    freq: f64,
    receiver: Receiver<DriverCommand>,
}

impl Driver {
    pub fn handle_commands(&mut self) {
        for command in self.receiver.try_iter() {
            match command {
                DriverCommand::SetNodes(nodes) => self.nodes = Some(nodes),
                DriverCommand::SetFreq(freq) => self.freq = freq,
            }
        }
    }

    pub fn run() -> Result<DriverHandle, anyhow::Error> {
        let (sender, receiver) = unbounded();

        let f = move || -> Result<Stream, anyhow::Error> {
            let host = default_host();

            let device = host
                .default_output_device()
                .expect("failed to get default device");

            let config = device
                .default_output_config()
                .expect("failed to get default output config");

            let driver = Driver {
                nodes: None,
                freq: 440.0,
                receiver,
            };

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => run::<f32>(driver, &device, &config.into())?,
                cpal::SampleFormat::I16 => run::<i16>(driver, &device, &config.into())?,
                cpal::SampleFormat::U16 => run::<u16>(driver, &device, &config.into())?,
            };

            Ok(stream)
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::spawn(move || {
                f().unwrap();

                std::thread::park();
            });

            Ok(DriverHandle {
                sender,
                stream: None,
            })
        }

        #[cfg(target_arch = "wasm32")]
        {
            let stream = f()?;
            Ok(DriverHandle {
                sender,
                stream: Some(stream),
            })
        }
    }
}

fn run<T: Sample>(
    mut driver: Driver,
    device: &Device,
    config: &StreamConfig,
) -> Result<Stream, anyhow::Error> {
    let sample_rate = config.sample_rate.0 as f64;
    let sample_length = 1.0 / sample_rate;
    let channels = config.channels as usize;
    let mut time = 0.0;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let ctx = NodeCtx {
                    freq: driver.freq,
                    time,
                    sample_length,
                };

                driver.handle_commands();

                time += 1.0 / sample_rate;

                let wave = driver.nodes.as_mut().unwrap().run(&ctx);

                let out = wave * 0.01;

                for sample in frame {
                    *sample = Sample::from::<f32>(&(out as f32));
                }
            }
        },
        |err| println!("Error: {}", err),
    )?;

    stream.play()?;

    Ok(stream)
}
