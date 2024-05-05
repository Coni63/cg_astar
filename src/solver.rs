use std::collections::HashMap;

use crate::{board::Board, cell::State, solution::Solution};

pub struct Solver {
    pub all_options: HashMap<usize, Vec<State>>,
    pub base_solution: Vec<(usize, State)>,
}

impl Solver {
    pub fn new(board: &mut Board) -> Solver {
        let mut solver = Solver {
            all_options: HashMap::new(),
            base_solution: Vec::new(),
        };

        solver.get_deadend(board);
        solver.disable_corridor(board);
        solver.get_all_options(board);
        solver
    }

    pub fn get_base_solution(&self) -> Solution {
        Solution {
            arrows: self.base_solution.clone(),
            score: 0,
        }
    }

    pub fn update(&self, base_solution: &Solution) -> Solution {
        let solution = base_solution.clone();
        solution
    }

    fn get_deadend(&mut self, board: &mut Board) {
        for (idx, cell) in board.get_cells().iter().enumerate() {
            if !cell.modifiable {
                continue;
            }

            let mut count = 0;
            let mut free_direction: State = State::Empty;
            if board.get_cell_idx(cell.up).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::UpArrow;
            }

            if board.get_cell_idx(cell.down).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::DownArrow;
            }

            if board.get_cell_idx(cell.left).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::LeftArrow;
            }
            if board.get_cell_idx(cell.right).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::RightArrow;
            }

            if count == 3 {
                self.base_solution.push((idx, free_direction));
            }
        }

        for (idx, arrow) in self.base_solution.iter() {
            board.force_arrow(*idx, *arrow);
        }

        eprintln!("Deadend: {}", self.base_solution.len());
    }

    fn disable_corridor(&mut self, board: &mut Board) {
        let mut cells_to_modify = Vec::new();

        for (idx, cell) in board.get_cells().iter().enumerate() {
            if !cell.modifiable {
                continue;
            }

            if board.get_cell_idx(cell.up).state == State::Empty
                && board.get_cell_idx(cell.down).state == State::Empty
            {
                cells_to_modify.push(idx);
            }

            if board.get_cell_idx(cell.left).state == State::Empty
                && board.get_cell_idx(cell.right).state == State::Empty
            {
                cells_to_modify.push(idx);
            }
        }

        for idx in cells_to_modify.iter() {
            board.force_arrow(*idx, State::Free);
        }

        eprintln!("Corridor: {}", cells_to_modify.len());
    }

    fn get_all_options(&mut self, board: &Board) {
        for (idx, cell) in board.get_cells().iter().enumerate() {
            if !cell.modifiable {
                continue;
            }

            let mut dir: Vec<State> = Vec::new();

            if board.get_cell_idx(cell.right).state == State::Free {
                dir.push(State::RightArrow);
            }

            if board.get_cell_idx(cell.left).state == State::Free {
                dir.push(State::LeftArrow);
            }

            if board.get_cell_idx(cell.up).state == State::Free {
                dir.push(State::UpArrow);
            }

            if board.get_cell_idx(cell.down).state == State::Free {
                dir.push(State::DownArrow);
            }

            self.all_options.insert(idx, dir);
        }

        eprintln!("All options: {}", self.all_options.len());
        for (idx, dir) in self.all_options.iter() {
            eprintln!("{}: {:?}", idx, dir);
        }
    }
}
