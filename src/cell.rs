use crate::{spring::Spring, particle::Particle, dna::CellDna, charge::{ChargeModel, pulse::Pulse, action_potential::ActionPotential}, config::CreatureConfig};

pub struct Cell {
    pub dna: CellDna,
    pub springs: [Spring; 6],
    pub charge_model: Box<dyn ChargeModel + Send>,
    pub pos: (usize, usize),
}

impl Cell {
    pub fn update(&mut self, particles: &mut Vec<Particle>, _dt: f64) {
        let charge = self.charge_model.get_charge();
        let len_mult = 1.0 + charge * self.dna.reactivity;

        for spring in self.springs.iter_mut() {
            spring.apply(particles);
            spring.length = spring.start_length * len_mult;
        }
    }

    pub fn new(cell_ids: [usize; 4], options: CreatureConfig, dna: CellDna, pos: (usize, usize)) -> Cell {
        let diagonal = (options.cell_size * options.cell_size * 2.0).sqrt();
        let springs = [
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[1],
                k: dna.toughness,
                length: options.cell_size,
                start_length: options.cell_size,
            },
            Spring {
                a_id: cell_ids[1],
                b_id: cell_ids[2],
                k: dna.toughness,
                length: options.cell_size,
                start_length: options.cell_size,
            },
            Spring {
                a_id: cell_ids[2],
                b_id: cell_ids[3],
                k: dna.toughness,
                length: options.cell_size,
                start_length: options.cell_size,
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[0],
                k: dna.toughness,
                length: options.cell_size,
                start_length: options.cell_size,
            },
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[2],
                k: dna.toughness,
                length: diagonal,
                start_length: diagonal,
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[1],
                k: dna.toughness,
                length: diagonal,
                start_length: diagonal,
            },
        ];

        let charge_model: Box<dyn ChargeModel + Send> = if dna.charge_rate > options.pulse_threshold {
            Box::new(Pulse::new(
                dna.charge_rate * 0.5,
                options.charge_threshold,
                options.discharge_threshold,
            ))
        } else {
            Box::new(ActionPotential::new(
                options.discharge_threshold,
                options.charge_accel,
                dna.conductivity,
            ))
        };

        Cell {
            dna,
            springs,
            charge_model,
            pos,
        }
    }
}

