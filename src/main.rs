use std::time::Instant;

use solution::Solution;
use solver::Solver;

mod board;
mod cell;
mod loader;
mod robot;
mod solution;
mod solver;

fn play(board: &mut board::Board, robots: &mut [robot::Robot], solution: &mut Solution) {
    board.apply_solution(solution);

    solution.score = 0;

    // Au premier tour Automaton2000 change de direction s'il est sur une flèche (i.e : vous pouvez changer la direction initiale d'Automaton2000 en plaçant une flèche sous lui).
    for robot in robots.iter_mut() {
        let cell = board.get_cell_idx(robot.idx);
        match cell.state {
            cell::State::UpArrow => robot.direction = robot::Direction::Up,
            cell::State::DownArrow => robot.direction = robot::Direction::Down,
            cell::State::LeftArrow => robot.direction = robot::Direction::Left,
            cell::State::RightArrow => robot.direction = robot::Direction::Right,
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
            solution.score += 1;

            // Les Automaton2000 avancent d'une case dans la direction vers laquelle ils font face.
            let cell = board.get_cell_idx(robot.idx);
            let next_idx = match robot.direction {
                robot::Direction::Up => cell.up,
                robot::Direction::Down => cell.down,
                robot::Direction::Left => cell.left,
                robot::Direction::Right => cell.right,
            };
            robot.idx = next_idx;

            // Les Automaton2000 changent de direction s'ils sont sur une flèche.
            let next_cell = board.get_cell_idx(next_idx);
            match next_cell.state {
                cell::State::UpArrow => robot.direction = robot::Direction::Up,
                cell::State::DownArrow => robot.direction = robot::Direction::Down,
                cell::State::LeftArrow => robot.direction = robot::Direction::Left,
                cell::State::RightArrow => robot.direction = robot::Direction::Right,
                _ => (),
            }

            // Les Automaton2000 meurent s'ils ont marchés dans le vide ou s'ils sont dans un état (position,direction) déjà visité (Les Automaton2000 ne partagent pas leur historique d'états).
            if next_cell.state == cell::State::Empty {
                robot.alive = false;
                eprintln!(
                    "Robot {} died at ({}, {}) -- out of board",
                    robot.idx, cell.x, cell.y
                );
                continue;
            }

            if robot.visited() {
                robot.alive = false;
                eprintln!(
                    "Robot {} died at ({}, {}) -- already visited",
                    robot.idx, cell.x, cell.y
                );
                solution.score -= 1;
                continue;
            }

            robot.set_visited();

            // eprintln!(
            //     "Robot {} at ({}, {}) facing {:?} -> ({}, {})",
            //     robot.idx, cell.x, cell.y, robot.direction, next_cell.x, next_cell.y
            // );
        }

        if game_over {
            break;
        }
    }

    board.remove_solution(solution);
    robots.iter_mut().for_each(|r| r.reset());
}

fn main() {
    let mut board = loader::load_board();
    let mut robots = loader::load_robots();

    let start_time = Instant::now();

    let solver = Solver::new(&mut board);
    let base_solution = solver.get_base_solution();
    let mut best_solution = base_solution.clone();

    loop {
        let mut solution = solver.update(&base_solution);

        play(&mut board, &mut robots, &mut solution);
        eprintln!("Score: {}", solution.score);

        if solution.score > best_solution.score {
            best_solution = solution.clone();
        }

        if start_time.elapsed().as_millis() > 9 {
            break;
        }
    }

    println!("{}", best_solution.to_string());
}
