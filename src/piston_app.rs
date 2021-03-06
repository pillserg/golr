use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use world::World;


static OFFSET: u32 = 10;
static MAX_FPS: u64 = 30;


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

        let cells = self.world.generation();
        let cell_size = self.cell_size as f64;
        let offset = OFFSET as f64 * cell_size;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG_GOLOR, gl);

            for cell in cells {
                let (x, y) = *cell;
                Rectangle::new(CELL_COLOR)
                .draw(
                    [(x as f64 * cell_size) + offset, (y as f64 * cell_size) + offset, cell_size, cell_size],
                    &c.draw_state,
                    c.transform,
                    gl)}
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        self.world.evolve();
    }
}


pub fn start_piston_app(world: World, period: u64, cell_size: u32, gl_version: u32) {
    let opengl = match gl_version {
        30 => OpenGL::V3_0,
        _ => OpenGL::V3_2
    };
    let (width, height) = world.size();

    let mut window: Window = WindowSettings::new(
            "G-O-L-R",
            [width as u32 * cell_size + 4 * OFFSET, 
             height as u32 * cell_size + 4 * OFFSET]
        )
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        cell_size: cell_size,
        world: world
    };

    let ups = 1000/period;

    let mut events = window.events().max_fps(MAX_FPS).ups(ups);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
