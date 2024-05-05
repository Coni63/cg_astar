use std::io;

use crate::{
    board::Board,
    cell::State,
    robot::{Direction, Robot},
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

pub fn load_board() -> Board {
    let mut board = Board::new();

    for y in 0..10 {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        eprintln!("{}", line);

        for (x, c) in line.chars().enumerate() {
            match c {
                'U' => board.setup(x, y, State::UpArrow),
                'D' => board.setup(x, y, State::DownArrow),
                'L' => board.setup(x, y, State::LeftArrow),
                'R' => board.setup(x, y, State::RightArrow),
                '.' => board.setup(x, y, State::Free),
                '#' => board.setup(x, y, State::Empty),
                _ => (),
            };
        }
    }

    board.post_init();

    board
}

pub fn load_robots() -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let robot_count = parse_input!(input_line, i32);
    eprintln!("{} robot(s)", robot_count);
    for _ in 0..robot_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let x = parse_input!(inputs[0], usize);
        let y = parse_input!(inputs[1], usize);

        let cell = match inputs[2].trim() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => Direction::Right,
        };

        eprintln!("Robot at ({}, {}) facing {:?}", x, y, cell);

        robots.push(Robot::new(y * 19 + x, cell));
    }
    robots
}
