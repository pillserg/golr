extern crate argparse;
use std::thread::sleep_ms;
use std::process::exit;
use std::env;
use argparse::{ArgumentParser, StoreTrue, Store};

mod display;
mod world;
use world::World;

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

    let mut world: World = World::new(width, height);
    
    world.randomize();

    display::clear_screen();

    loop {
        world = world.evolve();
        display::draw(&world);
        std::thread::sleep_ms(fps);
    }
}

