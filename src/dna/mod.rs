use rand::random;

use crate::config::{MutationConfig, MutationRange};

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

fn generate_field(range: MutationRange) -> f64 {
    range.min + rand::random::<f64>() * (range.max - range.min)
}

pub fn generate_dna(length: usize, config: MutationConfig) -> CreatureDna {
    let mut dna: CreatureDna = Vec::new();

    for _ in 0..length {
        dna.push(CellDna {
            conductivity: generate_field(config.conductivity),
            reactivity: generate_field(config.reactivity),
            toughness: generate_field(config.toughness),
            active: generate_field(config.active),
            charge_rate: generate_field(config.charge_rate),
        })
    }

    dna
}

fn apply_mutation(value: f64, multiplier: f64, range: MutationRange) -> f64 {
    let result = value * multiplier;
    result.clamp(range.min, range.max)
}

fn mutate_cell(cell: &mut CellDna, field: i32, config: MutationConfig) {
    let multiplier = (random::<f64>() * 2.0 - 1.0) * config.strength;
    match field {
        1 => cell.reactivity = apply_mutation(cell.reactivity, multiplier, config.reactivity),
        2 => cell.toughness = apply_mutation(cell.toughness, multiplier, config.toughness),
        3 => cell.active = apply_mutation(cell.active, multiplier, config.active),
        4 => cell.charge_rate = apply_mutation(cell.charge_rate, multiplier, config.charge_rate),
        _ => cell.conductivity = apply_mutation(cell.conductivity, multiplier, config.conductivity),
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

