use crate::{board::Board, cell::State};

#[derive(Debug, Default)]
pub struct Solution {
    pub arrows: Vec<(usize, State)>,
    pub score: i32,
}

impl Solution {
    pub fn from_board(board: &Board) -> Solution {
        Solution {
            arrows: board.get_forced_solution(),
            score: 0,
        }
    }
}

impl ToString for Solution {
    fn to_string(&self) -> String {
        let mut v: Vec<String> = Vec::new();
        for (idx, state) in &self.arrows {
            let row = idx / 19;
            let col = idx % 19;
            let letter = match state {
                State::UpArrow => "U",
                State::DownArrow => "D",
                State::LeftArrow => "L",
                _ => "R",
            };
            v.push(format!("{} {} {}", col, row, letter));
        }
        v.join(" ")
    }
}
