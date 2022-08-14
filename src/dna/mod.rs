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
            reactivity: rand::random::<f64>() * 0.1,
            toughness: 1000.0 * (rand::random::<f64>() + 1.0),
            active: rand::random(),
            charge_rate: rand::random::<f64>() * 2.0,
        })
    }

    dna
}

