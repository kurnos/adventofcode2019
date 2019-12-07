use std::env;
use std::fs;
use std::time::Instant;
mod computer;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    let t0 = Instant::now();

    run_day(
        day,
        1,
        || {
            let contents = fs::read_to_string("resources/day1.txt").unwrap();
            (day1::first(&contents), day1::second(&contents))
        },
        3325342,
        4985158,
    );

    run_day(
        day,
        2,
        || {
            let contents = fs::read_to_string("resources/day2.txt").unwrap();
            (day2::first(&contents), day2::second(&contents))
        },
        6568671,
        3951,
    );

    run_day(
        day,
        3,
        || {
            let contents = fs::read_to_string("resources/day3.txt").unwrap();
            (day3::first(&contents), day3::second(&contents))
        },
        316,
        16368,
    );

    run_day(
        day,
        4,
        || {
            let (low, high) = (124075, 580769);
            (day4::first(low, high), day4::second(low, high))
        },
        2150,
        1462,
    );

    run_day(
        day,
        5,
        || {
            let contents = fs::read_to_string("resources/day5.txt").unwrap();
            (day5::first(&contents), day5::second(&contents))
        },
        7692125,
        14340395,
    );

    run_day(
        day,
        6,
        || {
            let contents = fs::read_to_string("resources/day6.txt").unwrap();
            (day6::first(&contents), day6::second(&contents))
        },
        268504,
        409,
    );

    run_day(
        day,
        7,
        || {
            let contents = fs::read_to_string("resources/day7.txt").unwrap();
            (day7::first(&contents), day7::second(&contents))
        },
        118936,
        57660948,
    );

    let elapsed = t0.elapsed();
    println!("Time taken: {:?}", elapsed);
}

fn run_day<
    R1: std::fmt::Debug + PartialEq,
    R2: std::fmt::Debug + PartialEq,
    F: FnOnce() -> (R1, R2),
>(
    day: Option<u8>,
    i: u8,
    func: F,
    a: R1,
    b: R2,
) {
    if day.unwrap_or(i) == i {
        let t0 = Instant::now();
        let (r1, r2) = func();
        assert_eq!(r1, a);
        assert_eq!(r2, b);
        println!("Day{}a: {:?}", i, r1);
        println!("Day{}a: {:?}", i, r2);
        println!("{:?}", t0.elapsed());
    }
}
