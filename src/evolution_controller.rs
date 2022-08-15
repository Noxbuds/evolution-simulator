use std::{sync::mpsc::{self, Sender, Receiver, SendError}, thread, vec};

use chrono::UTC;

use crate::{config::SimulationConfig, simulator::{Simulator, SimulatorMessage}, dna::{CreatureDna, self, generate_dna}, fitness::FitnessFunction};

const LOG_OWNER: &str = "[evolution_controller]";

pub type CreatureResult = (CreatureDna, f64);

pub enum ControllerMessage {
    Start,
    Stop,
    Results(Vec<CreatureResult>),
}

pub struct EvolutionController {
    message_sender: Sender<ControllerMessage>,
    message_receiver: Receiver<ControllerMessage>,
    running: bool,
}

impl EvolutionController {
    fn generate_dna(config: SimulationConfig) -> Vec<CreatureDna> {
        let mut dna: Vec<CreatureDna> = Vec::with_capacity(config.creature_count as usize);
        let creature_size = config.creature_config.size;
        for _ in 0..config.creature_count {
            dna.push(generate_dna(creature_size * creature_size));
        }
        dna
    }

    pub fn new(config: SimulationConfig, fitness_func: FitnessFunction, threads: i32) -> EvolutionController {
        let simulators = vec![Simulator::from_config(config, fitness_func)];
        let mut running = false;
        let dna = Self::generate_dna(config);

        let (main_sender, thread_receiver) = mpsc::channel::<ControllerMessage>();
        let (thread_sender, main_receiver) = mpsc::channel::<ControllerMessage>();

        thread::spawn(move || loop {
            if running {
                let start = UTC::now();

                let creature_count = dna.len();
                let batch_size = creature_count / threads as usize;
                for (i, simulator) in simulators.iter().enumerate() {
                    let slice = &dna[i * batch_size..(i + 1) * batch_size];
                    simulator.start(slice);
                }

                let mut results: Vec<CreatureResult> = Vec::new();
                for simulator in simulators.iter() {
                    let message = simulator.message_receiver.recv();
                    match message {
                        Ok(result) => {
                            if let SimulatorMessage::Results(mut result) = result {
                                results.append(&mut result);
                            }
                        },
                        Err(msg) => eprintln!("{}: error while reading channel: {:?}", LOG_OWNER, msg)
                    }
                }

                let end = UTC::now();
                let time = end - start;
                println!("{}: generation completed in {}s", LOG_OWNER, time.num_seconds());

                let send_result = thread_sender.send(ControllerMessage::Results(results));
                if let Err(msg) = send_result {
                    eprintln!("{}: error while sending results: {:?}", LOG_OWNER, msg);
                }
            }

            let result = thread_receiver.try_recv();
            if let Ok(message) = result {
                match message {
                    ControllerMessage::Start => {
                        running = true;
                        println!("{}: controller running", LOG_OWNER);
                    },
                    ControllerMessage::Stop => {
                        running = false;
                        println!("{}: controller stopping", LOG_OWNER);
                    },
                    _ => {},
                };
            }
        });

        EvolutionController {
            message_sender: main_sender,
            message_receiver: main_receiver,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), SendError<ControllerMessage>> {
        let result = self.message_sender.send(ControllerMessage::Start);
        if let Ok(_) = result {
            self.running = true;
        }
        result
    }
    
    pub fn stop(&mut self) -> Result<Vec<CreatureResult>, SendError<ControllerMessage>> {
        let send_result = self.message_sender.send(ControllerMessage::Stop);
        if let Err(err) = send_result {
            return Err(err);
        }

        println!("{}: collecting results (will wait for final generation)", LOG_OWNER);
        let results: Vec<CreatureResult> = self.message_receiver.recv().into_iter()
            .filter_map(|message| {
                match message {
                    ControllerMessage::Results(results) => Some(results),
                    _ => None,
                }
            })
            .flatten()
            .collect();

        println!("{}: results gathered!", LOG_OWNER);
        self.running = false;
        Ok(results)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

