use crate::cell::{Cell, State};
use crate::solution::Solution;

pub struct Board {
    cells: [Cell; 190],
    width: usize,
    height: usize,
    modifiables: Vec<usize>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::default(); 190],
            width: 19,
            height: 10,
            modifiables: Vec::new(),
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

    pub fn post_init(&mut self) {
        for (idx, cell) in self.cells.iter().enumerate() {
            if cell.modifiable {
                self.modifiables.push(idx);
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        let idx = y * self.width + x;
        self.get_cell_idx(idx)
    }

    pub fn get_cell_idx(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: State) {
        let idx = y * self.width + x;
        self.set_cell_idx(idx, state)
    }

    pub fn set_cell_idx(&mut self, idx: usize, state: State) {
        self.cells[idx].state = state;
    }

    pub fn apply_solution(&mut self, solution: &Solution) {
        for (idx, state) in solution.arrows.iter() {
            self.cells[*idx].state = *state;
        }
    }

    pub fn remove_solution(&mut self, solution: &Solution) {
        for (idx, _) in solution.arrows.iter() {
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
