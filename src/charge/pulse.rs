use super::ChargeModel;

pub struct Pulse {
    charge_rate: f64,
    charge: f64,
    threshold: f64,
    reset_threshold: f64,
}

impl Pulse {
    pub fn new(charge_rate: f64, threshold: f64, reset_threshold: f64) -> Pulse {
        Pulse {
            charge_rate,
            charge: 0.0,
            threshold,
            reset_threshold,
        }
    }
}

impl ChargeModel for Pulse {
    fn get_charge(&self) -> f64 {
        self.charge
    }

    fn get_discharge(&self) -> f64 {
        if self.charge > self.threshold {
            self.charge - self.threshold
        } else {
            0.0
        }
    }

    fn update(&mut self, dt: f64) {
        self.charge += self.charge_rate * dt;
        if self.charge > self.reset_threshold {
            self.charge = 0.0;
        }
    }

    fn charge(&mut self, _amount: f64) {}

    fn copy(&self) -> Box<dyn ChargeModel> {
        Box::new(Pulse {
            charge: self.charge,
            charge_rate: self.charge_rate,
            threshold: self.threshold,
            reset_threshold: self.reset_threshold,
        })
    }
}

