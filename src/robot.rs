#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Robot {
    pub idx: usize,
    pub initial_idx: usize,
    pub direction: Direction,
    pub initial_direction: Direction,
    pub alive: bool,
}

impl Robot {
    pub fn new(idx: usize, direction: Direction) -> Robot {
        Robot {
            idx,
            direction: direction.clone(),
            initial_idx: idx,
            initial_direction: direction.clone(),
            alive: true,
        }
    }

    pub fn reset(&mut self) {
        self.idx = self.initial_idx;
        self.direction = self.initial_direction.clone();
        self.alive = true;
    }
}

impl Clone for Robot {
    fn clone(&self) -> Robot {
        Robot {
            idx: self.idx,
            initial_idx: self.initial_idx,
            direction: self.direction.clone(),
            initial_direction: self.initial_direction.clone(),
            alive: self.alive,
        }
    }
}
