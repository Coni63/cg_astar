use std::time::Instant;

use board::Board;
use cell::State;
use rand::Rng;
use robot::{Direction, Robot};
use solution::Solution;
use solver::Solver;

mod board;
mod cell;
mod loader;
mod robot;
mod solution;
mod solver;

fn play(board: &mut Board, robots: &mut [Robot], solution: &Solution, details: bool) -> i32 {
    let mut score = 0;

    if details {
        board.show();
        board.apply_solution(solution);
        board.show();
    } else {
        board.apply_solution(solution);
    }

    // Au premier tour Automaton2000 change de direction s'il est sur une flèche (i.e : vous pouvez changer la direction initiale d'Automaton2000 en plaçant une flèche sous lui).
    for robot in robots.iter_mut() {
        let cell = board.get_cell_idx(robot.idx);
        match cell.state {
            State::UpArrow => robot.direction = Direction::Up,
            State::DownArrow => robot.direction = Direction::Down,
            State::LeftArrow => robot.direction = Direction::Left,
            State::RightArrow => robot.direction = Direction::Right,
            _ => (),
        }
    }

    loop {
        let mut game_over = true;
        for robot in robots.iter_mut() {
            if !robot.alive {
                continue;
            }

            game_over = false;

            // Le score est incrémenté de 1 pour chaque robot en vie.
            score += 1;

            // Les Automaton2000 avancent d'une case dans la direction vers laquelle ils font face.
            let cell = board.get_cell_idx(robot.idx);
            let next_idx = match robot.direction {
                Direction::Up => cell.up,
                Direction::Down => cell.down,
                Direction::Left => cell.left,
                Direction::Right => cell.right,
            };
            robot.idx = next_idx;

            // Les Automaton2000 changent de direction s'ils sont sur une flèche.
            let next_cell = board.get_cell_idx(next_idx);
            match next_cell.state {
                cell::State::UpArrow => robot.direction = Direction::Up,
                cell::State::DownArrow => robot.direction = Direction::Down,
                cell::State::LeftArrow => robot.direction = Direction::Left,
                cell::State::RightArrow => robot.direction = Direction::Right,
                _ => (),
            }

            // Les Automaton2000 meurent s'ils ont marchés dans le vide ou s'ils sont dans un état (position,direction) déjà visité (Les Automaton2000 ne partagent pas leur historique d'états).
            if next_cell.state == cell::State::Empty {
                robot.alive = false;
                if details {
                    eprintln!(
                        "Robot {} died at ({}, {}) -- empty cell",
                        robot.idx, cell.x, cell.y
                    );
                }
                continue;
            }

            if robot.visited() {
                robot.alive = false;
                if details {
                    eprintln!(
                        "Robot {} died at ({}, {}) -- already visited",
                        robot.idx, cell.x, cell.y
                    );
                }
                score -= 1;
                continue;
            }

            robot.set_visited();

            if details {
                eprintln!(
                    "Robot {} at ({}, {}) facing {:?} -> ({}, {})",
                    robot.idx, cell.x, cell.y, robot.direction, next_cell.x, next_cell.y
                );
            }
        }

        if game_over {
            break;
        }
    }

    board.remove_solution(solution);
    robots.iter_mut().for_each(|r| r.reset());

    score
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = loader::load_board();
    let mut robots = loader::load_robots();

    let start_time = Instant::now();

    let mut solver = Solver::new(&mut board);

    let mut base_solution = solver.get_base_solution();
    base_solution.score = play(&mut board, &mut robots, &base_solution, false);

    let mut best_solution = base_solution.clone();
    let mut curr_solution = base_solution.clone();

    eprintln!("Time: {:?}", start_time.elapsed().as_millis());

    loop {
        solver.update(&mut curr_solution);

        curr_solution.score = play(&mut board, &mut robots, &curr_solution, false);
        // eprintln!("Score: {}", solution.score);

        let force_update = rng.gen::<f64>() < 0.1;
        if curr_solution.score > base_solution.score || force_update {
            // eprintln!("Update: {} -> {}", base_solution.score, curr_solution.score);
            base_solution = curr_solution.clone();
        }

        if curr_solution.score > best_solution.score {
            eprintln!("Best: {} -> {}", best_solution.score, curr_solution.score);
            best_solution = curr_solution.clone();
        }

        curr_solution = base_solution.clone();

        if start_time.elapsed().as_millis() > 900 {
            break;
        }
    }

    play(&mut board, &mut robots, &best_solution, true);

    eprintln!("Best Score: {}", best_solution.score);
    println!("{}", best_solution.to_string());
}
