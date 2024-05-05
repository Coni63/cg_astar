use crate::cell::State;

#[derive(Debug, Default)]
pub struct Solution {
    pub fixed_arrows: Vec<(usize, State)>,
    pub variant_arrows: Vec<(usize, State)>,
    pub score: i32,
}

impl ToString for Solution {
    fn to_string(&self) -> String {
        let mut v: Vec<String> = Vec::new();
        for (idx, state) in self.fixed_arrows.iter().chain(self.variant_arrows.iter()) {
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

impl Clone for Solution {
    fn clone(&self) -> Solution {
        Solution {
            variant_arrows: self.variant_arrows.clone(),
            fixed_arrows: self.fixed_arrows.clone(),
            score: self.score,
        }
    }
}
