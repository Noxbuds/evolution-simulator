use crate::{spring::Spring, particle::Particle};

pub struct Cell {
    pub springs: [Spring; 6],
}

impl Cell {
    pub fn apply(&self, particles: &mut Vec<Particle>) {
        for spring in &self.springs {
            spring.apply(particles);
        }
    }

    pub fn new(cell_ids: [usize; 4], size: f64, strength: f64) -> Cell {
        let diagonal = (size * size * 2.0).sqrt();

        let springs = [
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[1],
                k: strength,
                length: size
            },
            Spring {
                a_id: cell_ids[1],
                b_id: cell_ids[2],
                k: strength,
                length: size
            },
            Spring {
                a_id: cell_ids[2],
                b_id: cell_ids[3],
                k: strength,
                length: size
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[0],
                k: strength,
                length: size
            },
            Spring {
                a_id: cell_ids[0],
                b_id: cell_ids[2],
                k: strength,
                length: diagonal
            },
            Spring {
                a_id: cell_ids[3],
                b_id: cell_ids[1],
                k: strength,
                length: diagonal
            },
        ];

        Cell {
            springs
        }
    }
}

