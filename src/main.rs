extern crate rand;
extern crate argparse;

use std::collections::HashMap;
use std::thread::sleep_ms;
use std::process::exit;
use std::env;

use rand::{Rand, Rng};
use argparse::{ArgumentParser, StoreTrue, Store};

mod display;


type Point = (isize, isize);


#[derive(Debug, PartialEq, Clone)]
enum CellState{
    Alive,
    Dead
}


impl Rand for CellState {
    #[inline]
    fn rand<R:Rng>(_: &mut R) -> CellState {
        match rand::random() {
            true => CellState::Alive,
            false => CellState::Dead
        }
    }
}


#[derive(Debug, Clone)]
struct World {
    width: isize,
    height: isize,
    cells: HashMap<Point, CellState>
}



    // def _get_siblings(self, x, y):
    //     return [
    //         (_x, _y) for _x in [x + 1, x - 1, x] for _y in [y + 1, y - 1, y] if (_x != x or _y != y) and 0 <= _x < self.width and 0 <= _y < self.height
    //     ]

    // def _decide_fate(self, cell, cell_num, row_num):
        // num_alive_siblings = len(filter(bool, [self.cells[y][x] for (x, y) in self._get_siblings(cell_num, row_num)]))
        // if (cell is not ALIVE and num_alive_siblings is 3) or (cell is ALIVE and 2 <= num_alive_siblings <= 3):
        //     return ALIVE
        // else:
        //     return DEAD


impl World {
    fn new(width: isize, height: isize)  -> World {
        let mut world = World{width: width, height: height, cells: HashMap::new()};
        for y in 0..height { for x in 0..width {
            world.cells.insert((x, y), CellState::Dead);
        }}
        world
    }
    fn evolve(&self) -> World {
        let mut new_world = self.clone();
        for y in 0..self.height { for x in 0..self.width {
            new_world.set_state(x, y, self.decide_fate(x, y))
        }}
        new_world
        
    }

    fn get_sibling_coords(&self, x: isize, y: isize) -> Vec<Point> {
        let mut coords = vec![];
        for _x in [x + 1, x - 1, x].iter() {for _y in [y + 1, y - 1, y].iter() {
            if (*_x != x || *_y != y) && 0 <= *_x && *_x < self.width && 0 <= *_y && *_y < self.height {
                coords.push((*_x, *_y));
            }
        }}
        coords
    }

    fn decide_fate(&self, x: isize, y: isize) -> CellState {
        
        let mut num_alive_siblings = 0;

        for (_x, _y) in self.get_sibling_coords(x, y) {
            if self.is_alive(_x, _y) {
                num_alive_siblings += 1
            }
        }

        if (!self.is_alive(x, y) && num_alive_siblings == 3) ||  (self.is_alive(x, y) && 2 <= num_alive_siblings && num_alive_siblings <= 3) {
            CellState::Alive
        }
        else {
            CellState::Dead
        }
    }

    fn is_alive(&self, x: isize, y: isize) -> bool {
        let key = (x, y);
        self.cells.get(&key) == Some(&CellState::Alive)
    }

    fn set_state(&mut self, x: isize, y: isize, value: CellState) {
        self.cells.insert((x, y), value);
    }

    fn randomize(&mut self) {
        for y in 0..self.height { for x in 0..self.width { 
            self.set_state(x, y, rand::random());    
        }}
        
    }
}

impl display::Drawable for World {
    fn should_draw(&self, x: isize, y: isize) -> bool {
        self.is_alive(x, y)
    }
    fn get_height(&self) -> isize {
        self.height
    }
    fn get_width(&self) -> isize {
        self.width
    }
}


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


    let mut world:World = World::new(width, height);
    world.randomize();
    display::clear_screen();
    loop {
        world = world.evolve();
        display::draw(&world);
        std::thread::sleep_ms(fps);
    }
}

