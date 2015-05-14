use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use rand::random;

pub type Point = (isize, isize);

pub struct World {
    width: isize,
    height: isize,
    generation: HashSet<Point>,
    neighbours: HashMap<Point, usize>,
    age: usize,
}


impl World {
    pub fn new(width: isize, height: isize)  -> World {
        World {
            width: width,
            height: height,
            age: 0,
            generation: HashSet::with_capacity((width * height) as usize),
            neighbours: HashMap::new()
        }
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

    pub fn size(&self) -> (isize, isize) {(self.width, self.height)}
    pub fn age(&self) -> usize {self.age}
    pub fn generation(&self) -> &HashSet<Point> {&self.generation}

    fn calculate_neighbours(&mut self) {
        self.neighbours.clear();
        for &(x, y) in self.generation.iter() {
            (-1..2).cartesian_product(-1..2)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(|(dx, dy)| (dx + x, dy + y))
            .fold(&mut self.neighbours, |acc, point| {
                *acc.entry(point).or_insert(0) += 1;
                acc
            });
        };
    }

    fn decide_fate(&self, point: &Point, num_neighbours: usize) -> bool {
        match self.generation.contains(point) {
            true => num_neighbours == 3 || num_neighbours == 2,
            false => num_neighbours == 3
        }
    }
}
