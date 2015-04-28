extern crate rand;
use rand::{Rand, Rng};
use std::collections::HashMap;
use std::thread::sleep_ms;
mod display;


type Point = (u8, u8);


#[derive(Debug, PartialEq, Clone)]
enum CellState{
    Alive,
    Dead
}


impl Rand for CellState {
    #[inline]
    fn rand<R:Rng>(rng: &mut R) -> CellState {
        match rand::random() {
            true => CellState::Alive,
            false => CellState::Dead
        }
    }
}

trait Measurable {

}

#[derive(Debug, Clone)]
struct World {
    width: u8,
    height: u8,
    cells: HashMap<Point, CellState>
}


impl World {
    fn new(width: u8, height: u8)  -> World {
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

    fn decide_fate(&self, x: u8, y: u8) -> CellState {
        rand::random()
    }

    fn is_alive(&self, x: u8, y: u8) -> bool {
        let key = (x, y);
        self.cells.get(&key) == Some(&CellState::Alive)
    }

    fn set_state(&mut self, x: u8, y: u8, value: CellState) {
        self.cells.insert((x, y), value);
    }

    fn randomize(&mut self) {
        for y in 0..self.height { for x in 0..self.width { 
            self.set_state(x, y, rand::random());    
        }}
        
    }
}

impl display::Drawable for World {
    fn should_draw(&self, x: u8, y: u8) -> bool {
        self.is_alive(x, y)
    }
    fn get_height(&self) -> u8 {
        self.height
    }
    fn get_width(&self) -> u8 {
        self.width
    }
}


fn main() {
    println!("Hello");
    let mut world:World = World::new(30, 20);
    world.randomize();
    loop {
        world = world.evolve();
        display::draw(&world);
        std::thread::sleep_ms(50);
    }
}

