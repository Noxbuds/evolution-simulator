use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::world::World;

pub type RenderPass = Box<dyn Fn(&World, &RenderArgs, &mut GlGraphics)>;

pub mod wireframe;

