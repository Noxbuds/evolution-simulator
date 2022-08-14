use crate::{spring::Spring, particle::Particle, dna::CellDna, charge::{ChargeModel, pulse::Pulse, action_potential::ActionPotential}, creature::CellOptions};

pub struct Cell {
    pub dna: CellDna,
    pub springs: [Spring; 6],
    pub charge_model: Box<dyn ChargeModel>,
}

impl Cell {
    pub fn apply(&self, particles: &mut Vec<Particle>, _dt: f64) {
        for spring in &self.springs {
            spring.apply(particles);
        }
    }

    pub fn new(cell_ids: [usize; 4], options: CellOptions, dna: CellDna) -> Cell {
        let diagonal = (options.cell_size * options.cell_size * 2.0).sqrt();
        let springs = [
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[1],
                k: dna.toughness,
                length: options.cell_size
            },
            Spring {
                a_id: cell_ids[1],
                b_id: cell_ids[2],
                k: dna.toughness,
                length: options.cell_size
            },
            Spring {
                a_id: cell_ids[2],
                b_id: cell_ids[3],
                k: dna.toughness,
                length: options.cell_size
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[0],
                k: dna.toughness,
                length: options.cell_size
            },
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[2],
                k: dna.toughness,
                length: diagonal
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[1],
                k: dna.toughness,
                length: diagonal
            },
        ];

        let charge_model: Box<dyn ChargeModel> = if dna.charge_rate > options.pulse_threshold {
            Box::new(Pulse::new(dna.charge_rate * 0.5, options.charge_threshold, options.discharge_threshold))
        } else {
            Box::new(ActionPotential::new(options.discharge_threshold, options.charge_accel))
        };

        Cell {
            dna,
            springs,
            charge_model,
        }
    }
}

