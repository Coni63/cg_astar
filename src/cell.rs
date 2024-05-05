#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Empty,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Free,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub x: u8,
    pub y: u8,
    pub state: State,
    pub modifiable: bool,

    pub up: usize,
    pub down: usize,
    pub left: usize,
    pub right: usize,
}

impl Cell {}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            state: State::Empty,
            x: 0,
            y: 0,
            modifiable: false,
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }
}
