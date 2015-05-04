extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate itertools;

mod display;
mod world;

use docopt::Docopt;
use std::thread::sleep_ms;

use world::World;

static USAGE: &'static str = "
Usage:
    golr [options]

Options:
    -h <height>, --height <height>  World height, points                 [default: 25]
    -w <width>, --width <width>     World width, points                  [default: 80]
    -p <period>, --period <period>  World generational change period, ms [default: 350]
";

#[derive(RustcDecodable, Debug)]
struct CliArgs {
    flag_width: isize,
    flag_height: isize,
    flag_period: u32,
}

fn main() {
    let args = Docopt::new(USAGE).and_then(|d| d.decode::<CliArgs>())
                                 .unwrap_or_else(|e| e.exit());
    let mut world = World::new(args.flag_width, args.flag_height).seed();
    loop {
        println!("\x1b[2J\n{}", world.evolve());
        std::thread::sleep_ms(args.flag_period);
    }
}
