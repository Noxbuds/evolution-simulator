use crate::particle::Particle;

pub struct Spring {
    pub a_id: usize,
    pub b_id: usize,
    pub length: f64,
    pub k: f64,
}

impl Spring {
    pub fn apply(&self, particles: &mut Vec<Particle>) {
        let dir = particles[self.a_id].position - particles[self.b_id].position;
        let dist = dir.len();
        //print!("dist {} :: ", dist);

        // F = -kx, x is extension ie difference between length & target length
        let force_mag = self.k * (dist - self.length);

        let unit_dir = dir / dist;
        particles[self.a_id].add_force(unit_dir * -force_mag);
        particles[self.b_id].add_force(unit_dir * force_mag);
    }
}

