use crate::{cell::Cell, particle::Particle, vec2::Vec2};

pub struct Creature {
    pub particles: Vec<Particle>,
    pub cells: Vec<Cell>,
}

impl Creature {
    pub fn update(&mut self, dt: f64) {
        for cell in &self.cells {
            cell.apply(&mut self.particles)
        }

        for particle in self.particles.iter_mut() {
            particle.integrate(dt)
        }
    }

    fn get_cell_id(row: usize, col: usize, side_length: usize) -> usize {
        row * side_length + col
    }

    pub fn new(size: usize, cell_size: f64, node_damping: f64, node_mass: f64, muscle_strength: f64) -> Creature {
        let mut particles = Vec::with_capacity(size * size);
        for row in 0..size + 1 {
            for col in 0..size + 1 {
                let (x, y) = (col as f64 * cell_size, row as f64 * cell_size);

                particles.push(Particle {
                    position: Vec2 { x, y },
                    old_position: Vec2 { x, y},
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    mass: node_mass,
                    damping: node_damping,
                })
            }
        }

        let mut cells = Vec::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                let ids = [
                    Creature::get_cell_id(row, col, size + 1),
                    Creature::get_cell_id(row, col + 1, size + 1),
                    Creature::get_cell_id(row + 1, col + 1, size + 1),
                    Creature::get_cell_id(row + 1, col, size + 1),
                ];

                cells.push(Cell::new(ids, cell_size, muscle_strength));
            }
        }

        Creature {
            particles,
            cells,
        }
    }
}

