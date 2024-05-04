use crate::cell::Cell;
use crate::robot::Robot;

pub struct Board {
    pub cells: [Cell; 190],
    pub width: usize,
    pub height: usize,

    pub robots: Vec<Robot>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::Empty; 190],
            width: 19,
            height: 10,
            robots: vec![],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell;
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }
}
