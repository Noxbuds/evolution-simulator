use graphics::Viewport;
use opengl_graphics::GlGraphics;

use crate::{vec2::Vec2, evolution_controller::CreatureResult};

pub mod fitness_chart;

pub trait StatisticsPanel {
    fn gather_statistics(&mut self, results: &Vec<CreatureResult>);
    fn render(&self, viewport: Viewport, gl: &mut GlGraphics, position: Vec2, size: Vec2);
}

