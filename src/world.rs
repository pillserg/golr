use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use rand::random;

pub type Point = (isize, isize);

#[derive(Debug, Clone)]
pub struct World {
    width: isize,
    height: isize,
    generation: HashSet<Point>,
    neighbours: HashMap<Point, usize>,
    age: usize,
}

impl World {
    pub fn new(width: isize, height: isize)  -> World {
        World { width: width, height: height, age: 0,
                generation: HashSet::with_capacity((width * height) as usize),
                neighbours: HashMap::with_capacity((width * height * 8) as usize) }
    }

    pub fn seed(mut self) -> World {
        self.generation = (0..self.width).cartesian_product(0..self.height)
            .filter(|_| random())
            .collect::<HashSet<Point>>();
        self.calculate_neighbours();
        self
    }

    pub fn evolve(&mut self) -> &World {
        self.generation = self.neighbours.iter()
            .filter(|&(n, c)| if self.generation.contains(n) { *c == 3 || *c == 2 } else { *c == 3 })
            .map(|(n, _)| *n)
            .collect::<HashSet<Point>>();
        self.calculate_neighbours();
        self.age += 1;
        self
    }

    fn calculate_neighbours(&mut self) {
        self.neighbours.clear();
        for &(x, y) in self.generation.iter() {
            let neighbours = (-1..2).cartesian_product(-1..2)
                                    .filter(|&(dx, dy)| dx != 0 || dy != 0)
                                    .map(|(dx, dy)| (dx + x, dy + y));
            for n in neighbours {
                let counts = self.neighbours.entry(n).or_insert(0);
                *counts = *counts + 1;
            }
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::with_capacity(((self.width + 2) * (self.height + 2) + 50) as usize);
        for x in 0..self.width + 2 {
            buf.push(if x == 0 { '┌' } else if x == self.width + 1 { '┐' } else { '─' });
        }
        buf.push('\n');
        for y in 0..self.height {
            buf.push('│');
            for x in 0..self.width {
                buf.push(if self.generation.contains(&(x, y)) { '*' } else { ' ' });
            }
            buf.push('|');
            if y == 0 {
                buf.push_str(&format!(" cells: {}", self.generation.len()));
            }
            if y == 1 {
                buf.push_str(&format!(" age: {}", self.age));
            }
            buf.push('\n');
        }
        for x in 0..self.width + 2 {
            buf.push(if x == 0 { '└' } else if x == self.width + 1 { '┘' } else { '─' });
        }
        write!(f, "{}", buf)
    }
}
