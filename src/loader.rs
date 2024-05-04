use std::io;

use crate::{board::Board, cell::Cell, robot};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn load() -> Board {
    let mut board = Board::new();

    for y in 0..10 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();

        for (x, c) in line.chars().enumerate() {
            match c {
                'U' => board.set_cell(x, y, Cell::UpArrow),
                'D' => board.set_cell(x, y, Cell::DownArrow),
                'L' => board.set_cell(x, y, Cell::LeftArrow),
                'R' => board.set_cell(x, y, Cell::RightArrow),
                '.' => board.set_cell(x, y, Cell::Free),
                '#' => board.set_cell(x, y, Cell::Empty),
                _ => (),
            };
        }
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let robot_count = parse_input!(input_line, i32);
    for _ in 0..robot_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);

        let cell = match inputs[2].trim() {
            "U" => Cell::UpArrow,
            "D" => Cell::DownArrow,
            "L" => Cell::LeftArrow,
            "R" => Cell::RightArrow,
            _ => Cell::Empty,
        };

        let robot = robot::Robot::new(x, y, cell);
        board.add_robot(robot);
    }

    board
}
