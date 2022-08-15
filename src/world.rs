use crate::{creature::Creature, vec2::Vec2, renderers::RenderPass, config::WorldConfig};

pub struct World {
    pub creatures: Vec<Creature>,
    pub ground_y: f64,
    pub ground_friction: f64,
    pub gravity: f64,
}

impl World {
    pub fn update(&mut self, dt: f64) {
        for creature in self.creatures.iter_mut() {
            for particle in creature.particles.iter_mut() {
                particle.accelerate(Vec2 { x: 0.0, y: self.gravity });

                if particle.position.y > self.ground_y {
                    let velocity = particle.position.x - particle.old_position.x;
                    particle.accelerate(Vec2 {
                        x: velocity * -self.ground_friction * self.gravity,
                        y: 0.0,
                    });

                    particle.position.y = self.ground_y;
                }
            }

            creature.update(dt);
        }
    }

    pub fn add_creature(&mut self, creature: Creature) {
        self.creatures.push(creature);
    }

    pub fn reset(&mut self) {
        self.creatures.clear();
    }

    pub fn from_config(config: WorldConfig) -> World {
        World {
            creatures: vec![],
            ground_y: config.ground_y,
            ground_friction: config.ground_friction,
            gravity: config.gravity,
        }
    }
}

