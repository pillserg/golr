use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

    pub fn seed(mut self, seed: Option<HashSet<Point>>) -> World {
        self.generation = seed.unwrap_or(
            (0..self.width).cartesian_product(0..self.height)
            .filter(|_| random())
            .collect::<HashSet<Point>>()
        );
        self.calculate_neighbours();
        self
    }

    pub fn size(&self) -> (isize, isize) {
        (self.width, self.height)
    }

    pub fn age(&self) -> usize {
        self.age
    }

    pub fn generation(&self) -> &HashSet<Point> {
        &self.generation
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
