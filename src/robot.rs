use crate::cell::Cell;

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub direction: Cell,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Cell) -> Robot {
        Robot { x, y, direction }
    }
}

impl Clone for Robot {
    fn clone(&self) -> Robot {
        Robot {
            x: self.x,
            y: self.y,
            direction: self.direction,
        }
    }
}
