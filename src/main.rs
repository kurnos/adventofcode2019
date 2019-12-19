extern crate clipboard_win;
#[macro_use]
extern crate itertools;
extern crate num;
extern crate petgraph;
extern crate rayon;

use crate::infra::{run_day, FromFile, Literal};
use std::env;
use std::time::Instant;

mod computer;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod infra;
mod utils;

#[allow(clippy::unreadable_literal)]
fn main() {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());
    let times = env::args()
        .nth(2)
        .and_then(|d| d.parse::<u32>().ok())
        .unwrap_or(1);

    let t0 = Instant::now();

    run_day(
        day,
        times,
        day01::Day1,
        (FromFile("day1.txt"), 3325342),
        (FromFile("day1.txt"), 4985158),
    );

    run_day(
        day,
        times,
        day02::Day2,
        (FromFile("day2.txt"), 6568671),
        (FromFile("day2.txt"), 3951),
    );

    run_day(
        day,
        times,
        day03::Day3,
        (FromFile("day3.txt"), 316),
        (FromFile("day3.txt"), 16368),
    );

    run_day(
        day,
        times,
        day04::Day4,
        (Literal((124075, 580769)), 2150),
        (Literal((124075, 580769)), 1462),
    );

    run_day(
        day,
        times,
        day05::Day5,
        (FromFile("day5.txt"), 7692125),
        (FromFile("day5.txt"), 14340395),
    );

    run_day(
        day,
        times,
        day06::Day6,
        (FromFile("day6.txt"), 268504),
        (FromFile("day6.txt"), 409),
    );

    run_day(
        day,
        times,
        day07::Day7,
        (FromFile("day7.txt"), 118936),
        (FromFile("day7.txt"), 57660948),
    );

    run_day(
        day,
        times,
        day08::Day8,
        (FromFile("day8.txt"), 2806),
        (FromFile("day8.txt"), "ZBJAB".to_string()),
    );

    run_day(
        day,
        times,
        day09::Day9,
        (FromFile("day9.txt"), 2494485073),
        (FromFile("day9.txt"), 44997),
    );

    run_day(
        day,
        times,
        day10::Day10,
        (FromFile("day10.txt"), 278),
        (FromFile("day10.txt"), 1417),
    );

    run_day(
        day,
        times,
        day11::Day11,
        (FromFile("day11.txt"), 2594),
        (FromFile("day11.txt"), "AKERJFHK".to_string()),
    );

    run_day(
        day,
        times,
        day12::Day12,
        (FromFile("day12.txt"), 12082),
        (FromFile("day12.txt"), 295693702908636),
    );

    run_day(
        day,
        times,
        day13::Day13,
        (FromFile("day13.txt"), 291),
        (FromFile("day13.txt"), 14204),
    );

    run_day(
        day,
        times,
        day14::Day14,
        (FromFile("day14.txt"), 278404),
        (FromFile("day14.txt"), 4436981),
    );

    run_day(
        day,
        times,
        day15::Day15,
        (FromFile("day15.txt"), 248),
        (FromFile("day15.txt"), 382),
    );

    run_day(
        day,
        times,
        day16::Day16,
        (FromFile("day16.txt"), 28430146),
        (FromFile("day16.txt"), 12064286),
    );

    run_day(
        day,
        times,
        day17::Day17,
        (FromFile("day17.txt"), 5940),
        (FromFile("day17.txt"), 923795),
    );

    run_day(
        day,
        times,
        day18::Day18,
        (FromFile("day18.txt"), 4770),
        (FromFile("day18.txt"), 1578),
    );

    run_day(
        day,
        times,
        day19::Day19,
        (FromFile("day19.txt"), 141),
        (FromFile("day19.txt"), 15641348),
    );

    let elapsed = t0.elapsed();
    println!("Time taken: {:?}", elapsed);
}
