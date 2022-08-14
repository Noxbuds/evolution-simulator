use std::{sync::mpsc::{Sender, self, Receiver}, thread};

use crate::{config::SimulationConfig, world::World, dna::CreatureDna};

pub enum SimulatorMessage {
    Start,
    Stop,
    Dna(Vec<CreatureDna>)
}

pub struct Simulator {
    dna: Vec<CreatureDna>,
    thread: Option<thread::JoinHandle<()>>,
    message_sender: Sender<SimulatorMessage>,
    message_receiver: Receiver<SimulatorMessage>,
    running: bool,
}

impl Simulator {
    pub fn from_config(config: SimulationConfig) -> Simulator {
        let mut world = World::from_config(config.world_config);
        let mut running = false;

        let (sim_tx, thread_rx) = mpsc::channel::<SimulatorMessage>();
        let (thread_tx, sim_rx) = mpsc::channel::<SimulatorMessage>();
        let thread = thread::spawn(move || loop {
            if running {
                let dt = config.timestep / config.sub_steps as f64;
                for _ in 0..config.sub_steps {
                    world.update(dt);
                }
                println!("simulation step complete");
            }

            if let Ok(message) = thread_rx.try_recv() {
                match message {
                    SimulatorMessage::Start => running = true,
                    SimulatorMessage::Stop => running = false,
                    _ => {},
                }
            }
        });

        Simulator {
            dna: vec![],
            thread: Some(thread),
            message_sender: sim_tx,
            message_receiver: sim_rx,
            running: false,
        }
    }

    pub fn stop(&mut self) {
        if let Err(msg) = self.message_sender.send(SimulatorMessage::Stop) {
            eprintln!("failed to stop simulator: {:?}", msg);
        } else {
            self.running = false;
        }
    }

    pub fn start(&mut self) {
        if let Err(msg) = self.message_sender.send(SimulatorMessage::Start) {
            eprintln!("failed to start simulator: {:?}", msg);
        } else {
            self.running = true;
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

