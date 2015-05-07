extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate itertools;
extern crate unicode_segmentation;

mod display;
mod world;
mod parser;

use std::thread::sleep_ms;
use docopt::Docopt;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;


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

    let args = Docopt::new(USAGE).and_then(|d| d.decode::<CliArgs>())
                                 .unwrap_or_else(|e| e.exit());

    let mut seed = None;
    if !args.flag_inputfile.is_empty() {
        let path = Path::new(&args.flag_inputfile);
        let mut data = String::new();

        let mut file = File::open(&path)
            .unwrap_or_else(|_| {println!("Can't open {}", path.display()); ::std::process::exit(1)});
        file.read_to_string(&mut data)
            .unwrap_or_else(|_| {println!("Can't read {}", path.display()); ::std::process::exit(1)});
        seed = Some(parser::parse_plaintext(data)
            .unwrap_or_else(|_| {println!("Can't parse {}", path.display()); ::std::process::exit(1)}));
    };

    let mut world = World::new(args.flag_width, args.flag_height).seed(seed);

    print!("\x1b[2J");

    loop {
        println!("\x1b[H{}", world.evolve());
        std::thread::sleep_ms(args.flag_period);
    }
}
