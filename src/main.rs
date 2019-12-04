use std::env;
use std::fs;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day = env::args().nth(1).and_then(|d| d.parse::<u8>().ok());

    let t0 = Instant::now();

    run_day(day, 1, || {
        let contents = fs::read_to_string("resources/day1.txt").unwrap();
        (day1::first(&contents), day1::second(&contents))
    });

    run_day(day, 2, || {
        let contents = fs::read_to_string("resources/day2.txt").unwrap();
        (day2::first(&contents), day2::second(&contents))
    });

    run_day(day, 3, || {
        let contents = fs::read_to_string("resources/day3.txt").unwrap();
        (day3::first(&contents), day3::second(&contents))
    });

    run_day(day, 4, || {
        let (low, high) = (124075, 580769);
        (day4::first_async(low, high), day4::second_async(low, high))
    });

    let elapsed = t0.elapsed();
    println!("Time taken: {:?}", elapsed);
}

fn run_day<R1: std::fmt::Debug, R2: std::fmt::Debug, F: FnOnce() -> (R1, R2)>(
    day: Option<u8>,
    i: u8,
    func: F,
) {
    if day.unwrap_or(i) == i {
        let t0 = Instant::now();
        let (r1, r2) = func();
        println!("Day{}a: {:?}", i, r1);
        println!("Day{}a: {:?}", i, r2);
        println!("{:?}", t0.elapsed());
    }
}
