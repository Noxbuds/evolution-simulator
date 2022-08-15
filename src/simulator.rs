use std::{sync::{mpsc::{Sender, self, Receiver}, Arc}, thread};

use crate::{config::SimulationConfig, world::World, dna::CreatureDna, creature::Creature, fitness::FitnessFunction, evolution_controller::CreatureResult};

const LOG_OWNER: &str = "[simulator]";

pub enum SimulatorMessage {
    Run(Vec<CreatureDna>),
    Results(Vec<CreatureResult>),
}

pub struct Simulator {
    message_sender: Sender<SimulatorMessage>,
    pub message_receiver: Receiver<SimulatorMessage>,
}

impl Simulator {
    pub fn from_config(config: SimulationConfig, fitness_func: Arc<FitnessFunction>) -> Simulator {
        let mut world = World::from_config(config.world_config);

        let (sim_tx, thread_rx) = mpsc::channel::<SimulatorMessage>();
        let (thread_tx, sim_rx) = mpsc::channel::<SimulatorMessage>();

        let fitness = fitness_func.clone();

        thread::spawn(move || loop {
            if let Ok(message) = thread_rx.recv() {
                match message {
                    SimulatorMessage::Run(all_dna) => {
                        world.reset();
                        for dna in all_dna.iter() {
                            let creature = Creature::new(config.creature_config, dna.clone());
                            if let Some(creature) = creature {
                                world.add_creature(creature);
                            }
                        }

                        let dt = config.timestep / config.sub_steps as f64;
                        let total_steps = config.sim_time / dt;
                        for _ in 0..total_steps as i32 {
                            world.update(dt)
                        }

                        let fitnesses = fitness(&world.creatures);
                        let results = all_dna.iter().zip(fitnesses).map(|result| {
                            (result.0.clone(), result.1)
                        }).collect();
                        let result = thread_tx.send(SimulatorMessage::Results(results));
                        if let Err(msg) = result {
                            eprintln!("{}: error while trying to send result: {:?}", LOG_OWNER, msg);
                        }
                    },
                    _ => {}
                }
            }
        });

        Simulator {
            message_sender: sim_tx,
            message_receiver: sim_rx,
        }
    }

    pub fn start(&self, dna: &[CreatureDna]) {
        let message = SimulatorMessage::Run(dna.to_vec());
        let result = self.message_sender.send(message);
        if let Err(msg) = result {
            eprintln!("{}: error while starting - {:?}", LOG_OWNER, msg);
        }
    }
}

