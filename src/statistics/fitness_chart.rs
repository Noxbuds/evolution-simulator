use graphics::Viewport;

use crate::{vec2::Vec2, evolution_controller::CreatureResult};

use super::StatisticsPanel;

pub struct FitnessChart {
    statistics: Vec<Vec<f64>>,
    percentiles: Vec<f64>,
    interval: f64,
}

impl FitnessChart {
    pub fn new(percentiles: Vec<f64>, interval: f64) -> FitnessChart {
        FitnessChart {
            statistics: vec![],
            percentiles,
            interval
        }
    }
}

fn remap_range(value: f64, lowest: f64, highest: f64) -> f64 {
    (value - lowest) / (highest - lowest)
}

impl StatisticsPanel for FitnessChart {
    fn gather_statistics(&mut self, results: &Vec<CreatureResult>) {
        let mut gen_stats: Vec<f64> = Vec::new();
        for percentile in &self.percentiles {
            let id = results.len() as f64 * percentile / 100.0 - 1.0;

            if let Some((_, fitness)) = results.get(id as usize) {
                gen_stats.push(*fitness);
            }
        }

        self.statistics.push(gen_stats);
    }

    fn render(&self, viewport: Viewport, gl: &mut opengl_graphics::GlGraphics, position: Vec2, size: Vec2) {
        let line_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let stat_width = size.x / (self.statistics.len() - 1) as f64;
        let stat_height = size.y * 0.5;
        let y_off = size.y + position.y;

        if self.statistics.len() < 1 {
            return;
        }

        let mut highest_fitness = 0.0;
        let mut lowest_fitness = 200.0;
        for percentiles in &self.statistics {
            for p in percentiles {
                if p > &highest_fitness {
                    highest_fitness = *p;
                }
                if p < &lowest_fitness {
                    lowest_fitness = *p;
                }
            }
        }

        let interval_count = (highest_fitness - lowest_fitness) / self.interval + 1.0;
        for i in 0..=interval_count as usize {
            let y = ((i as f64 * self.interval + lowest_fitness) / self.interval).round() * self.interval;
            let points = [
                position.x,
                y_off - remap_range(y, lowest_fitness, highest_fitness) * stat_height,
                position.x + size.x,
                y_off - remap_range(y, lowest_fitness, highest_fitness) * stat_height,
            ];

            gl.draw(viewport, |c, gl| {
                graphics::line([0.5, 0.5, 0.5, 1.0], 1.0, points, c.transform, gl);
            });
        }

        for i in 0..self.statistics.len() - 1 {
            for p in 0..self.statistics[i].len() {
                let x_a = i as f64 * stat_width;
                let y_a = remap_range(self.statistics[i][p] as f64, lowest_fitness, highest_fitness);

                let x_b = (i + 1) as f64 * stat_width;
                let y_b = remap_range(self.statistics[i + 1][p] as f64, lowest_fitness, highest_fitness);

                let points = [
                    x_a,
                    y_off - y_a * stat_height,
                    x_b,
                    y_off - y_b * stat_height,
                ];

                gl.draw(viewport, |c, gl| {
                    graphics::line(line_color, 2.0, points, c.transform, gl);
                });
            }
        }
    }
}

