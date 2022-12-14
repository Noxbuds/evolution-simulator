use std::ops::Range;

#[derive(Copy, Clone)]
pub struct WorldConfig {
    pub ground_y: f64,
    pub ground_friction: f64,
    pub gravity: f64,
}

#[derive(Copy, Clone)]
pub struct CreatureConfig {
    pub size: usize,
    pub cell_size: f64,
    pub pulse_threshold: f64,
    pub charge_threshold: f64,
    pub discharge_threshold: f64,
    pub charge_accel: f64,
    pub active_threshold: f64,
    pub node_damping: f64,
    pub node_mass: f64,
}

#[derive(Copy, Clone)]
pub struct MutationRange {
    pub min: f64,
    pub max: f64,
}

#[derive(Copy, Clone)]
pub struct MutationConfig {
    pub chance: f64,
    pub strength: f64,
    pub conductivity: MutationRange,
    pub reactivity: MutationRange,
    pub toughness: MutationRange,
    pub active: MutationRange,
    pub charge_rate: MutationRange,
}

#[derive(Clone, Copy)]
pub struct SimulationConfig {
    pub world_config: WorldConfig,
    pub creature_config: CreatureConfig,
    pub mutation_config: MutationConfig,
    pub creature_count: i32, 
    pub timestep: f64,
    pub sub_steps: i32,
    pub sim_time: f64,
    pub threads: i32,
}

impl SimulationConfig {
    pub fn default() -> SimulationConfig {
        let creature_config = CreatureConfig {
            size: 6,
            node_damping: 4e-2,
            cell_size: 40.0,
            pulse_threshold: 1.9,
            charge_threshold: 1.0,
            discharge_threshold: 1.1,
            charge_accel: 300.0,
            active_threshold: 0.2,
            node_mass: 2.0,
        };
        let world_config = WorldConfig {
            ground_y: creature_config.cell_size * creature_config.size as f64 + 10.0,
            ground_friction: 200.0,
            gravity: 800.0,
        };
        let mutation_config = MutationConfig {
            chance: 0.2,
            strength: 0.5,
            conductivity: mutation_range(0.0, 2.5),
            reactivity: mutation_range(0.0, 0.4),
            toughness: mutation_range(1000.0, 2000.0),
            active: mutation_range(0.0, 1.0),
            charge_rate: mutation_range(0.0, 2.0),
        };

        SimulationConfig {
            world_config,
            creature_config,
            mutation_config,
            creature_count: 1000,
            timestep: 0.01,
            sub_steps: 4,
            sim_time: 10.0,
            threads: 6,
        }
    }
}

fn mutation_range(min: f64, max: f64) -> MutationRange {
    MutationRange { min, max }
}

