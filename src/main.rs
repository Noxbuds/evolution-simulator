use charge::action_potential::ActionPotential;
use creature::{Creature, CellOptions};
use dna::{CellDna, CreatureDna};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent};
use renderers::solid::render_solid;
use world::World;

mod particle;
mod vec2;
mod spring;
mod cell;
mod creature;
mod world;
mod renderers;
mod dna;
mod charge;

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

fn generate_dna(length: usize) -> CreatureDna {
    let mut dna: CreatureDna = Vec::new();

    for _ in 0..length {
        dna.push(CellDna {
            conductivity: rand::random(),
            reactivity: rand::random(),
            toughness: 1000.0 * (rand::random::<f64>() + 1.0),
            active: rand::random(),
            charge_rate: rand::random::<f64>() * 2.0,
        })
    }

    dna
}

fn main() {
    let opengl = OpenGL::V2_1;

    let mut window: GlutinWindow = WindowSettings::new("evolution simulator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let creature_size = 8;
    let dna = generate_dna(creature_size * creature_size);

    let cell_options = CellOptions {
        size: creature_size,
        node_damping: 2e-3,
        cell_size: 40.0,
        pulse_threshold: 1.9,
        charge_threshold: 1.0,
        discharge_threshold: 1.1,
        charge_accel: 200.0,
        node_mass: 2.0,
    };
    let creature = Creature::new(cell_options, dna).unwrap();

    let world = World {
        creatures: vec![creature],
        ground_y: 500.0,
        gravity: 100.0,
        render_passes: vec![Box::new(|world, args, gl| {
            render_solid(world, args, gl)
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
