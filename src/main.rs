#[macro_use]
extern crate indoc;

use std::cmp::max;
use std::collections::BTreeSet;
use std::fmt;

const MAX_DIM: usize = 16;

#[derive(Clone)]
struct Board {
    a: [[i8; MAX_DIM]; MAX_DIM],
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Piece {
    a: [[i8; MAX_DIM]; MAX_DIM],
    w: usize,
    h: usize,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
}

impl Piece {
    fn rotate(&self) -> Piece {
        let mut p = Piece {
            a: [[0; MAX_DIM]; MAX_DIM],
            w: self.h,
            h: self.w,
        };
        for y in 0..p.h {
            for x in 0..p.w {
                p.a[y][x] = self.a[self.h - 1 - x][y];
            }
        }
        p
    }

    fn from_str(s: &str) -> Piece {
        let mut f = Piece {
            a: [[0; MAX_DIM]; MAX_DIM],
            w: 1,
            h: 1,
        };
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                f.w = max(f.w, x + 1);
                f.h = max(f.h, y + 1);
                match c {
                    '#' => f.a[y][x] = 1,
                    '.' => (),
                    _ => panic!("Map symbol unknown: {}", c)
                }
            }
        }
        f
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repos = String::new();
        repos.push('\n');
        for y in 0..self.h {
            for x in 0..self.w {
                repos.push(if self.a[y][x] > 0 { '#' } else { '.' });
            }
            repos.push('\n');
        }
        f.write_str(&repos)
    }
}

impl Board {
    fn from_str(s: &str) -> Board {
        let mut f = Board {
            a: [[1; MAX_DIM]; MAX_DIM],
        };
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => (),
                    '.' => f.a[y][x] = 0,
                    _ => panic!("Map symbol unknown: {}", c)
                }
            }
        }
        f
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repos = String::new();
        repos.push('\n');
        for y in 0..MAX_DIM {
            for x in 0..MAX_DIM {
                repos.push(if self.a[y][x] > 0 { '#' } else { '.' });
            }
            repos.push('\n');
        }
        f.write_str(&repos)
    }
}

fn pieces_with_rotations(pieces: Vec<Piece>) -> Vec<BTreeSet<Piece>> {
    let mut res = vec![];
    for p in pieces {
        let mut rotas = BTreeSet::new();
        let mut pp = p;
        for _ in 0..4 {
            let next = pp.rotate();
            rotas.insert(pp);
            pp = next;
        }
        res.push(rotas);
    }
    res
}

fn place_piece(b: &mut Board, piece: &Piece, pos: Position, negate: bool) -> (bool, i32) {
    let mult = if negate { -1i8 } else { 1i8 };
    let mut any_good = false;
    let mut diff_penalty = 0;
    for y in 0..piece.h {
        for x in 0..piece.w {
            if b.a[y + pos.y][x + pos.x] == 0 {
                any_good = true;
            } else {
                diff_penalty += 1;
            }
            b.a[y + pos.y][x + pos.x] += piece.a[y][x] * mult;
        }
    }
    (any_good, diff_penalty)
}

fn calculate_penalty(b: &Board) -> i32 {
    let mut penalty: i32 = 0;
    for y in 0..MAX_DIM {
        for x in 0..MAX_DIM {
            if b.a[y][x] == 0 {
                return std::i32::MAX; // Failure
            }
            penalty += b.a[y][x] as i32 - 1;
        }
    }
    penalty
}

type Solution<'a> = Vec<(&'a Piece, Option<Position>)>;

#[derive(Clone)]
struct SolutionState<'a> {
    board: Board,
    solution: Solution<'a>,
    current_penalty: i32
}

fn advance_solution_vector_or_test_solution<'a>(
    pieces_with_rotations: &'a Vec<BTreeSet<Piece>>,
    state: &mut SolutionState<'a>,
    min_penalty: &mut i32,
    best_solution: &mut Option<Solution<'a>>,
) {
    if state.solution.len() == pieces_with_rotations.len() {
        let total_penalty = calculate_penalty(&mut state.board);
        if total_penalty < *min_penalty {
            *min_penalty = total_penalty;
            *best_solution = Some(state.solution.clone());
        }
    } else {
        let rotations = &pieces_with_rotations[state.solution.len()];
        for piece in rotations {

            state.solution.push((&piece, None));
            advance_solution_vector_or_test_solution(
                pieces_with_rotations,
                state,
                min_penalty,
                best_solution,
            );
            state.solution.pop();

            for y in 0..(MAX_DIM - piece.h) {
                for x in 0..(MAX_DIM - piece.w) {
                    let pos = Position { y: y, x: x };
                    state.solution.push((&piece, Some(pos)));
                    let (good_placement, added_penalty) = place_piece(&mut state.board, &piece, pos, false);
                    state.current_penalty += added_penalty;
                    if good_placement && state.current_penalty <= *min_penalty {
                        advance_solution_vector_or_test_solution(
                            pieces_with_rotations,
                            state,
                            min_penalty,
                            best_solution,
                        );
                    }
                    state.current_penalty -= added_penalty;
                    place_piece(&mut state.board, &piece, pos, true);
                    state.solution.pop();
                }
            }
        }
    }
}

fn render(pc: &Piece, pos: Position) -> String {
    let mut r = String::new();
    for y in 0..MAX_DIM {
        for x in 0..MAX_DIM {
            r.push(
                if x >= pos.x && y >= pos.y && y - pos.y < pc.h && x - pos.x < pc.w
                    && pc.a[y - pos.y][x - pos.x] > 0
                {
                    '#'
                } else {
                    '.'
                },
            );
        }
        r.push('\n');
    }
    r
}

fn main() {
    let board = Board::from_str(indoc!("
        ##########.#
        #....####..#
        #.#....###.#
        ############
    "));
    let pieces = vec![
        Piece::from_str(indoc!("
            ###
            .#.
        ")),
        Piece::from_str(indoc!("
            ##
            ##
        ")),
        Piece::from_str(indoc!("
            ####
        ")),
        Piece::from_str(indoc!("
            ###
        ")),
    ];

    let pwr = pieces_with_rotations(pieces);
    let mut min_penalty = std::i32::MAX;
    let mut best_solution: Option<Solution> = None;

    let mut state = SolutionState{board: board.clone(), solution: vec![], current_penalty: 0};

    advance_solution_vector_or_test_solution(
        &pwr,
        &mut state,
        &mut min_penalty,
        &mut best_solution,
    );

    println!("Minimal penalty: {:?}", &min_penalty);
    println!("Board:");
    println!("{:?}", board);
    println!("Pieces:");
    for (pc, optpos) in best_solution.unwrap() {
        if let Some(pos) = optpos {
            println!("{}", render(pc, pos));
        }
    }
}
