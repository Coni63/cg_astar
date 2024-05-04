#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Free,
}
