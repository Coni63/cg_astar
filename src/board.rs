use crate::cell::{Cell, State};
use crate::solution::Solution;

pub struct Board {
    cells: [Cell; 190],
    width: usize,
    height: usize,
    modifiables: Vec<usize>,
    forced_solution: Vec<(usize, State)>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::default(); 190],
            width: 19,
            height: 10,
            modifiables: Vec::new(),
            forced_solution: Vec::new(),
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

    pub fn get_cells(&self, only_modifiable: bool) -> Vec<&Cell> {
        if only_modifiable {
            let mut ans: Vec<&Cell> = Vec::new();
            for idx in self.modifiables.iter() {
                ans.push(&self.cells[*idx]);
            }
            ans
        } else {
            self.cells.iter().collect()
        }
    }

    pub fn post_init(&mut self) {
        self.set_list_modifiable();
        eprintln!("{} modifiable cells at start", self.modifiables.len());
        self.get_deadend();
        self.set_list_modifiable();
        eprintln!(
            "{} modifiable cells without deadend",
            self.modifiables.len()
        );
        self.disable_corridor();
        self.set_list_modifiable();
        eprintln!(
            "{} modifiable cells without corridors",
            self.modifiables.len()
        );
    }

    fn get_deadend(&mut self) {
        for idx in self.modifiables.iter() {
            let cell = self.get_cell_idx(*idx);

            let mut count = 0;
            let mut free_direction: State = State::Empty;
            if self.get_cell_idx(cell.up).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::UpArrow;
            }

            if self.get_cell_idx(cell.down).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::DownArrow;
            }

            if self.get_cell_idx(cell.left).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::LeftArrow;
            }
            if self.get_cell_idx(cell.right).state == State::Empty {
                count += 1;
            } else {
                free_direction = State::RightArrow;
            }

            if count == 3 {
                self.forced_solution.push((*idx, free_direction));
                self.cells[*idx].modifiable = false;
            }
        }
    }

    pub fn get_forced_solution(&self) -> Vec<(usize, State)> {
        self.forced_solution.clone()
    }

    fn disable_corridor(&mut self) {
        let mut cells_to_modify = Vec::new();

        for idx in self.modifiables.iter() {
            let cell = self.get_cell_idx(*idx);

            if self.get_cell_idx(cell.up).state == State::Empty
                && self.get_cell_idx(cell.down).state == State::Empty
            {
                cells_to_modify.push(*idx);
            }

            if self.get_cell_idx(cell.left).state == State::Empty
                && self.get_cell_idx(cell.right).state == State::Empty
            {
                cells_to_modify.push(*idx);
            }
        }

        for idx in cells_to_modify.iter() {
            self.cells[*idx].modifiable = false;
        }
    }

    fn set_list_modifiable(&mut self) {
        self.modifiables.clear();
        for (idx, cell) in self.cells.iter().enumerate() {
            if cell.modifiable {
                self.modifiables.push(idx);
            }
        }
    }

    pub fn get_cell_idx(&self, idx: usize) -> &Cell {
        &self.cells[idx]
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
