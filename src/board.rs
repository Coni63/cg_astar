use crate::cell::{Cell, State};
use crate::solution::Solution;

pub struct Board {
    cells: [Cell; 190],
    width: usize,
    height: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::default(); 190],
            width: 19,
            height: 10,
        }
    }

    pub fn setup(&mut self, x: usize, y: usize, state: State) {
        let idx = y * self.width + x;

        let top_row = if y == 0 { 9 } else { y - 1 };
        let bottom_row = if y == 9 { 0 } else { y + 1 };
        let left_col = if x == 0 { 18 } else { x - 1 };
        let right_col = if x == 18 { 0 } else { x + 1 };

        self.cells[idx] = Cell {
            x: x as u8,
            y: y as u8,
            state,
            modifiable: state == State::Free,
            up: top_row * self.width + x,
            down: bottom_row * self.width + x,
            left: y * self.width + left_col,
            right: y * self.width + right_col,
        };
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn get_cell_idx(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn force_arrow(&mut self, idx: usize, state: State) {
        self.cells[idx].state = state;
        self.cells[idx].modifiable = false;
    }

    pub fn apply_solution(&mut self, solution: &Solution) {
        for (idx, state) in solution.variant_arrows.iter() {
            self.cells[*idx].state = *state;
        }
    }

    pub fn remove_solution(&mut self, solution: &Solution) {
        for (idx, _) in solution.variant_arrows.iter() {
            self.cells[*idx].state = State::Free;
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut board = Board::new();
        board.cells = self.cells;
        board.width = self.width;
        board.height = self.height;
        board
    }
}
