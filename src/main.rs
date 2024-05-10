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
        robot.set_visited();
    }

    loop {
        let mut game_over = true;
        for robot in robots.iter_mut().filter(|r| r.alive) {
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
                        robot.id, cell.x, cell.y
                    );
                }
                continue;
            }

            if robot.visited() {
                robot.alive = false;
                if details {
                    eprintln!(
                        "Robot {} died at ({}, {}) -- already visited",
                        robot.id, cell.x, cell.y
                    );
                }
                continue;
            }

            robot.set_visited();

            // if details {
            //     eprintln!(
            //         "Robot {} at ({}, {}) facing {:?} -> ({}, {}) | {:?}",
            //         robot.id, cell.x, cell.y, robot.direction, next_cell.x, next_cell.y, score
            //     );
            // }
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

    let mut all_best = Solution::default();

    for turn in 1..11 {
        let end_time = turn * 95;
        let mut temperature = 10.0;
        let mut cooling_rate = 0.997;

        eprintln!("New run: {}/10", turn);
        let mut base_solution = solver.get_base_solution();
        base_solution.score = play(&mut board, &mut robots, &base_solution, false);

        let mut best_solution = base_solution.clone();
        let mut curr_solution = base_solution.clone();

        while start_time.elapsed().as_millis() < end_time {
            solver.update(&mut curr_solution);

            curr_solution.score = play(&mut board, &mut robots, &curr_solution, false);
            // eprintln!("Score: {}", solution.score);

            if curr_solution.score > base_solution.score {
                // eprintln!("Update: {} -> {}", base_solution.score, curr_solution.score);
                base_solution = curr_solution.clone();
            } else {
                let p =
                    0.5 * ((curr_solution.score - base_solution.score) as f64 / temperature).exp();
                // eprintln!("{} -> {} ({})", base_solution.score, curr_solution.score, p);
                temperature *= cooling_rate;
                if rng.gen::<f64>() < p {
                    base_solution = curr_solution.clone();
                }
            }

            if curr_solution.score > best_solution.score {
                eprintln!(
                    "Best: {} -> {} ({:?})",
                    best_solution.score,
                    curr_solution.score,
                    start_time.elapsed()
                );
                best_solution = curr_solution.clone();
            }

            curr_solution = base_solution.clone();
        }

        eprintln!("Run {} best Score: {}", turn, best_solution.score);
        if best_solution.score > all_best.score {
            all_best = best_solution.clone();
        }
    }

    // board.show();
    // board.apply_solution(&best_solution);
    // board.show();
    // board.remove_solution(&best_solution);
    // eprintln!("{}", play(&mut board, &mut robots, &best_solution, true));
    // board.show();

    play(&mut board, &mut robots, &all_best, true);

    eprintln!("All runs best Score: {}", all_best.score);
    println!("{}", all_best.to_string());
}
