use crate::{cell::Cell, particle::Particle, vec2::Vec2, dna::CreatureDna};

#[derive(Copy, Clone)]
pub struct CellOptions {
    pub size: usize,
    pub cell_size: f64,
    pub pulse_threshold: f64,
    pub charge_threshold: f64,
    pub discharge_threshold: f64,
    pub charge_accel: f64,
    pub node_damping: f64,
    pub node_mass: f64,
}

pub struct Creature {
    pub particles: Vec<Particle>,
    pub cells: Vec<Cell>,
    pub size: usize,
}

impl Creature {
    pub fn update(&mut self, dt: f64) {
        let mut discharges: Vec<(usize, f64)> = vec![];
        for (i, cell) in self.cells.iter_mut().enumerate() {
            cell.apply(&mut self.particles, dt);
            cell.charge_model.update(dt);

            let discharge = cell.charge_model.get_discharge();
            if discharge > 0.0 {
                discharges.push((i, discharge));
            }
        }

        for (i, discharge) in discharges.iter() {
            let neighbors = [i + 1, i - 1, i + self.size, i - self.size];
            for n in neighbors {
                if let Some(cell) = self.cells.get_mut(n) {
                    cell.charge_model.charge(*discharge);
                }
            }
        }

        for particle in self.particles.iter_mut() {
            particle.integrate(dt)
        }
    }

    pub fn get_cell_id(row: usize, col: usize, side_length: usize) -> usize {
        row * side_length + col
    }

    pub fn new(options: CellOptions, dna: CreatureDna) -> Option<Creature> {
        let mut particles = Vec::with_capacity(options.size * options.size);
        for row in 0..options.size + 1 {
            for col in 0..options.size + 1 {
                let (x, y) = (col as f64 * options.cell_size, row as f64 * options.cell_size);

                particles.push(Particle {
                    position: Vec2 { x, y },
                    old_position: Vec2 { x, y},
                    acceleration: Vec2 { x: 0.0, y: 0.0 },
                    mass: options.node_mass,
                    damping: options.node_damping,
                })
            }
        }

        let mut cells = Vec::with_capacity(options.size * options.size);
        for row in 0..options.size {
            for col in 0..options.size {
                let ids = [
                    Creature::get_cell_id(row, col, options.size + 1),
                    Creature::get_cell_id(row, col + 1, options.size + 1),
                    Creature::get_cell_id(row + 1, col + 1, options.size + 1),
                    Creature::get_cell_id(row + 1, col, options.size + 1),
                ];

                let cell_dna = dna.get(Creature::get_cell_id(row, col, options.size))?;
                cells.push(Cell::new(
                    ids,
                    options,
                    cell_dna.clone(),
                ));
            }
        }

        Some(Creature {
            particles,
            cells,
            size: options.size,
        })
    }
}

