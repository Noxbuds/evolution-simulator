use crate::{cell::Cell, particle::Particle, vec2::Vec2, dna::CreatureDna, config::CreatureConfig};

pub struct Creature {
    pub particles: Vec<Particle>,
    pub cells: Vec<Option<Cell>>,
    pub size: usize,
}

impl Creature {
    pub fn update(&mut self, dt: f64) {
        let mut discharges: Vec<((usize, usize), f64)> = vec![];
        for cell in self.cells.iter_mut() {
            if let Some(cell) = cell {
                cell.update(&mut self.particles, dt);
                cell.charge_model.update(dt);

                let discharge = cell.charge_model.get_discharge();
                if discharge > 0.0 {
                    discharges.push((cell.pos, discharge));
                }
            }
        }

        for (pos, discharge) in discharges.iter() {
            let (row, col) = *pos;
            let neighbors = [
                (row, col + 1),
                (row, col - 1),
                (row + 1, col),
                (row - 1, col),
            ];
            for (row, col) in neighbors {
                if col > self.size - 1 {
                    continue;
                }
                if let Some(cell) = self.cells.get_mut(col + row * self.size) {
                    if let Some(cell) = cell {
                        cell.charge_model.charge(*discharge);
                    }
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

    pub fn new(options: CreatureConfig, dna: CreatureDna) -> Option<Creature> {
        let mut particles = Vec::with_capacity(options.size * options.size);
        for row in 0..options.size + 1 {
            for col in 0..options.size + 1 {
                let (x, y) = (col as f64 * options.cell_size, row as f64 * options.cell_size);

                particles.push(Particle {
                    position: Vec2 { x: x + 10.0, y },
                    old_position: Vec2 { x: x + 10.0, y },
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
                    (row, col),
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

