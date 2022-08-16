use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{world::World, spring::Spring};

pub fn render_wireframe(world: &World, args: &RenderArgs, gl: &mut GlGraphics) {
    use graphics::*;

    const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    gl.draw(args.viewport(), |_, gl| {
        clear(BG, gl);
    });

    let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let square = rectangle::square(0.0, 0.0, 8.0);

    for creature in world.creatures.iter() {
        for particle in &creature.particles {
            gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform
                    .trans(particle.position.x - 4.0, particle.position.y - 4.0);

                rectangle(color, square, transform, gl);
            });
        }

        let springs: Vec<&Spring> = creature.cells.iter()
            .filter_map(|cell| {
                if let Some(cell) = cell {
                    Some(cell)
                } else {
                    None
                }
            })
            .flat_map(|cell| {
                &cell.springs
            })
            .collect();

        for spring in springs {
            gl.draw(args.viewport(), |c, gl| {
                let points = [
                    creature.particles[spring.a_id].position.x.clone(),
                    creature.particles[spring.a_id].position.y.clone(),
                    creature.particles[spring.b_id].position.x.clone(),
                    creature.particles[spring.b_id].position.y.clone(),
                ];

                line(color, 1.0, points, c.transform, gl);
            });
        }
    }
}

