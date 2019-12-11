extern crate clipboard_win;
extern crate itertools;
extern crate num;
extern crate petgraph;
extern crate rayon;

use crate::infra::{run_day, FromClipboard, FromFile, Literal};
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
mod infra;
mod utils;

fn main() {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    let t0 = Instant::now();

    run_day(
        day,
        day01::Day1,
        (FromFile("day1.txt"), 3325342),
        (FromFile("day1.txt"), 4985158),
    );

    run_day(
        day,
        day02::Day2,
        (FromFile("day2.txt"), 6568671),
        (FromFile("day2.txt"), 3951),
    );

    run_day(
        day,
        day03::Day3,
        (FromFile("day3.txt"), 316),
        (FromFile("day3.txt"), 16368),
    );

    run_day(
        day,
        day04::Day4,
        (Literal((124075, 580769)), 2150),
        (Literal((124075, 580769)), 1462),
    );

    run_day(
        day,
        day05::Day5,
        (FromFile("day5.txt"), 7692125),
        (FromFile("day5.txt"), 14340395),
    );

    run_day(
        day,
        day06::Day6,
        (FromFile("day6.txt"), 268504),
        (FromFile("day6.txt"), 409),
    );

    run_day(
        day,
        day07::Day7,
        (FromFile("day7.txt"), 118936),
        (FromFile("day7.txt"), 57660948),
    );

    run_day(
        day,
        day08::Day8,
        (FromFile("day8.txt"), 2806),
        (FromFile("day8.txt"), "ZBJAB".to_string()),
    );

    run_day(
        day,
        day09::Day9,
        (FromFile("day9.txt"), 2494485073),
        (FromFile("day9.txt"), 44997),
    );

    run_day(
        day,
        day10::Day10,
        (FromFile("day10.txt"), 278),
        (FromFile("day10.txt"), 1417),
    );

    run_day(
        day,
        day11::Day11,
        (FromFile("day11.txt"), 2594),
        (FromFile("day11.txt"), "AKERJFHK".to_string()),
    );

    let elapsed = t0.elapsed();
    println!("Time taken: {:?}", elapsed);
}
