extern crate rand;
extern crate time;
extern crate docopt;
extern crate itertools;
extern crate rustc_serialize;
extern crate unicode_segmentation;

mod display;
mod world;
mod parser;
mod util;

use std::thread::sleep_ms;
use docopt::Docopt;

use world::World;


static USAGE: &'static str = "
Usage:
    golr [options]

Options:
    --help                                      Show this message
    -h <height>, --height <height>              World height, points                         [default: 25]
    -w <width>, --width <width>                 World width, points                          [default: 80]
    -p <period>, --period <period>              World generational change period, ms         [default: 350]
    -i <inputfile>, --inputfile <inputfile>     Populate world from file
    -r <render>, --render <render>              Choose render engine [0-console | 1-piston]  [default: 0]
";

#[derive(RustcDecodable, Debug)]
struct CliArgs {
    flag_width: isize,
    flag_height: isize,
    flag_period: u64,
    flag_inputfile: String,
    flag_render: isize,
}


fn start_console_loop(world: &mut World, period: u64) {
    print!("\x1b[2J");
    loop {
        let t_start = util::time_ms();
        println!("\x1b[H{}", world.evolve());
        let t_taken = util::time_ms() - t_start;

        if t_taken < period {
            std::thread::sleep_ms((period - t_taken) as u32);
        };
    }
}

fn main() {

    let args = Docopt::new(USAGE)
        .and_then(|d| d.decode::<CliArgs>())
        .unwrap_or_else(|e| e.exit());

    let seed = if args.flag_inputfile.is_empty() {
        None
    } else {
        util::read_file_to_string(&args.flag_inputfile)
            .and_then(|d: String| Ok(Some(parser::parse_plaintext(d))))
            .unwrap_or(None)
    };

    let mut world = World::new(args.flag_width, args.flag_height).seed(seed);

    match args.flag_render {
        0 => start_console_loop(&mut world, args.flag_period),
        _ => panic!("Unknown render engine")
    };
}
