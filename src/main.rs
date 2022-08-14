use config::SimulationConfig;
use creature::Creature;
use dna::{CellDna, CreatureDna};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent, ButtonEvent, Key};
use renderers::{solid::render_solid, RenderPass};
use simulator::Simulator;
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
mod config;
mod simulator;

pub struct App {
    gl: GlGraphics,
    world: World,
    sub_steps: i32,
    render_passes: Vec<RenderPass>,
}

impl App {
    fn from_config(config: SimulationConfig, gl: GlGraphics) -> App {
        let world = World::from_config(config.world_config);

        App {
            gl,
            world,
            sub_steps: config.sub_steps,
            render_passes: vec![
                Box::new(|world, args, gl| {
                    render_solid(world, args, gl)
                })
            ],
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        for pass in &self.render_passes {
            pass(&self.world, args, &mut self.gl);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.world.update(dt);
        }
    }

    fn set_config(&mut self, config: SimulationConfig) {
        self.world.ground_y = config.world_config.ground_y;
        self.world.ground_friction = config.world_config.ground_friction;
        self.world.gravity = config.world_config.gravity;
    }
}

fn main() {
    let opengl = OpenGL::V2_1;

    let mut window: GlutinWindow = WindowSettings::new("evolution simulator", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let config = SimulationConfig::default();
    let mut app = App::from_config(config, GlGraphics::new(opengl));
    let mut simulator = Simulator::from_config(config);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            match args.button {
                piston::Button::Keyboard(key) => {
                    if key == Key::Space {
                        if simulator.is_running() {
                            simulator.stop();
                        } else {
                            simulator.start();
                        }
                    }
                },
                _ => {}
            }
        }
    }
}
