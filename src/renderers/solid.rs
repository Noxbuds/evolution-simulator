use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{world::World, creature::Creature};

fn get_position(row: usize, col: usize, creature: &Creature) -> [f64; 2] {
    let position = creature.particles[Creature::get_cell_id(row, col, creature.size + 1)].position;
    [position.x, position.y]
}

pub fn render_solid(world: &World, args: &RenderArgs, gl: &mut GlGraphics) {
    use graphics::*;

    const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    gl.draw(args.viewport(), |_, gl| {
        clear(BG, gl);
    });

    for creature in world.creatures.iter() {
        for row in 0..creature.size {
            for col in 0..creature.size {
                if let Some(cell) = &creature.cells[Creature::get_cell_id(row, col, creature.size)] {
                    let charge = cell.charge_model.get_charge();
                    let brightness: f32 = charge as f32 * 0.5 + 0.5;
                    let color = [brightness, brightness, brightness, 1.0];

                    let points = [
                        get_position(row, col, creature),
                        get_position(row, col + 1, creature),
                        get_position(row + 1, col + 1, creature),
                        get_position(row + 1, col, creature),
                    ];

                    gl.draw(args.viewport(), |c, gl| {
                        polygon(color, &points, c.transform, gl);
                    });
                }
            }
        }
    }
}

