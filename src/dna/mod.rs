use rand::random;

use crate::config::MutationConfig;

const NUM_FIELDS: f64 = 5.0;

#[derive(Clone, Copy)]
pub struct CellDna {
    pub conductivity: f64,
    pub reactivity: f64,
    pub toughness: f64,
    pub active: f64,
    pub charge_rate: f64,
}

pub type CreatureDna = Vec<CellDna>;

pub fn generate_dna(length: usize) -> CreatureDna {
    let mut dna: CreatureDna = Vec::new();

    for _ in 0..length {
        dna.push(CellDna {
            conductivity: rand::random::<f64>() * 1.5,
            reactivity: rand::random::<f64>() * 0.2,
            toughness: 1000.0 * (rand::random::<f64>() + 1.0),
            active: rand::random(),
            charge_rate: rand::random::<f64>() * 2.0,
        })
    }

    dna
}

fn mutate_cell(cell: &mut CellDna, field: i32, config: MutationConfig) {
    let multiplier = (random::<f64>() * 2.0 - 1.0) * config.strength;
    match field {
        1 => cell.reactivity *= multiplier,
        // 2 => cell.toughness *= multiplier,
        3 => cell.active *= multiplier,
        4 => cell.charge_rate *= multiplier,
        _ => cell.conductivity *= multiplier,
    }
}

pub fn mutate_dna(dna: &CreatureDna, config: MutationConfig) -> CreatureDna {
    let mut new_dna = dna.clone();

    let chance_roll = random::<f64>();
    if chance_roll > config.chance {
        let id = random::<f64>() * new_dna.len() as f64;
        let field = random::<f64>() * NUM_FIELDS;

        mutate_cell(&mut new_dna[id as usize], field as i32, config);
    }

    new_dna
}

