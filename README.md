# Golr [![Build Status](https://travis-ci.org/pillserg/golr.svg)](https://travis-ci.org/pillserg/golr.svg)
Conway's Game of Life in Rust

# Getting started
```
$ git clone https://github.com/pillserg/golr.git
$ cd golr
$ cargo run
```

# Usage
```
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
```
You can pass params to ```golr``` using ```cargo run``` command
```
cargo run -- -w 100 -h 30 -p 66 -r 1 --cell-size 10 -i src/gardens/gosper.txt
```

# Contribute
Open an issue. Disscuss. Implement. Send PR.

# Misc
 - [multirust][multirust] is great https://github.com/brson/multirust
 - [racer][racer] works - and thus is also great

[multirust]: https://github.com/brson/multirust
[racer]: https://github.com/phildawes/racer
