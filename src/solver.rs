use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{board::Board, cell::State, solution::Solution};

pub struct Solver {
    pub all_options: HashMap<i32, Vec<State>>,
    pub array_keys_options: Vec<i32>,
    pub fixed_arrows: Vec<(usize, State)>,
    pub variant_arrows: Vec<(usize, State)>,
    rng: rand::rngs::ThreadRng,
}

impl Solver {
    pub fn new(board: &mut Board) -> Solver {
        let mut solver = Solver {
            all_options: HashMap::new(),
            array_keys_options: Vec::new(),
            fixed_arrows: Vec::new(),
            variant_arrows: Vec::new(),
            rng: rand::thread_rng(),
        };
        solver.get_deadend(board);
        solver.disable_corridor(board);
        solver.get_all_options(board);
        solver
    }

    pub fn get_base_solution(&mut self) -> Solution {
        let mut solution = Solution {
            fixed_arrows: self.fixed_arrows.clone(),
            variant_arrows: Vec::new(),
            score: 0,
        };

        for idx in self.array_keys_options.iter() {
            let dir = self.all_options.get(idx).unwrap();
            if dir.len() == 2 {
                let sols = Uniform::from(0..2);
                solution
                    .variant_arrows
                    .push((*idx as usize, dir[sols.sample(&mut self.rng)]));
            } else if self.rng.gen::<f64>() < 0.5 {
                let n = dir.len();
                let i = Uniform::from(0..n);
                solution
                    .variant_arrows
                    .push((*idx as usize, dir[i.sample(&mut self.rng)]));
            } else {
                solution.variant_arrows.push((*idx as usize, State::Free));
            }
        }

        solution
    }

    pub fn update(&self, solution: &mut Solution) {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..self.array_keys_options.len());

        let idx = self.array_keys_options[n];
        let dir = self.all_options.get(&idx).unwrap();
        if dir.len() == 2 {
            let sols = Uniform::from(0..2);
            solution.variant_arrows[n] = (idx as usize, dir[sols.sample(&mut rng)]);
        } else if rng.gen::<f64>() < 0.5 {
            let n = dir.len();
            let i = Uniform::from(0..n);
            solution.variant_arrows[n] = (idx as usize, dir[i.sample(&mut rng)]);
        } else {
            solution.variant_arrows[n] = (idx as usize, State::Free);
        }
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
                self.fixed_arrows.push((idx, free_direction));
            }
        }

        for (idx, arrow) in self.fixed_arrows.iter() {
            board.force_arrow(*idx, *arrow);
        }

        eprintln!("Deadend: {}", self.fixed_arrows.len());
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

            self.array_keys_options.push(idx as i32);
            self.all_options.insert(idx as i32, dir);
        }

        // eprintln!("All options: {}", self.all_options.len());
        // for (idx, dir) in self.all_options.iter() {
        //     eprintln!("{}: {:?}", idx, dir);
        // }
    }
}
