use crate::infra::Problem;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day24;

impl Problem<String, String, u32, usize> for Day24 {
    fn day() -> u8 {
        24
    }
    fn first(contents: String) -> u32 {
        let mut b = parse_board(&contents);
        let mut seen = HashSet::new();

        loop {
            let br = (b & 0b01111_01111_01111_01111_01111 as u32) << 1;
            let bl = (b & 0b11110_11110_11110_11110_11110 as u32) >> 1;
            let bd = (b & 0b00000_11111_11111_11111_11111 as u32) << 5;
            let bu = (b & 0b11111_11111_11111_11111_00000 as u32) >> 5;
            b = br & !bl & !bd & !bu
                | !br & bl & !bd & !bu
                | !br & !bl & bd & !bu
                | !br & !bl & !bd & bu
                | !b & br & bl & !bd & !bu
                | !b & br & !bl & bd & !bu
                | !b & br & !bl & !bd & bu
                | !b & !br & bl & bd & !bu
                | !b & !br & bl & !bd & bu
                | !b & !br & !bl & bd & bu;
            if seen.contains(&b) {
                return b;
            }
            seen.insert(b);
        }
    }

    fn second(contents: String) -> usize {
        let mut boards = VecDeque::new();
        boards.push_back(0);
        boards.push_back(0);
        boards.push_back(parse_board(&contents));
        boards.push_back(0);
        boards.push_back(0);

        for _ in 0..200 {
            let mut res = VecDeque::new();
            for (inner, board, outer) in boards.into_iter().tuple_windows() {
                res.push_back(b_evolve2(inner, board, outer));
            }
            while res[0] != 0 || res[1] != 0 {
                res.push_front(0);
            }
            while res[res.len() - 1] != 0 || res[res.len() - 2] != 0 {
                res.push_back(0);
            }
            boards = res;
        }

        boards.into_iter().map(|b| b.count_ones()).sum::<u32>() as usize
    }
}

fn b_evolve2(inner: u32, b: u32, outer: u32) -> u32 {
    let mu = (inner & 0b11111_00000_00000_00000_00000 | (b & 0b00100_01010_00000_00000_00000) >> 3)
        .count_ones();
    let ml = (inner & 0b10000_10000_10000_10000_10000 | (b & 0b00000_01000_10000_01000_00000) >> 1)
        .count_ones();
    let mr = (inner & 0b00001_00001_00001_00001_00001 | (b & 0b00000_00010_00001_00010_00000) << 1)
        .count_ones();
    let md = (inner & 0b00000_00000_00000_00000_11111 | (b & 0b00000_00000_00000_01010_00100) << 3)
        .count_ones();

    let br = ((b & 0b01111_01111_01111_01111_01111 as u32) << 1)
        | (0b00001_00001_00001_00001_00001 * ((outer >> 11) & 1));
    let bl = ((b & 0b11110_11110_11110_11110_11110 as u32) >> 1)
        | (0b10000_10000_10000_10000_10000 * ((outer >> 13) & 1));
    let bd = ((b & 0b00000_11111_11111_11111_11111 as u32) << 5)
        | (0b00000_00000_00000_00000_11111 * ((outer >> 7) & 1));
    let bu = ((b & 0b11111_11111_11111_11111_00000 as u32) >> 5)
        | (0b11111_00000_00000_00000_00000 * ((outer >> 17) & 1));
    0b11111_11011_10001_11011_11111
        & (br & !bl & !bd & !bu
            | !br & bl & !bd & !bu
            | !br & !bl & bd & !bu
            | !br & !bl & !bd & bu
            | !b & br & bl & !bd & !bu
            | !b & br & !bl & bd & !bu
            | !b & br & !bl & !bd & bu
            | !b & !br & bl & bd & !bu
            | !b & !br & bl & !bd & bu
            | !b & !br & !bl & bd & bu)
        | ((mu == 1 || ((b >> 17) & 1) == 0 && mu == 2) as u32) << 17
        | ((ml == 1 || ((b >> 13) & 1) == 0 && ml == 2) as u32) << 13
        | ((mr == 1 || ((b >> 11) & 1) == 0 && mr == 2) as u32) << 11
        | ((md == 1 || ((b >> 7) & 1) == 0 && md == 2) as u32) << 7
}

fn parse_board(contents: &str) -> u32 {
    contents
        .lines()
        .flat_map(|line| line.chars())
        .rev()
        .fold(0, |x, c| 2 * x + (c == '#') as u32)
}
