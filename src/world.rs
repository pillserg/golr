use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use rand::random;

pub type Point = (isize, isize);

#[derive(Debug, Clone)]
pub struct World {
    width: isize,
    height: isize,
    generation: Vec<Point>,
    age: usize,
}

impl World {
    pub fn new(width: isize, height: isize)  -> World {
        World { width: width, height: height, age: 0, generation: vec![] }
    }

    pub fn seed(mut self) -> World {
        self.generation = (0..self.width).cartesian_product(0..self.height)
            .filter(|_| random())
            .collect::<Vec<Point>>();
        self
    }

    pub fn evolve(&mut self) -> &World {
        let mut neighbours_counts = HashMap::new();
        for &(x, y) in &self.generation[..] {
            let neighbours = (-1..2).cartesian_product(-1..2)
                                    .filter(|&(dx, dy)| dx != 0 || dy != 0)
                                    .map(|(dx, dy)| (dx + x, dy + y));
            for n in neighbours {
                let counts = neighbours_counts.entry(n).or_insert(0);
                *counts = *counts + 1;
            }
        }
        self.generation = neighbours_counts.iter()
            .filter(|&(n, c)| if self.generation.contains(n) { *c == 3 || *c == 2 } else { *c == 3 })
            .map(|(n, _)| *n)
            .collect::<Vec<Point>>();
        self.age += 1;
        self
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
