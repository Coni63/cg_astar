use std::collections::HashMap;

use crate::{board::Board, cell::State, solution::Solution};

pub struct Solver {
    pub all_options: HashMap<usize, Vec<State>>,
}

impl Solver {
    pub fn new(board: &Board) -> Solver {
        let mut solver = Solver {
            all_options: HashMap::new(),
        };

        solver.get_all_options(board);
        solver
    }

    pub fn update(&self, base_solution: &Solution) -> Solution {
        let solution = base_solution.clone();
        solution
    }

    fn get_all_options(&mut self, board: &Board) {
        for (idx, cell) in board.get_cells(true).iter().enumerate() {
            if board.get_cell_idx(cell.right).state == State::Free {
                self.all_options
                    .entry(idx)
                    .or_default()
                    .push(State::RightArrow);
            }

            if board.get_cell_idx(cell.left).state == State::Free {
                self.all_options
                    .entry(idx)
                    .or_default()
                    .push(State::LeftArrow);
            }

            if board.get_cell_idx(cell.up).state == State::Free {
                self.all_options
                    .entry(idx)
                    .or_default()
                    .push(State::UpArrow);
            }

            if board.get_cell_idx(cell.down).state == State::Free {
                self.all_options
                    .entry(idx)
                    .or_default()
                    .push(State::DownArrow);
            }
        }
    }
}
