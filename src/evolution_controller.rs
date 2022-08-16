use std::{sync::{mpsc::{self, Sender, Receiver, SendError}, Arc}, thread, vec};

use chrono::UTC;
use rand::{Rng, thread_rng};

use crate::{config::{SimulationConfig, MutationConfig}, simulator::{Simulator, SimulatorMessage}, dna::{CreatureDna, generate_dna, mutate_dna}, fitness::FitnessFunction};

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

fn simulate_generation(dna: &Vec<CreatureDna>, simulators: &Vec<Simulator>) -> Vec<CreatureResult> {
    let creature_count = dna.len();
    let batch_size = creature_count / simulators.len();
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

    return results;
}

fn select_fittest(results: &Vec<(CreatureDna, f64)>) -> Vec<CreatureDna> {
    let mut sorted = results.clone();
    sorted.sort_by(|(_, a), (_, b)| {
        a.total_cmp(b)
    });

    let mut rng = thread_rng();
    for _ in 0..results.len() / 2 {
        let id = (rng.gen::<f64>() - 0.5) * sorted.len() as f64;
        sorted.remove(id.abs() as usize);
    }

    sorted.into_iter().map(|(dna, _)| { dna }).collect()
}

fn reproduce(config: MutationConfig, initial: &Vec<CreatureDna>) -> Vec<CreatureDna> {
    initial.iter().flat_map(|dna| {
        [mutate_dna(dna, config), mutate_dna(dna, config)]
    }).collect()
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

    pub fn new(config: SimulationConfig, fitness_func: FitnessFunction) -> EvolutionController {
        let fitness = Arc::new(fitness_func);
        let simulators = (0..config.threads).into_iter().map(|_| {
            Simulator::from_config(config, fitness.clone())
        }).collect();

        let mut running = false;
        let mut dna = Self::generate_dna(config);
        let mut results: Vec<CreatureResult> = Vec::new();

        let (main_sender, thread_receiver) = mpsc::channel::<ControllerMessage>();
        let (thread_sender, main_receiver) = mpsc::channel::<ControllerMessage>();

        thread::spawn(move || loop {
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

                        let send_result = thread_sender.send(ControllerMessage::Results(results.clone()));
                        if let Err(msg) = send_result {
                            eprintln!("{}: error while sending results: {:?}", LOG_OWNER, msg);
                        }
                    },
                    _ => {},
                };
            }

            if running {
                let start = UTC::now();

                results = simulate_generation(&dna, &simulators);
                let fittest = select_fittest(&results);
                dna = reproduce(config.mutation_config, &fittest);

                let end = UTC::now();
                let time = end - start;
                println!("{}: generation completed in {}ms", LOG_OWNER, time.num_milliseconds());
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

        println!("{}: collecting results (will wait for final generation)...", LOG_OWNER);
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

