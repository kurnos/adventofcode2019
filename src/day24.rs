use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use bit_set::BitSet;
use itertools::Either::{Left, Right};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

type Pos = Point2d<i8>;
struct Board(BitSet);

impl Board {
    fn new() -> Board {
        Board(BitSet::with_capacity(25))
    }

    fn is_bug(&self, p: Pos) -> bool {
        p.x >= 0 && p.y >= 0 && self.0.contains((p.x * 6 + p.y) as usize)
    }

    fn add_bug(&mut self, p: Pos) {
        self.0.insert((p.x * 6 + p.y) as usize);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn count_bugs(&self) -> usize {
        self.0.len()
    }
}

pub struct Day24;

impl Problem<String, String, u32, usize> for Day24 {
    fn day() -> u8 {
        24
    }
    fn first(contents: String) -> u32 {
        let mut b = parse_board(&contents);
        let mut seen = HashSet::new();

        loop {
            b = evolve(b);
            let s = score(&b);
            if seen.contains(&s) {
                println!("{}", s);
                return s;
            }
            seen.insert(s);
        }
    }
    fn second(contents: String) -> usize {
        let mut boards = HashMap::<i32, Board>::new();
        boards.insert(0, parse_board(&contents));
        (0..200)
            .fold(boards, |b, _| evolve2(b))
            .values()
            .map(|b| b.count_bugs())
            .sum()
    }
}

fn evolve2(boards: HashMap<i32, Board>) -> HashMap<i32, Board> {
    let min_i = boards.keys().cloned().min().unwrap();
    let max_i = boards.keys().cloned().max().unwrap();

    (min_i - 1..=max_i + 1)
        .into_par_iter()
        .map(|i| {
            let mut new_board = Board::new();
            for (x, y) in iproduct!(0..5, 0..5) {
                if (x, y) == (2, 2) {
                    continue;
                }
                let p = Pos::new(x, y);

                let live = neighbours_rec(i, p)
                    .filter(|(i, np)| boards.get(&i).map(|b| b.is_bug(*np)).unwrap_or_default())
                    .count();

                if boards.get(&i).map(|b| b.is_bug(p)).unwrap_or_default() {
                    if live == 1 {
                        new_board.add_bug(p);
                    }
                } else if live == 1 || live == 2 {
                    new_board.add_bug(p);
                }
            }
            (i, new_board)
        })
        .filter(|(_, b)| !b.is_empty())
        .collect()
}

fn neighbours_rec(i: i32, t: Pos) -> impl Iterator<Item = (i32, Pos)> {
    [Dir::East, Dir::West, Dir::South, Dir::North]
        .iter()
        .flat_map(move |&d| {
            let p = t + d;
            if p.x == -1 {
                Left(std::iter::once((i - 1, Pos::new(1, 2))))
            } else if p.x == 5 {
                Left(std::iter::once((i - 1, Pos::new(3, 2))))
            } else if p.y == -1 {
                Left(std::iter::once((i - 1, Pos::new(2, 1))))
            } else if p.y == 5 {
                Left(std::iter::once((i - 1, Pos::new(2, 3))))
            } else if (p.x, p.y) == (2, 2) {
                Right((0..5).map(move |j| match d {
                    Dir::South => (i + 1, Pos::new(j, 0)),
                    Dir::North => (i + 1, Pos::new(j, 4)),
                    Dir::East => (i + 1, Pos::new(0, j)),
                    Dir::West => (i + 1, Pos::new(4, j)),
                }))
            } else {
                Left(std::iter::once((i, p)))
            }
        })
}

fn score(board: &Board) -> u32 {
    let mut d = 1u32;
    let mut s = 0;
    for y in 0..5 {
        for x in 0..5 {
            if board.is_bug(Pos::new(x, y)) {
                s += d;
            }
            d *= 2;
        }
    }
    s
}

fn parse_board(c: &str) -> Board {
    let mut res = Board::new();
    for (y, line) in c.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.trim().split("").skip(1).enumerate() {
            if c == "#" {
                res.add_bug(Pos::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    res
}

fn evolve(board: Board) -> Board {
    let mut res = Board::new();
    for (x, y) in iproduct!(0..5, 0..5) {
        let p = Pos::new(x, y);
        let live = [Dir::North, Dir::East, Dir::West, Dir::South]
            .iter()
            .filter(|&&d| board.is_bug(p + d))
            .count();
        if board.is_bug(p) {
            if live == 1 {
                res.add_bug(p);
            }
        } else if live == 1 || live == 2 {
            res.add_bug(p);
        }
    }
    res
}
