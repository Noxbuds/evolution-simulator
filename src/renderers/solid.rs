use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{world::World, creature::Creature, charge::ChargeModel, cell::Cell};

fn get_position(row: usize, col: usize, creature: &Creature) -> [f64; 2] {
    let position = creature.particles[Creature::get_cell_id(row, col, creature.size + 1)].position;
    [position.x, position.y]
}

fn get_color(cell: &Cell) -> [f32; 4] {
    let charge = cell.charge_model.get_charge() as f32 * 0.2;
    let toughness = cell.dna.toughness as f32 / 2000.0;
    let conductivity = cell.dna.conductivity as f32 * 0.5;

    [
        toughness * conductivity + charge * 2.0,
        toughness + charge,
        toughness,
        1.0,
    ]
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
                    let color = get_color(&cell);

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

