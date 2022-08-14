// the goal of this is to mimic an action potential that appears in
// natural cells, where charge quickly spikes, discharges and then
// recovers to a resting charge
use super::ChargeModel;

pub struct ActionPotential {
    charge: f64,
    old_charge: f64,
    acceleration: f64, // probably not a good name, rate of rate of change of charge
    max_acceleration: f64,
    active: bool,
    threshold: f64,
    rest_charge: f64,
}

impl ActionPotential {
    pub fn new(threshold: f64, max_acceleration: f64) -> ActionPotential {
        ActionPotential {
            charge: 0.0,
            old_charge: 0.0,
            acceleration: 0.0,
            max_acceleration,
            active: false,
            threshold,
            rest_charge: 0.0,
        }
    }
}

impl ChargeModel for ActionPotential {
    fn update(&mut self, dt: f64) {
        // verlet integration for charge
        let new_charge = self.charge * 2.0 - self.old_charge + self.acceleration * dt * dt;

        self.old_charge = self.charge;
        self.charge = new_charge;

        if self.charge > self.threshold {
            self.acceleration = -self.max_acceleration;
        }

        if self.charge < self.rest_charge {
            self.acceleration = self.max_acceleration;
            self.active = false;
        }

        if self.charge > self.rest_charge && !self.active {
            self.charge = self.rest_charge;
            self.old_charge = self.rest_charge;
            self.acceleration = 0.0;
        }
    }

    fn get_charge(&self) -> f64 {
        self.charge
    }

    fn get_discharge(&self) -> f64 {
        if self.charge >= self.threshold {
            self.charge - self.threshold
        } else {
            0.0
        }
    }

    fn charge(&mut self, _amount: f64) {
        if !self.active && self.charge >= self.rest_charge {
            self.acceleration = self.max_acceleration;
            self.active = true;
        }
    }

    fn copy(&self) -> Box<dyn ChargeModel> {
        Box::new(ActionPotential {
            charge: self.charge,
            old_charge: self.old_charge,
            acceleration: self.acceleration,
            max_acceleration: self.max_acceleration,
            threshold: self.threshold,
            active: self.active,
            rest_charge: self.rest_charge,
        })
    }
}

