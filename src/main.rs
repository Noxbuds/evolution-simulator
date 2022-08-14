use creature::Creature;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent};
use vec2::Vec2;

use crate::spring::Spring;

mod particle;
mod vec2;
mod spring;
mod cell;
mod creature;

pub struct App {
    gl: GlGraphics,
    creature: Creature,
    floor_y: f64,
    sub_steps: i32,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_, gl| {
            clear(BG, gl);
        });

        let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 8.0);

        for particle in &self.creature.particles {
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform
                    .trans(particle.position.x - 4.0, particle.position.y - 4.0);

                rectangle(color, square, transform, gl);
            });
        }

        let springs: Vec<&Spring> = self.creature.cells.iter().flat_map(|cell| {
            &cell.springs
        }).collect();

        for spring in springs {
            self.gl.draw(args.viewport(), |c, gl| {
                let points = [
                    self.creature.particles[spring.a_id].position.x,
                    self.creature.particles[spring.a_id].position.y,
                    self.creature.particles[spring.b_id].position.x,
                    self.creature.particles[spring.b_id].position.y,
                ];

                line(color, 1.0, points, c.transform, gl);
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            for particle in self.creature.particles.iter_mut() {
                particle.accelerate(Vec2 { x: 0.0, y: 60.0 });

                if particle.position.y > self.floor_y {
                    particle.position.y = self.floor_y;
                }
            }

            self.creature.update(dt);
        }
    }
}

fn main() {
    let opengl = OpenGL::V2_1;

    let mut window: GlutinWindow = WindowSettings::new("evolution simulator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let creature = Creature::new(4, 40.0);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        creature,
        floor_y: 500.0,
        sub_steps: 4,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
