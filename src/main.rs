use creature::Creature;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent};
use renderers::wireframe::render_wireframe;
use world::World;

mod particle;
mod vec2;
mod spring;
mod cell;
mod creature;
mod world;
mod renderers;

pub struct App {
    gl: GlGraphics,
    world: World,
    sub_steps: i32,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.world.render(args, &mut self.gl);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.world.update(dt);
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

    let creature = Creature::new(4, 40.0, 2e-3, 2.0, 1500.0);

    let world = World {
        creatures: vec![creature],
        ground_y: 500.0,
        render_passes: vec![Box::new(|world, args, gl| {
            render_wireframe(world, args, gl)
        })],
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world,
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
