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
                neighbours: HashMap::new(), }
    }

    pub fn seed(mut self, maybe_seed: Option<HashSet<Point>>) -> World {
        match maybe_seed {
            Some(seed) => self.generation = seed.clone(),
            None => {
                (0..self.width).cartesian_product(0..self.height)
                    .filter(|_| random())
                    .fold(&mut self.generation, |acc, point| {
                        acc.insert(point);
                        acc
                    });
            }
        };
        self.calculate_neighbours();
        self
    }

    pub fn evolve(&mut self) -> &World {
        let mut next_generation = HashSet::with_capacity(self.generation.len() * 2);
        self.neighbours.iter()
            .filter(|&(n, c)| self.decide_fate(n, *c))
            .map(|(n, _)| *n)
            .fold(&mut next_generation, |acc, point| {
                acc.insert(point);
                acc
            });
        self.generation = next_generation;
        self.calculate_neighbours();
        self.age += 1;
        self
    }

    fn calculate_neighbours(&mut self) {
        self.neighbours.clear();
        for &(x, y) in self.generation.iter() {
            (-1..2)
            .cartesian_product(-1..2)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(|(dx, dy)| (dx + x, dy + y))
            .fold(&mut self.neighbours, |acc, point| {
                *acc.entry(point).or_insert(0) += 1;
                acc
            });
        };
    }

    fn decide_fate(&self, p: &Point, c: usize) -> bool {
        if self.generation.contains(p) {
            c == 3 || c == 2
        } else {
            c == 3
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
        f.write_str(&buf)
    }
}
