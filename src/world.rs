extern crate rand;

use std::collections::HashMap;
use std::fmt;

type Point = (isize, isize);


#[derive(Debug, PartialEq, Clone)]
enum CellState{
    Alive,
    Dead
}


impl rand::Rand for CellState {
    #[inline]
    fn rand<R:rand::Rng>(_: &mut R) -> CellState {
        match rand::random() {
            true => CellState::Alive,
            false => CellState::Dead
        }
    }
}


#[derive(Debug, Clone)]
pub struct World {
    width: isize,
    height: isize,
    cells: HashMap<Point, CellState>
}


impl World {
    pub fn new(width: isize, height: isize)  -> World {
        let mut world = World{width: width, height: height, cells: HashMap::new()};
        for y in 0..height { for x in 0..width {
            world.cells.insert((x, y), CellState::Dead);
        }}
        world
    }

    pub fn evolve(&self) -> World {
        let mut new_world = self.clone();
        for y in 0..self.height { for x in 0..self.width {
            new_world.set_state(x, y, self.decide_fate(x, y))
        }}
        new_world

    }

    pub fn randomize(&mut self) {
        for y in 0..self.height { for x in 0..self.width {
            self.set_state(x, y, rand::random());
        }}

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
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for x in 0..self.width + 2 {
            buf.push(if x == 0 { '┌' } else if x == self.width + 1 { '┐' } else { '─' });
        }
        buf.push('\n');
        for y in 0..self.height {
            buf.push('│');
            for x in 0..self.width {
                buf.push(if self.is_alive(x, y) { '*' } else { ' ' });
            }
            buf.push_str("│\n");
        }
        for x in 0..self.width + 2 {
            buf.push(if x == 0 { '└' } else if x == self.width + 1 { '┘' } else { '─' });
        }
        write!(f, "{}", buf)
    }
}
