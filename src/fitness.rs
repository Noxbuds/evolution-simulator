use crate::creature::Creature;

pub type FitnessFunction = Box<dyn Fn(&Vec<Creature>) -> Vec<f64> + Send + Sync>;

pub fn fitness_distance(creatures: &Vec<Creature>) -> Vec<f64> {
    creatures.iter().map(|creature| {
        let mut total: f64 = 0.0;
        for particle in &creature.particles {
            total += particle.position.x;
        }

        total / creature.particles.len() as f64
    }).collect()
}

