extern crate rand;
extern crate time;
extern crate docopt;
extern crate itertools;
extern crate rustc_serialize;
// piston stuff
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod piston_app;
mod console_app;
mod world;
mod parser;
mod util;

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
    --cell-size <cell_size>                     Choose cell size (Only for visual renderers) [default: 2]
    --gl-version <gl_version>                   OpenGl version [32, 30]                      [default: 32]
";

#[derive(RustcDecodable, Debug)]
struct CliArgs {
    flag_width: isize,
    flag_height: isize,
    flag_period: u64,
    flag_inputfile: Option<String>,
    flag_render: isize,
    flag_cell_size: u32,
    flag_gl_version: u32,
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.decode::<CliArgs>())
        .unwrap_or_else(|e| e.exit());

    let seed = args.flag_inputfile.as_ref()
                   .and_then(|fif| util::read_file_to_string(fif)
                                  .and_then(|d: String| Ok(Some(parser::parse_plaintext(d))))
                                  .unwrap_or(None));

    let mut world = World::new(args.flag_width, args.flag_height).seed(seed);

    match args.flag_render {
        0 => console_app::start_console_app(&mut world, args.flag_period),
        1 => piston_app::start_piston_app(world, args.flag_period, args.flag_cell_size, args.flag_gl_version),
        _ => panic!("Unknown render engine")
    };
}
