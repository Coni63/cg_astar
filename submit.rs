use std::collections::HashMap;
use std::io;
use std::time::Instant;

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

    board
}

pub fn load_robots() -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let robot_count = parse_input!(input_line, i32);
    for i in 0..robot_count {
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

        robots.push(Robot::new(i as i8, y * 19 + x, cell));
    }
    robots
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
                State::RightArrow => "R",
                _ => continue,
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

pub struct Robot {
    pub id: i8,
    pub idx: usize,
    pub initial_idx: usize,
    pub direction: Direction,
    pub initial_direction: Direction,
    pub alive: bool,
    pub visited: [bool; 800],
}

impl Robot {
    pub fn new(id: i8, idx: usize, direction: Direction) -> Robot {
        Robot {
            id,
            idx,
            direction: direction.clone(),
            initial_idx: idx,
            initial_direction: direction.clone(),
            alive: true,
            visited: [false; 800],
        }
    }

    pub fn reset(&mut self) {
        self.idx = self.initial_idx;
        self.direction = self.initial_direction.clone();
        self.alive = true;
        self.visited = [false; 800];
    }

    pub fn set_visited(&mut self) {
        let offset = match self.direction {
            Direction::Up => 0,
            Direction::Down => 200,
            Direction::Left => 400,
            Direction::Right => 600,
        };
        self.visited[self.idx + offset] = true;
    }

    pub fn visited(&self) -> bool {
        let offset = match self.direction {
            Direction::Up => 0,
            Direction::Down => 200,
            Direction::Left => 400,
            Direction::Right => 600,
        };
        self.visited[self.idx + offset]
    }
}

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

