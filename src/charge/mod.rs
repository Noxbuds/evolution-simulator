pub mod action_potential;
pub mod pulse;

pub trait ChargeModel {
    fn get_charge(&self) -> f64;
    fn get_discharge(&self) -> f64;
    fn update(&mut self, dt: f64);
    fn charge(&mut self, amount: f64);
    fn copy(&self) -> Box<dyn ChargeModel>;
}

