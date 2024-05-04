use crate::cell::Cell;
use crate::robot::Robot;
use crate::solution::Solution;

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

    pub fn apply_solution(&mut self, solution: &Solution) {
        for (idx, cell) in solution.arrows.iter() {
            self.cells[*idx as usize] = *cell;
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut board = Board::new();
        board.cells = self.cells;
        board.width = self.width;
        board.height = self.height;
        board.robots = self.robots.to_vec();
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let mut board = Board::new();
        assert_eq!(board.width, 19);
        assert_eq!(board.height, 10);

        board.set_cell(0, 0, Cell::UpArrow);
        board.add_robot(Robot::new(0, 0, Cell::UpArrow));

        let mut copy = board.clone();

        let solution = Solution {
            arrows: vec![(2, Cell::DownArrow)],
            score: 0,
        };
        copy.apply_solution(&solution);
        copy.robots[0].x = 2; // change the robot that is present in both board and copy
        copy.add_robot(Robot::new(1, 0, Cell::UpArrow)); // add a new robot to copy

        assert_eq!(board.get_cell(0, 0), &Cell::UpArrow);
        assert_eq!(board.get_cell(2, 0), &Cell::Empty);
        assert_eq!(board.robots.len(), 1);
        assert_eq!(board.robots[0].x, 0); // robot in board should not be changed

        assert_eq!(copy.get_cell(0, 0), &Cell::UpArrow);
        assert_eq!(copy.get_cell(2, 0), &Cell::DownArrow);
        assert_eq!(copy.robots.len(), 2);
        assert_eq!(copy.robots[0].x, 2); // robot in copy should be changed
    }
}
