extern crate rand;
use rand::{Rand, Rng};
use std::collections::HashMap;
use std::thread::sleep_ms;


#[derive(Debug, PartialEq, Clone)]
enum CellState{
    Alive,
    Dead
}


type WSize = i8;
type Point = (WSize, WSize);


impl Rand for CellState {
    #[inline]
    fn rand<R:Rng>(rng: &mut R) -> CellState {
        let x: bool = rand::random();
        match x {
            true => CellState::Alive,
            false => CellState::Dead
        }
    }
}
#[derive(Debug, Clone)]
struct World {
    width: WSize,
    height: WSize,
    cells: HashMap<Point, CellState>
}


impl World {
    fn new(width: WSize, height: WSize)  -> World {
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

    fn decide_fate(&self, x: WSize, y: WSize) -> CellState {
        rand::random()
    }

    fn is_alive(&self, x: WSize, y: WSize) -> bool {
        let key = (x, y);
        self.cells.get(&key) == Some(&CellState::Alive)
    }

    fn set_state(&mut self, x: WSize, y: WSize, value: CellState) {
        self.cells.insert((x, y), value);
    }

    fn randomize(&mut self) {
        for y in 0..self.height { for x in 0..self.width { 
            self.set_state(x, y, rand::random());    
        }}
        
    }
}


fn clear_screen() {
    print!("\x1b[2J\n");
}

fn draw(ref world: &World) {
    clear_screen();
    for y in 0..world.height {
        print!("\n");
        for x in 0..world.width {
            if world.is_alive(x,y) {
                print!("□");
            } else {
                print!(" ");
            }
                
        }
    }
    print!("\n");
}


fn main() {
    println!("Hello");
    let mut world:World = World::new(30, 20);
    world.randomize();
    loop {
        world = world.evolve();
        draw(&world);
        std::thread::sleep_ms(50);
    }
}

