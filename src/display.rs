fn clear_screen() {
    print!("\x1b[2J\n");
}

pub trait Drawable {
    fn should_draw(&self, x: u8, y: u8) -> bool;
    fn get_height(&self) -> u8;
    fn get_width(&self) -> u8;
}

pub fn draw<T: Drawable>(ref world: &T) {
    clear_screen();
    for y in 0..world.get_height() {
        print!("\n");
        for x in 0..world.get_width() {
            if world.should_draw(x,y) {
                print!("□");
            } else {
                print!(" ");
            }
                
        }
    }
    print!("\n");
}
