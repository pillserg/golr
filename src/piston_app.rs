use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use world::World;


static OFFSET: u32 = 100;

struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    cell_size: u32,
    world: World
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_GOLOR: [f32; 4] = [0.150, 0.164, 0.176, 1.0];
        const CELL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let cells = self.world.get_generation();
        let cell_size = self.cell_size as f64;
        let offset = OFFSET as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG_GOLOR, gl);

            for cell in cells {
                let (x, y): (isize, isize) = *cell;
                let x = x as f64;
                let y = y as f64;
                Rectangle::new(CELL_COLOR)
                .draw([(x * cell_size) + offset, (y * cell_size) + offset, cell_size, cell_size],
                      &c.draw_state,
                      c.transform,
                      gl
                );
            }
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        self.world.evolve();
    }
}

pub fn start_piston_app(world: World, period: u64, cell_size: u32, gl_version: u32) {
    let opengl = match gl_version {
        30 => OpenGL::_3_0,
        _ => OpenGL::_3_2
    };
    let (width, height) = world.get_size();
    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "G-O-L-R",
            [width as u32 * cell_size + 2 * OFFSET, height as u32 * cell_size + 2 * OFFSET]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cell_size: cell_size,
        world: world
    };

    let ups = 1000/period;

    for e in window.events().max_fps(30).ups(ups) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
