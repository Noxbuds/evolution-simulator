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

    pub fn new(size: usize, cell_size: f64) -> Creature {
        let mut particles = Vec::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                let (x, y) = (col as f64 * cell_size, row as f64 * cell_size);

                particles.push(Particle {
                    position: Vec2 { x, y },
                    old_position: Vec2 { x, y},
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    mass: 5.0,
                    damping: 1e-3,
                })
            }
        }

        let side_length = size - 1;
        let mut cells = Vec::with_capacity(side_length * side_length);
        for row in 0..side_length {
            for col in 0..side_length {
                let ids = [
                    Creature::get_cell_id(row, col, size),
                    Creature::get_cell_id(row, col + 1, size),
                    Creature::get_cell_id(row + 1, col + 1, size),
                    Creature::get_cell_id(row + 1, col, size),
                ];

                cells.push(Cell::new(ids, cell_size, 1500.0));
            }
        }

        Creature {
            particles,
            cells,
        }
    }
}

