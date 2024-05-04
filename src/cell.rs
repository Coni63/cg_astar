#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Free,
}
