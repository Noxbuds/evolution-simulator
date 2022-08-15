use crate::creature::Creature;

pub type FitnessFunction = Box<dyn Fn(&Vec<Creature>) -> Vec<f64> + Send>;

pub fn fitness_distance(creatures: &Vec<Creature>) -> Vec<f64> {
    creatures.iter().map(|creature| {
        let mut furthest: f64 = 0.0;
        for particle in &creature.particles {
            let distance = particle.position.x;
            if distance > furthest {
                furthest = distance;
            }
        }

        furthest
    }).collect()
}

