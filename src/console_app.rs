use std;

use util;
use world::World;


pub fn start_console_app(world: &mut World, period: u64) {
    print!("\x1b[2J");
    loop {
        let t_start = util::time_ms();
        println!("\x1b[H{}", world.evolve());
        let t_taken = util::time_ms() - t_start;

        if t_taken < period {
            std::thread::sleep_ms((period - t_taken) as u32);
        };
    }
}
