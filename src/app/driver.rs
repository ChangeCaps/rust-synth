use crate::modulator::*;
use crate::node::*;
use crate::wave::*;
use cpal::{traits::*, *};
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum DriverCommand {
    SetNodes(NodeManager),
    SetFreq(f64),
}

pub struct DriverHandle {
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
        let (sender, receiver) = channel();

        std::thread::spawn(move || -> Result<(), anyhow::Error> {
            let host = default_host();

            let device = host.default_output_device().unwrap();

            let config = device.default_output_config().unwrap();

            let driver = Driver {
                nodes: None,
                freq: 440.0,
                receiver,
            };

            let _stream = match config.sample_format() {
                cpal::SampleFormat::F32 => run::<f32>(driver, &device, &config.into())?,
                cpal::SampleFormat::I16 => run::<i16>(driver, &device, &config.into())?,
                cpal::SampleFormat::U16 => run::<u16>(driver, &device, &config.into())?,
            };

            std::thread::park();

            Ok(())
        });

        Ok(DriverHandle { sender })
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
