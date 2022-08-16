use config::SimulationConfig;
use creature::Creature;
use dna::CreatureDna;
use evolution_controller::EvolutionController;
use fitness::fitness_distance;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent, ButtonEvent, Key, ButtonState, ButtonArgs, Button};
use renderers::{solid::render_solid, RenderPass};
use statistics::{StatisticsPanel, fitness_chart::FitnessChart};
use vec2::Vec2;
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
mod statistics;

const LOG_OWNER: &str = "[main]";

pub struct App {
    gl: GlGraphics,
    world: World,
    sub_steps: i32,
    render_passes: Vec<RenderPass>,
    config: SimulationConfig,
    evolution_controller: EvolutionController,
    statistics_panels: Vec<Box<dyn StatisticsPanel>>,
}

impl App {
    fn from_config(config: SimulationConfig, gl: GlGraphics) -> App {
        let world = World::from_config(config.world_config);
        let fitness = Box::new(fitness_distance);
        let percentiles: Vec<f64> = vec![25.0, 50.0, 75.0, 100.0];

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
            statistics_panels: vec![
                Box::new(FitnessChart::new(percentiles, 20.0))
            ],
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        for pass in &self.render_passes {
            pass(&self.world, args, &mut self.gl);
        }

        let panel_count = self.statistics_panels.len() as f64;
        let panel_width = args.window_size[0] / panel_count;
        let panel_height = args.window_size[1] * 0.5;
        let panel_y = args.window_size[1] * 0.5;

        for (i, panel) in self.statistics_panels.iter().enumerate() {
            let position = Vec2 {
                x: i as f64 * panel_width,
                y: panel_y,
            };

            let size = Vec2 {
                x: panel_width,
                y: panel_height,
            };

            panel.render(args.viewport(), &mut self.gl, position, size);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.world.update(dt);
        }

        let results = self.evolution_controller.try_get_results();
        if results.len() > 0 {
            for panel in self.statistics_panels.iter_mut() {
                panel.gather_statistics(&results);
            }
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
            Ok(results) => {
                self.world.reset();
                println!("{}: controller stopped, previewing...", LOG_OWNER);

                let first_result = results.last();
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

    let mut window: GlutinWindow = WindowSettings::new("evolution simulator", [1920, 1080])
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
