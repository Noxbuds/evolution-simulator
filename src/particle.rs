use crate::vec2::Vec2;

pub struct Particle {
    pub position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub mass: f64,
    pub damping: f64,
}

impl Particle {
    pub fn integrate(&mut self, dt: f64) {
        let velocity = self.position - self.old_position;
        self.old_position = self.position;
        self.position = self.position + velocity * (1.0 - self.damping) + self.acceleration * dt * dt;
        self.acceleration = Vec2 { x: 0.0, y: 0.0 };
    }

    pub fn accelerate(&mut self, acceleration: Vec2) {
        self.acceleration = self.acceleration + acceleration;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.accelerate(force / self.mass)
    }
}

