use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs, EventSettings, WindowSettings, Events, RenderEvent, UpdateEvent};

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            rectangle(color, square, transform, gl);
        });
    }

    fn update(&self, args: &UpdateArgs) {
    }
}

fn main() {
    // my vm only supports 2.1 lol
    let opengl = OpenGL::V2_1;

    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
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