pub struct Board {
    cells: [Cell; 190],
    width: usize,
    height: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::default(); 190],
            width: 19,
            height: 10,
        }
    }

    pub fn show(&self) {
        for i in 0..10 {
            for j in 0..19 {
                let letter = match self.cells[i * 19 + j].state {
                    State::UpArrow => "^",
                    State::DownArrow => "v",
                    State::LeftArrow => "<",
                    State::RightArrow => ">",
                    State::Free => ".",
                    State::Empty => "#",
                };
                eprint!("{}", letter);
            }
            eprintln!();
        }
    }

    pub fn setup(&mut self, x: usize, y: usize, state: State) {
        let idx = y * self.width + x;

        let top_row = if y == 0 { 9 } else { y - 1 };
        let bottom_row = if y == 9 { 0 } else { y + 1 };
        let left_col = if x == 0 { 18 } else { x - 1 };
        let right_col = if x == 18 { 0 } else { x + 1 };

        self.cells[idx] = Cell {
            x: x as u8,
            y: y as u8,
            state,
            modifiable: state == State::Free,
            up: top_row * self.width + x,
            down: bottom_row * self.width + x,
            left: y * self.width + left_col,
            right: y * self.width + right_col,
        };
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn get_cell_idx(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn force_arrow(&mut self, idx: usize, state: State) {
        self.cells[idx].state = state;
        self.cells[idx].modifiable = false;
    }

    pub fn apply_solution(&mut self, solution: &Solution) {
        for (idx, state) in solution.variant_arrows.iter() {
            self.cells[*idx].state = *state;
        }
    }

    pub fn remove_solution(&mut self, solution: &Solution) {
        for (idx, _) in solution.variant_arrows.iter() {
            self.cells[*idx].state = State::Free;
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        let mut board = Board::new();
        board.cells = self.cells;
        board.width = self.width;
        board.height = self.height;
        board
    }
}

const W: usize = 19;
const H: usize = 10;
const N: usize = W * H;

// Cell codes. Directions: 0 = Up, 1 = Right, 2 = Down, 3 = Left
const NONE: u8 = 4;
const VOID: u8 = 5;
/// Pseudo state reached when a robot walks into the void
const DEAD: u16 = (N * 4) as u16;

fn state_to_code(state: State) -> u8 {
    match state {
        State::UpArrow => 0,
        State::RightArrow => 1,
        State::DownArrow => 2,
        State::LeftArrow => 3,
        State::Free => NONE,
        State::Empty => VOID,
    }
}

fn code_to_state(code: u8) -> State {
    match code {
        0 => State::UpArrow,
        1 => State::RightArrow,
        2 => State::DownArrow,
        3 => State::LeftArrow,
        NONE => State::Free,
        _ => State::Empty,
    }
}

fn direction_to_code(direction: &Direction) -> u8 {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

// Annealing schedule (tuned on the 30 tests of tests/)
const EXPLORATIONS: usize = 2;
const EXPLORATION_SHARE: f64 = 0.6;
const T_START: f64 = 10.0;
const T_START_REFINE: f64 = 3.0;
const T_END: f64 = 2.0;

/// Xorshift64: much cheaper than ThreadRng, which matters in the hot loop.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed | 1)
    }

    #[inline]
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    #[inline]
    fn below(&mut self, n: usize) -> usize {
        (((self.next() >> 32) * n as u64) >> 32) as usize
    }

    #[inline]
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// A symmetry of the torus (mirror along x and/or y) mapping the map and the robots onto themselves.
#[derive(Clone)]
struct Transform {
    cell: [u16; N],
    /// Image of each cell code (arrows are mirrored, NONE / VOID unchanged)
    code: [u8; 6],
}

impl Transform {
    fn new(mirror_x: Option<usize>, mirror_y: Option<usize>) -> Transform {
        let mut t = Transform {
            cell: [0; N],
            code: [0, 1, 2, 3, NONE, VOID],
        };
        for y in 0..H {
            for x in 0..W {
                let nx = mirror_x.map_or(x, |a| (a + W - x) % W);
                let ny = mirror_y.map_or(y, |b| (b + H - y) % H);
                t.cell[y * W + x] = (ny * W + nx) as u16;
            }
        }
        if mirror_x.is_some() {
            t.code.swap(1, 3);
        }
        if mirror_y.is_some() {
            t.code.swap(0, 2);
        }
        t
    }
}

pub struct Solver {
    /// next[cell][dir] = neighbouring cell index (torus)
    next: [[u16; 4]; N],
    /// Current content of each cell (0..3 arrow, NONE, VOID)
    grid: [u8; N],
    /// Allowed values for each candidate cell: NONE, then arrows not pointing to the void
    opts: [[u8; 5]; N],
    opt_len: [u8; N],
    /// Cells where an arrow may be placed (at least one option besides NONE)
    candidates: Vec<usize>,
    robots: Vec<(usize, u8)>,
    fixed_arrows: Vec<(usize, State)>,
    /// Connected component of each platform cell, cells of each component, component of each robot
    comp: [u8; N],
    comp_cells: Vec<Vec<u16>>,
    robot_comp: Vec<usize>,
    /// Non-identity symmetries of the map (at most one rotation)
    symmetries: Vec<Transform>,

    /// visited[cell * 4 + dir] == stamp  <=> state visited by the robot being simulated
    visited: [u32; N * 4],
    stamp: u32,
    /// trans[pos * 4 + dir] = state reached at the next turn (DEAD if falling)
    trans: [u16; N * 4],
    rng: Rng,
}

/// Best solution found, kept per connected component of the map: components
/// are independent, so their best configurations can be combined.
struct Best {
    comp_score: Vec<i32>,
    grid: [u8; N],
}

impl Best {
    fn score(&self) -> i32 {
        self.comp_score.iter().map(|&s| s.max(0)).sum()
    }
}

/// Trajectory of a simulated robot during an annealing, kept to re-simulate
/// only from the first step reaching a modified cell.
struct Track {
    robot: usize,
    /// Number of robots it stands for (symmetry), and their components
    weight: i32,
    comps: Vec<usize>,
    /// States (cell * 4 + dir) in walking order; its length is the robot score
    path: Vec<u16>,
    /// step[state] = index in path + 1, 0 when not visited
    step: Vec<u16>,
    /// First step on each cell, u16::MAX if never visited
    first: [u16; N],
    /// Pending re-simulation: states from step `from`, and the resulting score
    suffix: Vec<u16>,
    from: usize,
    new_score: i32,
}

impl Track {
    /// Replaces the path from step `from` with the re-simulated suffix.
    fn commit(&mut self) {
        let from = self.from;
        for &state in self.path[from..].iter() {
            self.step[state as usize] = 0;
            let cell = state as usize >> 2;
            if self.first[cell] as usize >= from {
                self.first[cell] = u16::MAX;
            }
        }
        self.path.truncate(from);
        for (i, &state) in self.suffix.iter().enumerate() {
            let step = from + i;
            self.step[state as usize] = (step + 1) as u16;
            self.path.push(state);
            let cell = state as usize >> 2;
            if self.first[cell] == u16::MAX {
                self.first[cell] = step as u16;
            }
        }
    }
}

impl Solver {
    pub fn new(board: &mut Board, robots: &[Robot]) -> Solver {
        let fixed_arrows = Self::get_deadend(board);

        let mut solver = Solver {
            next: [[0; 4]; N],
            grid: [VOID; N],
            opts: [[NONE; 5]; N],
            opt_len: [1; N],
            candidates: Vec::new(),
            robots: robots
                .iter()
                .map(|r| (r.initial_idx, direction_to_code(&r.initial_direction)))
                .collect(),
            fixed_arrows,
            comp: [u8::MAX; N],
            comp_cells: Vec::new(),
            robot_comp: Vec::new(),
            symmetries: Vec::new(),
            visited: [0; N * 4],
            trans: [DEAD; N * 4],
            stamp: 0,
            rng: Rng::new(0x9E37_79B9_7F4A_7C15 ^ ((std::process::id() as u64) << 20)),
        };

        let mut modifiable = [false; N];
        for (idx, cell) in board.get_cells().iter().enumerate() {
            solver.next[idx] = [
                cell.up as u16,
                cell.right as u16,
                cell.down as u16,
                cell.left as u16,
            ];
            solver.grid[idx] = state_to_code(cell.state);
            modifiable[idx] = cell.modifiable;
        }

        for idx in 0..N {
            // Arrows inside corridors are kept: a U-turn can be worth it.
            if !modifiable[idx] {
                continue;
            }
            let mut len = 1;
            for dir in 0..4u8 {
                if solver.grid[solver.next[idx][dir as usize] as usize] != VOID {
                    solver.opts[idx][len] = dir;
                    len += 1;
                }
            }
            solver.opt_len[idx] = len as u8;
            if len > 1 {
                solver.candidates.push(idx);
            }
        }

        solver.rebuild_trans();
        solver.detect_components();
        solver.detect_symmetries(&modifiable);
        eprintln!(
            "Candidates: {}, symmetries: {}",
            solver.candidates.len(),
            solver.symmetries.len()
        );
        solver
    }

    fn detect_components(&mut self) {
        for idx in 0..N {
            if self.grid[idx] == VOID || self.comp[idx] != u8::MAX {
                continue;
            }
            let id = self.comp_cells.len() as u8;
            let mut cells = vec![idx as u16];
            self.comp[idx] = id;
            let mut i = 0;
            while i < cells.len() {
                let cell = cells[i] as usize;
                for dir in 0..4 {
                    let n = self.next[cell][dir] as usize;
                    if self.grid[n] != VOID && self.comp[n] == u8::MAX {
                        self.comp[n] = id;
                        cells.push(n as u16);
                    }
                }
                i += 1;
            }
            self.comp_cells.push(cells);
        }
        self.robot_comp = self
            .robots
            .iter()
            .map(|&(pos, _)| self.comp[pos] as usize)
            .collect();
    }

    // ------------------------------------------------------------------
    // Symmetries
    // ------------------------------------------------------------------

    fn is_symmetry(&self, t: &Transform, modifiable: &[bool; N]) -> bool {
        for idx in 0..N {
            let image = t.cell[idx] as usize;
            if self.grid[image] != t.code[self.grid[idx] as usize]
                || modifiable[image] != modifiable[idx]
            {
                return false;
            }
        }
        self.robots.iter().all(|&(pos, dir)| {
            let image = (t.cell[pos] as usize, t.code[dir as usize]);
            self.robots.contains(&image)
        })
    }

    /// Only point symmetries (180° rotations) are used: constraining a solution
    /// to be mirror-symmetric turned out to hurt a lot, whereas rotation helps.
    fn detect_symmetries(&mut self, modifiable: &[bool; N]) {
        self.symmetries = (0..W)
            .flat_map(|a| (0..H).map(move |b| (a, b)))
            .map(|(a, b)| Transform::new(Some(a), Some(b)))
            .find(|t| self.is_symmetry(t, modifiable))
            .into_iter()
            .collect();
    }

    /// Robots to simulate, with the number of robots each one stands for and
    /// the components of those robots.
    fn representatives(&self, symmetric: bool) -> Vec<(usize, i32, Vec<usize>)> {
        if !symmetric {
            return (0..self.robots.len())
                .map(|r| (r, 1, vec![self.robot_comp[r]]))
                .collect();
        }
        let mut seen = vec![false; self.robots.len()];
        let mut reps = Vec::new();
        for r in 0..self.robots.len() {
            if seen[r] {
                continue;
            }
            seen[r] = true;
            let mut comps = vec![self.robot_comp[r]];
            let (pos, dir) = self.robots[r];
            for t in self.symmetries.iter() {
                let image = (t.cell[pos] as usize, t.code[dir as usize]);
                let o = self.robots.iter().position(|&x| x == image).unwrap();
                if !seen[o] {
                    seen[o] = true;
                    comps.push(self.robot_comp[o]);
                }
            }
            reps.push((r, comps.len() as i32, comps));
        }
        reps
    }

    // ------------------------------------------------------------------
    // Simulation
    // ------------------------------------------------------------------

    #[inline]
    fn transition(&self, pos: usize, dir: usize) -> u16 {
        let next = self.next[pos][dir] as usize;
        match self.grid[next] {
            VOID => DEAD,
            NONE => (next * 4 + dir) as u16,
            arrow => (next * 4 + arrow as usize) as u16,
        }
    }

    fn rebuild_trans(&mut self) {
        for pos in 0..N {
            for dir in 0..4 {
                self.trans[pos * 4 + dir] = self.transition(pos, dir);
            }
        }
    }

    /// Changes a cell and updates the transitions of the four states entering it.
    #[inline]
    fn set_cell(&mut self, idx: usize, value: u8) {
        self.grid[idx] = value;
        for dir in 0..4 {
            let prev = self.next[idx][(dir + 2) & 3] as usize;
            self.trans[prev * 4 + dir] = self.transition(prev, dir);
        }
    }

    // ------------------------------------------------------------------
    // Simulated annealing
    // ------------------------------------------------------------------

    /// Adds the change `idx <- value` (and its symmetric images) to `changes`.
    /// Returns false when it conflicts with a change already there.
    fn push_change(
        &self,
        changes: &mut Vec<(usize, u8)>,
        idx: usize,
        value: u8,
        symmetric: bool,
    ) -> bool {
        let start = changes.len();
        let add = |changes: &mut Vec<(usize, u8)>, i: usize, v: u8| -> bool {
            if let Some(&(_, other)) = changes.iter().find(|(j, _)| *j == i) {
                return other == v;
            }
            changes.push((i, v));
            true
        };
        let mut ok = add(changes, idx, value);
        if symmetric {
            for t in self.symmetries.iter() {
                ok = ok && add(changes, t.cell[idx] as usize, t.code[value as usize]);
            }
        }
        if !ok {
            changes.truncate(start);
        }
        ok
    }

    /// Re-simulates the robot of track `k` from step `from` (the steps before are
    /// unchanged), writing the new states into `track.suffix`. Returns the new score.
    #[inline]
    fn resimulate(&mut self, track: &mut Track, from: usize) -> i32 {
        self.stamp += 1;
        let stamp = self.stamp;
        track.suffix.clear();

        let mut state = if from == 0 {
            let (start, start_dir) = self.robots[track.robot];
            let dir = if self.grid[start] < 4 {
                self.grid[start]
            } else {
                start_dir
            };
            start * 4 + dir as usize
        } else {
            let next = self.trans[track.path[from - 1] as usize];
            // A state of the prefix has step + 1 <= from
            if next == DEAD
                || track.step[next as usize] != 0 && track.step[next as usize] as usize <= from
            {
                return from as i32;
            }
            next as usize
        };

        loop {
            self.visited[state] = stamp;
            track.suffix.push(state as u16);
            let next = self.trans[state];
            if next == DEAD {
                break;
            }
            let n = next as usize;
            if self.visited[n] == stamp || track.step[n] != 0 && track.step[n] as usize <= from {
                break;
            }
            state = n;
        }
        (from + track.suffix.len()) as i32
    }

    /// Anneals from the current grid until `end_ms`; updates `best` when improved.
    fn anneal(
        &mut self,
        start: &Instant,
        end_ms: f64,
        symmetric: bool,
        t0: f64,
        t1: f64,
        best: &mut Best,
    ) -> u64 {
        let mut tracks: Vec<Track> = self
            .representatives(symmetric)
            .into_iter()
            .map(|(robot, weight, comps)| Track {
                robot,
                weight,
                comps,
                path: Vec::with_capacity(N * 4),
                step: vec![0; N * 4],
                first: [u16::MAX; N],
                suffix: Vec::with_capacity(N * 4),
                from: 0,
                new_score: 0,
            })
            .collect();
        let n = tracks.len();

        let mut current = 0;
        let mut comp_score = vec![0; self.comp_cells.len()];
        let mut dirty = vec![false; self.comp_cells.len()];
        for track in tracks.iter_mut() {
            let s = self.resimulate(track, 0);
            track.from = 0;
            track.commit();
            current += s * track.weight;
            for &comp in track.comps.iter() {
                comp_score[comp] += s;
                dirty[comp] = true;
            }
        }
        self.save_best(best, &comp_score, &mut dirty);

        let begin = start.elapsed().as_secs_f64() * 1000.0;
        let duration = end_ms - begin;
        if duration <= 0.0 {
            return 0;
        }
        let mut temperature = t0;
        let mut iterations: u64 = 0;
        let mut changes: Vec<(usize, u8)> = Vec::with_capacity(16);
        let mut previous: Vec<(usize, u8)> = Vec::with_capacity(16);
        let mut touched: Vec<usize> = Vec::with_capacity(n);

        loop {
            if iterations & 127 == 0 {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0 - begin;
                if elapsed >= duration {
                    break;
                }
                temperature = t0 * (t1 / t0).powf(elapsed / duration);
            }
            iterations += 1;

            // Mutation: one to three cells get a new random value
            changes.clear();
            let count = match self.rng.below(8) {
                0..=4 => 1,
                5 | 6 => 2,
                _ => 3,
            };
            for _ in 0..count {
                let idx = self.candidates[self.rng.below(self.candidates.len())];
                let len = self.opt_len[idx] as usize;
                let mut value = self.grid[idx];
                while value == self.grid[idx] {
                    value = self.opts[idx][self.rng.below(len)];
                }
                self.push_change(&mut changes, idx, value, symmetric);
            }
            if changes.is_empty() {
                continue;
            }

            previous.clear();
            for &(idx, value) in changes.iter() {
                previous.push((idx, self.grid[idx]));
                self.set_cell(idx, value);
            }

            // Incremental evaluation: only the robots walking on a modified cell,
            // from the first time they reach one
            touched.clear();
            let mut score = current;
            for k in 0..n {
                let track = &mut tracks[k];
                let mut from = u16::MAX;
                for &(i, _) in changes.iter() {
                    from = from.min(track.first[i]);
                }
                if from == u16::MAX {
                    continue;
                }
                track.from = from as usize;
                let s = self.resimulate(track, from as usize);
                score += (s - track.path.len() as i32) * track.weight;
                track.new_score = s;
                touched.push(k);
            }

            let delta = score - current;
            if delta >= 0 || self.rng.unit() < (delta as f64 / temperature).exp() {
                current = score;
                for &k in touched.iter() {
                    let track = &mut tracks[k];
                    let diff = track.new_score - track.path.len() as i32;
                    for &comp in track.comps.iter() {
                        comp_score[comp] += diff;
                        dirty[comp] = true;
                    }
                    track.commit();
                }
                self.save_best(best, &comp_score, &mut dirty);
            } else {
                for &(idx, old) in previous.iter().rev() {
                    self.set_cell(idx, old);
                }
            }
        }
        iterations
    }

    /// Saves the components of the current grid that beat their best score.
    fn save_best(&self, best: &mut Best, comp_score: &[i32], dirty: &mut [bool]) {
        for comp in 0..comp_score.len() {
            if dirty[comp] {
                dirty[comp] = false;
                if comp_score[comp] > best.comp_score[comp] {
                    best.comp_score[comp] = comp_score[comp];
                    for &idx in self.comp_cells[comp].iter() {
                        best.grid[idx as usize] = self.grid[idx as usize];
                    }
                }
            }
        }
    }

    pub fn solve(&mut self, start: &Instant, deadline_ms: f64) -> Solution {
        let base_grid = self.grid;
        let mut best = Best {
            comp_score: vec![-1; self.comp_cells.len()],
            grid: self.grid,
        };

        if self.candidates.is_empty() {
            // Nothing to decide: a zero-length annealing just evaluates the grid
            self.anneal(start, 0.0, false, 1.0, 1.0, &mut best);
            return self.to_solution(&base_grid, best.score());
        }

        let begin = start.elapsed().as_secs_f64() * 1000.0;
        let total = deadline_ms - begin;
        let symmetric = !self.symmetries.is_empty();
        let mut iterations = 0;

        // Exploration: independent anneals from the empty grid (symmetric when possible)
        for k in 0..EXPLORATIONS {
            self.grid = base_grid;
            self.rebuild_trans();
            let end = begin + total * EXPLORATION_SHARE * (k + 1) as f64 / EXPLORATIONS as f64;
            iterations += self.anneal(start, end, symmetric, T_START, T_END, &mut best);
            eprintln!("Exploration {} best: {}", k, best.score());
        }
        // Refinement of the best solution found
        self.grid = best.grid;
        self.rebuild_trans();
        iterations += self.anneal(start, deadline_ms, false, T_START_REFINE, T_END, &mut best);

        eprintln!("SA iterations: {}", iterations);
        self.grid = base_grid;
        self.to_solution(&best.grid, best.score())
    }

    fn to_solution(&self, grid: &[u8; N], score: i32) -> Solution {
        let mut solution = Solution {
            fixed_arrows: self.fixed_arrows.clone(),
            variant_arrows: Vec::new(),
            score,
        };
        for &idx in self.candidates.iter() {
            if grid[idx] < 4 {
                solution
                    .variant_arrows
                    .push((idx, code_to_state(grid[idx])));
            }
        }
        solution
    }

    /// A platform cell with a single platform neighbour: the only sensible arrow
    /// points towards that neighbour (anything else kills the robot).
    fn get_deadend(board: &mut Board) -> Vec<(usize, State)> {
        let mut fixed_arrows = Vec::new();

        for (idx, cell) in board.get_cells().iter().enumerate() {
            if !cell.modifiable {
                continue;
            }

            let mut count = 0;
            let mut free_direction: State = State::Empty;
            for (neighbour, arrow) in [
                (cell.up, State::UpArrow),
                (cell.down, State::DownArrow),
                (cell.left, State::LeftArrow),
                (cell.right, State::RightArrow),
            ] {
                if board.get_cell_idx(neighbour).state == State::Empty {
                    count += 1;
                } else {
                    free_direction = arrow;
                }
            }

            if count == 3 {
                fixed_arrows.push((idx, free_direction));
            }
        }

        for (idx, arrow) in fixed_arrows.iter() {
            board.force_arrow(*idx, *arrow);
        }

        eprintln!("Deadend: {}", fixed_arrows.len());
        fixed_arrows
    }
}

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
                State::UpArrow => robot.direction = Direction::Up,
                State::DownArrow => robot.direction = Direction::Down,
                State::LeftArrow => robot.direction = Direction::Left,
                State::RightArrow => robot.direction = Direction::Right,
                _ => (),
            }

            // Les Automaton2000 meurent s'ils ont marchés dans le vide ou s'ils sont dans un état (position,direction) déjà visité (Les Automaton2000 ne partagent pas leur historique d'états).
            if next_cell.state == State::Empty {
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
            //         robot.idx, cell.x, cell.y, robot.direction, next_cell.x, next_cell.y, score
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
    let mut board = load_board();
    let mut robots = load_robots();
    // The CodinGame timer starts when the input is sent, not when the process starts
    let start_time = Instant::now();

    let mut solver = Solver::new(&mut board, &robots);
    let solution = solver.solve(&start_time, 900.0);

    eprintln!("All runs best Score: {}", solution.score);
    eprintln!("Elapsed: {:?}", start_time.elapsed());
    println!("{}", solution.to_string());
}
