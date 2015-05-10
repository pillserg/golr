use std::{fmt, thread};
use util;
use world::World;


pub fn start_console_app(world: &mut World, period: u64) {
    print!("\x1b[2J");
    loop {
        let t_start = util::time_ms();
        println!("\x1b[H{}", world.evolve());
        let t_taken = util::time_ms() - t_start;

        if t_taken < period {
            thread::sleep_ms((period - t_taken) as u32);
        };
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (width, height) = self.size();
        let generation = self.generation();
        let age = self.age();
        let mut buf = String::with_capacity(((width + 2) * (height + 2) + 50) as usize);
        for x in 0..width + 2 {
            buf.push(if x == 0 { '┌' } else if x == width + 1 { '┐' } else { '─' });
        }
        buf.push('\n');
        for y in 0..height {
            buf.push('│');
            for x in 0..width {
                buf.push(if generation.contains(&(x, y)) { '*' } else { ' ' });
            }
            buf.push('|');
            if y == 0 {
                buf.push_str(&format!(" cells: {}", generation.len()));
            }
            if y == 1 {
                buf.push_str(&format!(" age: {}", age));
            }
            buf.push('\n');
        }
        for x in 0..width + 2 {
            buf.push(if x == 0 { '└' } else if x == width + 1 { '┘' } else { '─' });
        }
        f.write_str(&buf)
    }
}
