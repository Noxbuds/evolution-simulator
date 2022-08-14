use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{creature::Creature, vec2::Vec2, renderers::RenderPass};

pub struct World {
    pub creatures: Vec<Creature>,
    pub ground_y: f64,
    pub render_passes: Vec<RenderPass>,
}

impl World {
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        for pass in self.render_passes.iter() {
            pass(self, args, gl);
        }
    }

    pub fn update(&mut self, dt: f64) {
        for creature in self.creatures.iter_mut() {
            for particle in creature.particles.iter_mut() {
                particle.accelerate(Vec2 { x: 0.0, y: 60.0 });

                if particle.position.y > self.ground_y {
                    particle.position.y = self.ground_y;
                }
            }

            creature.update(dt);
        }
    }
}

