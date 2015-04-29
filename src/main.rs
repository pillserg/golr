extern crate argparse;

mod display;
mod world;

use std::thread::sleep_ms;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut width = 10;
    let mut height = 10;
    let mut fps = 33;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Run golr.");
        ap.refer(&mut height).add_option(
            &["-h", "--height"],
            Store,
            "World height"
        );
        ap.refer(&mut width).add_option(
            &["-w", "--width"],
            Store,
            "World width"
        );
        ap.refer(&mut fps).add_option(
            &["-f", "--fps"],
            Store,
            "FPS"
        );
        ap.parse_args_or_exit();
    }

    let mut world = world::World::new(width, height);

    world.randomize();

    loop {
        world = world.evolve();
        println!("\x1b[2J\n{}", world);
        std::thread::sleep_ms(fps);
    }
}
