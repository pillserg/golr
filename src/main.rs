enum CellState{
    Alive,
    Dead
}


struct Cell {
    x: i8,
    y: i8,
    state: CellState
}


impl Cell {
    fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            CellState::Dead => false
        }
    }


}

fn print_cell_data (cell: &Cell) {
    println!(
        "x: {x}, y: {y}, state: {state}", 
        x=cell.x,
        y=cell.y,
        state = match cell.state {
            CellState::Dead => "dead",
            CellState::Alive => "alive"
        }
    );
}

fn main() {
    println!("Hello");
    let height = 10;
    let width = height;

    for y in 0..height {
        for x in 0..width {
            print_cell_data(&Cell{x: x, y: y, state: CellState::Alive});
        }
    }
    let cell = Cell{x: 0, y: 0, state: CellState::Alive};
    match cell {
        Cell {state: CellState::Alive, ..} => println!("cell is alive"),
        Cell {state: CellState::Dead, ..} => println!("cell is dead")
    }  

    if cell.is_alive() {
        println!("We determined that cell is alive through 'is_alive' method");
    }
}

