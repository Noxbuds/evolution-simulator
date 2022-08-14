#[derive(Clone, Copy)]
pub struct CellDna {
    pub conductivity: f64,
    pub reactivity: f64,
    pub toughness: f64,
    pub active: f64,
    pub charge_rate: f64,
}

pub type CreatureDna = Vec<CellDna>;

