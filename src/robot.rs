#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Robot {
    pub id: i8,
    pub idx: usize,
    pub initial_idx: usize,
    pub direction: Direction,
    pub initial_direction: Direction,
    pub alive: bool,
    pub visited: [bool; 800],
}

impl Robot {
    pub fn new(id: i8, idx: usize, direction: Direction) -> Robot {
        Robot {
            id,
            idx,
            direction: direction.clone(),
            initial_idx: idx,
            initial_direction: direction.clone(),
            alive: true,
            visited: [false; 800],
        }
    }

    pub fn reset(&mut self) {
        self.idx = self.initial_idx;
        self.direction = self.initial_direction.clone();
        self.alive = true;
        self.visited = [false; 800];
    }

    pub fn set_visited(&mut self) {
        let offset = match self.direction {
            Direction::Up => 0,
            Direction::Down => 200,
            Direction::Left => 400,
            Direction::Right => 600,
        };
        self.visited[self.idx + offset] = true;
    }

    pub fn visited(&self) -> bool {
        let offset = match self.direction {
            Direction::Up => 0,
            Direction::Down => 200,
            Direction::Left => 400,
            Direction::Right => 600,
        };
        self.visited[self.idx + offset]
    }
}
