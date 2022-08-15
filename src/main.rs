use config::SimulationConfig;
use creature::Creature;
use dna::CreatureDna;
use evolution_controller::EvolutionController;
use fitness::fitness_distance;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent, ButtonEvent, Key, ButtonState, ButtonArgs, Button};
use renderers::{solid::render_solid, RenderPass};
use world::World;

extern crate chrono;

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
mod fitness;
mod evolution_controller;

const LOG_OWNER: &str = "[main]";

pub struct App {
    gl: GlGraphics,
    world: World,
    sub_steps: i32,
    render_passes: Vec<RenderPass>,
    config: SimulationConfig,
    evolution_controller: EvolutionController,
}

impl App {
    fn from_config(config: SimulationConfig, gl: GlGraphics) -> App {
        let world = World::from_config(config.world_config);
        let fitness = Box::new(fitness_distance);

        App {
            gl,
            world,
            sub_steps: config.sub_steps,
            render_passes: vec![
                Box::new(|world, args, gl| {
                    render_solid(world, args, gl)
                })
            ],
            config,
            evolution_controller: EvolutionController::new(config, fitness),
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

    fn start_controller(&mut self) {
        let result = self.evolution_controller.start();
        if let Err(msg) = result {
            eprintln!("{}: error while starting controller: {:?}", LOG_OWNER, msg);
        }
    }

    fn stop_controller(&mut self) {
        match self.evolution_controller.stop() {
            Ok(mut results) => {
                self.world.reset();
                println!("{}: controller stopped, previewing...", LOG_OWNER);

                results.sort_by(|(_, a), (_, b)| {
                    a.total_cmp(b)
                });

                let first_result = results.first();
                if let Some((preview, fitness)) = first_result {
                    println!("{}: previewing best creature out of {}, fitness: {}", LOG_OWNER, results.len(), fitness);
                    let creature = Creature::new(self.config.creature_config, preview.clone());
                    if let Some(creature) = creature {
                        self.world.add_creature(creature);
                    }
                }
            },
            Err(msg) => {
                eprintln!("{}: error while stopping controller: {:?}", LOG_OWNER, msg);
            }
        }
    }

    fn handle_input(&mut self, args: &ButtonArgs) {
        if let Button::Keyboard(key) = args.button {
            if key == Key::Space && args.state == ButtonState::Press {
                if self.evolution_controller.is_running() {
                    self.stop_controller();
                } else {
                    self.start_controller();
                }
            }
        }
        match args.button {
            piston::Button::Keyboard(key) => {
                if key == Key::Space && args.state == ButtonState::Press {
                }
            },
            _ => {}
        }
    }

    pub fn set_creatures(&mut self, dna: Vec<CreatureDna>) {
        let creatures = dna.iter().map(|dna| {
            Creature::new(self.config.creature_config, dna.clone())
        });

        self.world.reset();
        for creature in creatures {
            if let Some(creature) = creature {
                self.world.add_creature(creature);
            }
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

    let config = SimulationConfig::default();
    let mut app = App::from_config(config, GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.handle_input(&args);
        }
    }
}
