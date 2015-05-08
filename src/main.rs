extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate itertools;
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
    -h <height>, --height <height>              World height, points                 [default: 25]
    -w <width>, --width <width>                 World width, points                  [default: 80]
    -p <period>, --period <period>              World generational change period, ms [default: 350]
    -i <inputfile>, --inputfile <inputfile>     populate world from file
";

#[derive(RustcDecodable, Debug)]
struct CliArgs {
    flag_width: isize,
    flag_height: isize,
    flag_period: u32,
    flag_inputfile: String,
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

    print!("\x1b[2J");

    loop {
        println!("\x1b[H{}", world.evolve());
        std::thread::sleep_ms(args.flag_period);
    }
}
